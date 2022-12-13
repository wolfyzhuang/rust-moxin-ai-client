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
