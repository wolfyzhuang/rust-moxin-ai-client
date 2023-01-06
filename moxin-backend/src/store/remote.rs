use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Seek, Write};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use tokio::time::timeout;
use std::time::Duration;
use moxin_protocol::data::Model;
use moxin_protocol::protocol::FileDownloadResponse;

use crate::backend_impls::DownloadControlCommand;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct RemoteFile {
    pub name: String,
    pub size: String,
    pub quantization: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Author {
    pub name: S