
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