use rusqlite::Row;
use std::{sync::Arc, vec};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum PendingDownloadsStatus {
    #[default]
    Downloading,
    Paused,
    Error,
}

impl PendingDownloadsStatus {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Downloading => "downloading",
            Self::Paused => "paused",
            Self::Error => "error",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "downloading" => Self::Downloading,
            "paused" => Self::Paused,
            "error" => Self::Error,
            _ => Self::Downloading,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PendingDownloads {
    pub file_id: Arc<String>,
    pub progress: f64,
    pub status: PendingDownloadsStatus,
}

// TODO I'm not 100% convinced that this is the best way to handle this
// I will attempt to merge PendingDownloads and