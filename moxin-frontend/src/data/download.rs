
use makepad_widgets::SignalToUI;
use moxin_backend::Backend;
use moxin_protocol::data::*;
use moxin_protocol::protocol::{Command, FileDownloadResponse};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub enum DownloadFileAction {
    Progress(f64),
    Error,
    StreamingDone,
}

#[derive(Clone, Copy, Debug)]
pub enum DownloadState {
    Downloading(f64),
    Errored(f64),
    Paused(f64),
    Completed,
}

#[derive(Debug)]
pub struct Download {
    pub file: File,
    pub model: Model,
    pub sender: Sender<DownloadFileAction>,
    pub receiver: Receiver<DownloadFileAction>,
    pub state: DownloadState,
    pub notification_pending: bool,
}

impl Download {
    pub fn new(file: File, model: Model, progress: f64, backend: &Backend) -> Self {
        let (tx, rx) = channel();
        let mut download = Self {
            file: file,
            model: model,
            sender: tx,
            receiver: rx,
            state: DownloadState::Downloading(progress),
            notification_pending: false,
        };

        download.start(backend);
        download
    }

    pub fn start(&mut self, backend: &Backend) {
        let (tx, rx) = channel();

        let store_download_tx = self.sender.clone();
        let cmd = Command::DownloadFile(self.file.id.clone(), tx);
        backend.command_sender.send(cmd).unwrap();

        thread::spawn(move || loop {
            let mut is_done = false;
            if let Ok(response) = rx.recv() {
                match response {
                    Ok(response) => match response {
                        FileDownloadResponse::Completed(_completed) => {
                            is_done = true;
                            store_download_tx
                                .send(DownloadFileAction::StreamingDone)
                                .unwrap();
                        }
                        FileDownloadResponse::Progress(_file, value) => store_download_tx
                            .send(DownloadFileAction::Progress(value as f64))
                            .unwrap(),
                    },
                    Err(err) => {
                        store_download_tx
                            .send(DownloadFileAction::Error)
                            .unwrap();

                        eprintln!("Error downloading file: {:?}", err)
                    },
                }
            } else {
                break
            }

            SignalToUI::set_ui_signal();
            if is_done {
                break
            }
        });
    }

    pub fn process_download_progress(&mut self) {
        for msg in self.receiver.try_iter() {
            match msg {
                DownloadFileAction::StreamingDone => {
                    self.state = DownloadState::Completed;
                    self.notification_pending = true;
                }
                DownloadFileAction::Progress(value) => {
                    self.state = DownloadState::Downloading(value)
                }
                DownloadFileAction::Error => {
                    let current_progress = self.get_progress();
                    self.state = DownloadState::Errored(current_progress);
                    self.notification_pending = true;
                }
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state, DownloadState::Completed)
    }

    pub fn is_errored(&self) -> bool {