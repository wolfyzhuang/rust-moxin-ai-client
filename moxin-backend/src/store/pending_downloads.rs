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
    pub fn to_st