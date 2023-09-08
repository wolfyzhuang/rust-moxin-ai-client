
use super::chat::ChatID;
use super::download::DownloadState;
use super::filesystem::{moxin_home_dir, setup_model_downloads_folder};
use super::preferences::Preferences;
use super::{chat::Chat, download::Download, search::Search};
use anyhow::{Context, Result};
use chrono::Utc;
use makepad_widgets::{DefaultNone, SignalToUI};
use moxin_backend::Backend;
use moxin_protocol::data::{
    DownloadedFile, File, FileID, Model, PendingDownload, PendingDownloadsStatus,
};
use moxin_protocol::protocol::{Command, LoadModelOptions, LoadModelResponse};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::channel;

pub const DEFAULT_MAX_DOWNLOAD_THREADS: usize = 3;

#[derive(Clone, DefaultNone, Debug)]
pub enum StoreAction {
    Search(String),
    ResetSearch,
    Sort(SortCriteria),
    None,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum SortCriteria {
    #[default]
    MostDownloads,
    LeastDownloads,
    MostLikes,
    LeastLikes,
}

#[derive(Clone, Debug)]
pub struct DownloadInfo {
    pub file: File,
    pub model: Model,
    pub state: DownloadState,
}

impl DownloadInfo {
    pub fn get_progress(&self) -> f64 {
        match self.state {
            DownloadState::Downloading(progress) => progress,
            DownloadState::Errored(progress) => progress,
            DownloadState::Paused(progress) => progress,
            DownloadState::Completed => 100.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModelWithPendingDownloads {
    pub model: Model,
    pub pending_downloads: Vec<PendingDownload>,
    pub current_file_id: Option<FileID>,
}

pub enum DownloadPendingNotification {
    DownloadedFile(File),
    DownloadErrored(File),
}

#[derive(Default)]
pub struct Store {
    /// This is the backend representation, including the sender and receiver ends of the channels to
    /// communicate with the backend thread.
    pub backend: Backend,

    /// Local cache of search results and downloaded files
    pub models: Vec<Model>,
    pub downloaded_files: Vec<DownloadedFile>,
    pub pending_downloads: Vec<PendingDownload>,

    pub search: Search,
    pub sorted_by: SortCriteria,

    /// Locally saved chats
    pub saved_chats: Vec<RefCell<Chat>>,
    pub current_chat_id: Option<ChatID>,
    pub current_downloads: HashMap<FileID, Download>,

    pub preferences: Preferences,
    pub downloaded_files_dir: String,
}

impl Store {
    pub fn new() -> Self {
        let downloaded_files_dir = setup_model_downloads_folder();
        let moxin_home_dir = moxin_home_dir().to_string_lossy().to_string();

        let backend = Backend::new(
            moxin_home_dir,
            downloaded_files_dir.clone(),
            DEFAULT_MAX_DOWNLOAD_THREADS,
        );
        let mut store = Self {
            backend,
            // Initialize the local cache with empty values
            models: vec![],

            // TODO we should unify those two into a single struct
            downloaded_files: vec![],
            pending_downloads: vec![],

            search: Search::new(),
            sorted_by: SortCriteria::MostDownloads,
            saved_chats: vec![],
            current_chat_id: None,
            current_downloads: HashMap::new(),

            preferences: Preferences::load(),
            downloaded_files_dir,
        };
        store.load_downloaded_files();
        store.load_pending_downloads();

        store.load_featured_models();

        store.sort_models(SortCriteria::MostDownloads);
        store
    }

    pub fn get_current_chat(&self) -> Option<&RefCell<Chat>> {
        if let Some(current_chat_id) = self.current_chat_id {
            self.saved_chats
                .iter()
                .find(|c| c.borrow().id == current_chat_id)
        } else {
            None
        }
    }

    // Commands to the backend

    pub fn load_featured_models(&mut self) {
        self.search.load_featured_models(&self.backend);
    }

    pub fn load_search_results(&mut self, query: String) {
        self.search.run_or_enqueue(query.clone(), &self.backend);
    }

    pub fn load_downloaded_files(&mut self) {
        let (tx, rx) = channel();
        self.backend
            .command_sender
            .send(Command::GetDownloadedFiles(tx))
            .unwrap();

        if let Ok(response) = rx.recv() {
            match response {
                Ok(files) => {
                    self.downloaded_files = files;
                }
                Err(err) => eprintln!("Error fetching downloaded files: {:?}", err),
            }
        };
    }

    pub fn load_pending_downloads(&mut self) {
        let (tx, rx) = channel();
        self.backend
            .command_sender
            .send(Command::GetCurrentDownloads(tx))
            .unwrap();

        if let Ok(response) = rx.recv() {
            match response {
                Ok(files) => {
                    self.pending_downloads = files;
                }
                Err(err) => eprintln!("Error fetching pending downloads: {:?}", err),
            }
        };
    }

    fn get_model_and_file_download(&self, file_id: &str) -> (Model, File) {
        if let Some(result) = self.get_model_and_file_for_pending_download(file_id) {
            result
        } else {
            self.get_model_and_file_from_search_results(file_id)
                .unwrap()
        }
    }

    fn get_model_and_file_from_search_results(&self, file_id: &str) -> Option<(Model, File)> {
        self.models.iter().find_map(|m| {
            m.files
                .iter()
                .find(|f| f.id == file_id)
                .map(|f| (m.clone(), f.clone()))
        })
    }

    fn get_model_and_file_for_pending_download(&self, file_id: &str) -> Option<(Model, File)> {
        self.pending_downloads.iter().find_map(|d| {
            if d.file.id == file_id {
                Some((d.model.clone(), d.file.clone()))
            } else {
                None
            }
        })
    }

    pub fn download_file(&mut self, file_id: FileID) {
        let (model, file) = self.get_model_and_file_download(&file_id);
        let mut current_progress = 0.0;

        if let Some(pending) = self
            .pending_downloads
            .iter_mut()
            .find(|d| d.file.id == file_id)
        {
            current_progress = pending.progress;
            pending.status = PendingDownloadsStatus::Downloading;
        } else {
            let pending_download = PendingDownload {
                file: file.clone(),
                model: model.clone(),
                progress: 0.0,
                status: PendingDownloadsStatus::Downloading,
            };
            self.pending_downloads.push(pending_download);
        }

        self.current_downloads.insert(
            file_id.clone(),
            Download::new(file, model, current_progress, &self.backend),
        );
    }

    pub fn pause_download_file(&mut self, file_id: FileID) {
        let (tx, rx) = channel();
        self.backend
            .command_sender
            .send(Command::PauseDownload(file_id.clone(), tx))
            .unwrap();

        if let Ok(response) = rx.recv() {
            match response {
                Ok(()) => {
                    self.current_downloads.remove(&file_id);
                    self.load_pending_downloads();
                }
                Err(err) => eprintln!("Error pausing download: {:?}", err),
            }
        };
    }

    pub fn cancel_download_file(&mut self, file_id: FileID) {
        let (tx, rx) = channel();
        self.backend
            .command_sender
            .send(Command::CancelDownload(file_id.clone(), tx))
            .unwrap();

        if let Ok(response) = rx.recv() {
            match response {
                Ok(()) => {
                    self.current_downloads.remove(&file_id);
                    self.pending_downloads.retain(|d| d.file.id != file_id);
                    self.load_pending_downloads();
                }
                Err(err) => eprintln!("Error cancelling download: {:?}", err),
            }
        };
    }

    pub fn eject_model(&mut self) -> Result<()> {
        let (tx, rx) = channel();
        self.backend
            .command_sender
            .send(Command::EjectModel(tx))
            .context("Failed to send eject model command")?;

        let _ = rx
            .recv()
            .context("Failed to receive eject model response")?
            .context("Eject model operation failed");

        self.current_chat_id = None;
        Ok(())
    }

    pub fn delete_file(&mut self, file_id: FileID) -> Result<()> {
        let (tx, rx) = channel();
        self.backend
            .command_sender
            .send(Command::DeleteFile(file_id.clone(), tx))
            .context("Failed to send delete file command")?;

        rx.recv()
            .context("Failed to receive delete file response")?
            .context("Delete file operation failed")?;

        self.set_file_downloaded_state(&file_id, false);
        self.load_downloaded_files();
        self.load_pending_downloads();
        SignalToUI::set_ui_signal();
        Ok(())
    }

    pub fn load_model(&mut self, file: &File) {
        let (tx, rx) = channel();
        let cmd = Command::LoadModel(
            file.id.clone(),
            LoadModelOptions {
                prompt_template: None,
                gpu_layers: moxin_protocol::protocol::GPULayers::Max,
                use_mlock: false,
                n_batch: 512,
                n_ctx: 512,
                rope_freq_scale: 0.0,
                rope_freq_base: 0.0,
                context_overflow_policy:
                    moxin_protocol::protocol::ContextOverflowPolicy::StopAtLimit,
            },
            tx,
        );

        self.backend.command_sender.send(cmd).unwrap();

        if let Ok(response) = rx.recv() {
            match response {
                Ok(response) => {
                    let LoadModelResponse::Completed(_) = response else {
                        eprintln!("Error loading model");
                        return;
                    };
                    // TODO: Creating a new chat, maybe put in a method and save on disk or smth.
                    let new_chat = RefCell::new(Chat::new(file.name.clone(), file.id.clone()));
                    self.current_chat_id = Some(new_chat.borrow().id);
                    self.saved_chats.push(new_chat);

                    self.preferences.set_current_chat_model(file.id.clone());
                }
                Err(err) => eprintln!("Error loading model: {:?}", err),
            }
        };
    }

    pub fn send_chat_message(&mut self, prompt: String) {
        if let Some(chat) = self.get_current_chat() {
            chat.borrow_mut()
                .send_message_to_model(prompt, &self.backend);
        }
    }

    pub fn cancel_chat_streaming(&mut self) {
        if let Some(chat) = self.get_current_chat() {
            chat.borrow_mut().cancel_streaming(&self.backend);
        }
    }

    pub fn delete_chat_message(&mut self, message_id: usize) {
        if let Some(chat) = self.get_current_chat() {
            chat.borrow_mut().delete_message(message_id);
        }
    }

    pub fn edit_chat_message(
        &mut self,
        message_id: usize,
        updated_message: String,
        regenerate: bool,
    ) {
        if let Some(chat) = &mut self.get_current_chat() {
            let mut chat = chat.borrow_mut();
            if regenerate {
                if chat.is_streaming {
                    chat.cancel_streaming(&self.backend);
                }

                chat.remove_messages_from(message_id);
                chat.send_message_to_model(updated_message, &self.backend);