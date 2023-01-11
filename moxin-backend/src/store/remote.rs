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
    pub name: String,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct RemoteModel {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub size: String,
    pub requires: String,
    pub architecture: String,
    pub released_at: DateTime<Utc>,
    pub files: Vec<RemoteFile>,
    pub prompt_template: String,
    pub reverse_prompt: String,
    pub author: Author,
    pub like_count: u32,
    pub download_count: u32,
    #[serde(default)]
    pub metrics: Option<HashMap<String, f32>>,
}

impl RemoteModel {
    pub fn search(search_text: &str, limit: usize, offset: usize) -> reqwest::Result<Vec<Self>> {
        let url = format!("https://code.flows.network/webhook/DsbnEK45sK3NUzFUyZ9C/models?status=published&trace_status=tracing,renamed&model_type=instruct,chat&order=most_likes&offset={offset}&limit={limit}&search={search_text}");
        let response = reqwest::blocking::get(&url)?;
        response.json()
    }

    pub fn get_featured_model(limit: usize, offset: usize) -> reqwest::Result<Vec<Self>> {
  