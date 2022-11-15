
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

use chrono::Utc;
use moxin_protocol::{
    data::{DownloadedFile, FileID, Model, PendingDownload},
    open_ai::{ChatRequestData, ChatResponse},
    protocol::{
        Command, FileDownloadResponse, LoadModelOptions, LoadModelResponse, LocalServerConfig,
        LocalServerResponse,
    },
};
use wasmedge_sdk::Module;

use crate::store::{self, ModelFileDownloader, RemoteModel};

mod chat_ui {

    #[derive(Debug)]
    pub enum TokenError {
        EndOfSequence = 1,
        ContextFull,
        PromptTooLong,
        TooLarge,
        InvalidEncoding,
        Other,
    }

    impl Into<StopReason> for TokenError {
        fn into(self) -> StopReason {
            match self {
                TokenError::EndOfSequence => StopReason::Stop,
                TokenError::ContextFull => StopReason::Length,
                TokenError::PromptTooLong => StopReason::Length,
                TokenError::TooLarge => StopReason::Length,
                TokenError::InvalidEncoding => StopReason::Stop,
                TokenError::Other => StopReason::Stop,
            }
        }
    }

    use std::{
        collections::HashMap,
        io::Read,
        path::Path,
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::{Receiver, Sender},
            Arc,
        },
        thread::JoinHandle,
    };

    use moxin_protocol::{
        open_ai::{
            ChatRequestData, ChatResponse, ChatResponseChunkData, ChatResponseData, ChoiceData,
            ChunkChoiceData, MessageData, Role, StopReason, UsageData,
        },
        protocol::{LoadModelOptions, LoadModelResponse, LoadedModelInfo},
    };
    use wasmedge_sdk::{
        error::{CoreError, CoreExecutionError},
        wasi::WasiModule,
        CallingFrame, ImportObject, Instance, Module, Store, Vm, WasmValue,
    };

    use crate::store::download_files::DownloadedFile;

    #[derive(Debug)]
    pub struct ChatBotUi {
        pub current_req: std::io::Cursor<Vec<u8>>,
        pub request_rx: Receiver<(ChatRequestData, Sender<anyhow::Result<ChatResponse>>)>,
        request_id: uuid::Uuid,
        chat_completion_message: Option<Vec<u8>>,
        pub token_tx: Option<Sender<anyhow::Result<ChatResponse>>>,
        running_controller: Arc<AtomicBool>,
        pub load_model_state: Option<(
            DownloadedFile,
            LoadModelOptions,
            Sender<anyhow::Result<LoadModelResponse>>,
        )>,
    }

    impl ChatBotUi {
        pub fn new(
            request_rx: Receiver<(ChatRequestData, Sender<anyhow::Result<ChatResponse>>)>,
            running_controller: Arc<AtomicBool>,
            file: DownloadedFile,
            load_model: LoadModelOptions,
            tx: Sender<anyhow::Result<LoadModelResponse>>,
        ) -> Self {
            Self {
                request_rx,
                request_id: uuid::Uuid::new_v4(),
                token_tx: None,
                running_controller,
                current_req: std::io::Cursor::new(vec![]),
                load_model_state: Some((file, load_model, tx)),
                chat_completion_message: None,
            }
        }

        fn init_request(&mut self) -> Result<(), ()> {
            if let Ok((req, tx)) = self.request_rx.recv() {
                // Init current_req
                if !req.stream.unwrap_or_default() {
                    self.chat_completion_message = Some(Vec::with_capacity(
                        (req.max_tokens.unwrap_or(512) * 8) as usize,
                    ))
                }
                *self.current_req.get_mut() = serde_json::to_vec(&req).unwrap();
                self.current_req.set_position(0);
                self.request_id = uuid::Uuid::new_v4();
                self.token_tx = Some(tx);
                self.running_controller.store(true, Ordering::Release);
                Ok(())
            } else {
                Err(())
            }
        }

        pub fn read_data(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let n = self.current_req.read(buf)?;
            if n == 0 {
                self.current_req.get_mut().clear();
                self.current_req.set_position(0);
            }
            Ok(n)
        }

        fn send_completion_output(
            token_tx: &mut Sender<anyhow::Result<ChatResponse>>,
            id: String,
            stop_reason: StopReason,
            chat_completion_message: &mut Option<Vec<u8>>,
        ) -> bool {
            if let Some(chat_completion_message) = chat_completion_message.take() {
                let _ = token_tx.send(Ok(ChatResponse::ChatFinalResponseData(ChatResponseData {
                    id,
                    choices: vec![ChoiceData {
                        finish_reason: stop_reason,
                        index: 0,
                        message: MessageData {