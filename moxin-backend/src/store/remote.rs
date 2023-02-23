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
        let url = format!("https://code.flows.network/webhook/DsbnEK45sK3NUzFUyZ9C/models?status=published&trace_status=tracing,renamed&model_type=instruct,chat&order=most_likes&offset={offset}&limit={limit}&featured=featured");
        let response = reqwest::blocking::get(&url)?;
        response.json()
    }

    pub fn to_model(
        remote_models: &[Self],
        conn: &rusqlite::Connection,
    ) -> rusqlite::Result<Vec<moxin_protocol::data::Model>> {
        let model_ids = remote_models
            .iter()
            .map(|m| m.id.clone())
            .collect::<Vec<_>>();
        let files = super::download_files::DownloadedFile::get_by_models(conn, &model_ids)?;

        fn to_file(
            model_id: &str,
            remote_files: &[RemoteFile],
            save_files: &HashMap<Arc<String>, super::download_files::DownloadedFile>,
        ) -> rusqlite::Result<Vec<moxin_protocol::data::File>> {
            let mut files = vec![];
            for remote_f in remote_files {
                let file_id = format!("{}#{}", model_id, remote_f.name);
                let downloaded_path = save_files.get(&file_id).map(|file| {
                    let file_path = Path::new(&file.download_dir)
                        .join(&file.model_id)
                        .join(&file.name);
                    file_path
                        .to_str()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                });

                let file = moxin_protocol::data::File {
                    id: file_id,
                    name: remote_f.name.clone(),
                    size: remote_f.size.clone(),
                    quantization: remote_f.quantization.clone(),
                    downloaded: downloaded_path.is_some(),
                    downloaded_path,
                    tags: remote_f.tags.clone(),
                    featured: false,
                };

                files.push(file);
            }

            Ok(files)
        }

        let mut models = Vec::with_capacity(remote_models.len());

        for remote_m in remote_models {
            let model = Model {
                id: remote_m.id.clone(),
                name: remote_m.name.clone(),
                summary: remote_m.summary.clone(),
                size: remote_m.size.clone(),
                requires: remote_m.requires.clone(),
                architecture: remote_m.architecture.clone(),
                released_at: remote_m.released_at.clone(),
                files: to_file(&remote_m.id, &remote_m.files, &files)?,
                author: moxin_protocol::data::Author {
                    name: remote_m.author.name.clone(),
                    url: remote_m.author.url.clone(),
                    description: remote_m.author.description.clone(),
                },
                like_count: remote_m.like_count.clone(),
                download_count: remote_m.download_count.clone(),
                metrics: remote_m.metrics.clone().unwrap_or_default(),
            };

            models.push(model);
        }

        Ok(models)
    }
}

async fn get_file_content_length(client: &reqwest::Client, url: &str) -> reqwest::Result<u64> {
    let response = client.head(url).send().await?;

    let content_length = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(0);

    Ok(content_length)
}

pub enum DownloadResult {
    Completed(f64),
    Stopped(f64),
}

async fn download_file<P: AsRef<Path>>(
    client: &reqwest::Client,
    content_length: u64,
    url: &str,
    local_path: P,
    step: f64,
    report_fn: &mut (dyn FnMut(f64) -> anyhow::Result<()> + Send),
) -> anyhow::Result<DownloadResult> {
    use futures_util::stream::StreamExt;

    let path: &Path = local_path.as_ref();
    std::fs::create_dir_all(path.parent().unwrap())?;

    let mut file = File::options().write(true).create(true).open(&local_path)?;

    let file_length = file.metadata()?.len();

    if file_length < content_length {
        file.seek(io::SeekFrom::End(0))?;

        let range = format!("bytes={}-", file_length);
        let resp = client
            .get(url)
            .header("Range", range)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let mut downloaded: u64 = file_length;
        let mut last_progress = 0.0;

        let mut stream = resp.bytes_stream();

        loop {
            match timeout(Duration::from_secs(10), stream.next()).await? {
                Some(chunk) => {
                    let chunk = chunk.map_err(|e| anyhow::anyhow!(e))?;
                    let len = chunk.len();
                    file.write_all(&chunk)?;
                    downloaded += len as u64;

                    let progress = (downloaded as f64 / content_length as f64) * 100.0;
                    if progress > last_progress + step {
                        last_progress = progress;
                        match report_fn(progress) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                },
                None => {
                    // Download is complete
                    break;
                }
            }
        }

        // TODO I don't know how to handle when it is complete but not 100%
        // Maybe we should return Completed without any value?
        Ok(DownloadResult::Completed(
            (downloaded as f64 / content_length as f64) * 100.0,
        ))
    } else {
        Ok(DownloadResult::Completed(100.0))
    }
}

#[derive(Debug, Clone)]
pub struct ModelFileDownloader {
    client: reqwest::Client,
    sql_conn: Arc<Mutex<rusqlite::Connection>>,
    control_tx: tokio::sync::broadcast::Sender<DownloadControlCommand>,
    step: f64,
}

impl ModelFileDownloader {
    pub fn new(
        client: reqwest::Client,
        sql_conn: Arc<Mutex<rusqlite::Connection>>,
        control_tx: tokio::sync::broadcast::Sender<DownloadControlCommand>,
        step: f64,
    ) -> Self {
        Self {
            client,
            sql_conn,
            control_tx,
            step,
        }
    }

    fn get_download_url(&self, file: &super::download_files::DownloadedFile) -> String {
        format!(
            "https://huggingface.co/{}/resolve/main/{}",
            file.model_id,