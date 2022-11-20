
use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use rusqlite::Row;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct DownloadedFile {
    pub id: Arc<String>,
    pub model_id: String,
    pub name: String,
    pub size: String,
    pub quantization: String,
    pub prompt_template: String,
    pub reverse_prompt: String,
    pub downloaded: bool,
    pub file_size: u64,
    pub download_dir: String,
    pub downloaded_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub featured: bool,
    pub sha256: String,
}

impl DownloadedFile {
    pub fn insert_into_db(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        conn.execute(
            "INSERT OR IGNORE INTO download_files (
                id, model_id, name, size, quantization, 
                prompt_template, reverse_prompt, 
                downloaded, file_size, download_dir, downloaded_at, tags, featured, sha256) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![
                self.id,
                self.model_id,
                self.name,
                self.size,
                self.quantization,
                self.prompt_template,
                self.reverse_prompt,
                self.downloaded,
                self.file_size,
                self.download_dir,
                self.downloaded_at.to_rfc3339(),
                serde_json::to_string(&self.tags).unwrap(),
                self.featured,
                self.sha256,
            ],
        )?;

        Ok(())
    }

    pub fn mark_downloads(&mut self) {
        self.downloaded = true;
        self.downloaded_at = Utc::now();
    }

    pub fn update_downloaded(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        debug_assert!(self.downloaded);

        conn.execute(
            "UPDATE download_files
                SET downloaded = ?2,
                    downloaded_at = ?3
                WHERE id = ?1",
            rusqlite::params![self.id, self.downloaded, self.downloaded_at.to_rfc3339()],
        )?;
        Ok(())
    }

    fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        let downloaded_at = chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
            .map(|s| s.to_utc())
            .unwrap_or_default();

        let tags = serde_json::from_str(row.get::<_, String>(11)?.as_str()).unwrap_or_default();

        Ok(DownloadedFile {
            id: Arc::new(row.get(0)?),
            model_id: row.get(1)?,
            name: row.get(2)?,
            size: row.get(3)?,
            quantization: row.get(4)?,
            prompt_template: row.get(5)?,
            reverse_prompt: row.get(6)?,
            downloaded: row.get(7)?,
            file_size: row.get(8)?,
            download_dir: row.get(9)?,
            downloaded_at,
            tags,
            featured: row.get(12)?,
            sha256: row.get(13)?,
        })
    }

    pub fn get_all(conn: &rusqlite::Connection) -> rusqlite::Result<HashMap<Arc<String>, Self>> {
        let mut stmt = conn.prepare("SELECT * FROM download_files")?;
        let mut rows = stmt.query([])?;
        let mut files = HashMap::new();

        while let Some(row) = rows.next()? {
            let file = Self::from_row(row)?;
            files.insert(file.id.clone(), file);
        }

        Ok(files)
    }

    pub fn get_finished(
        conn: &rusqlite::Connection,
    ) -> rusqlite::Result<HashMap<Arc<String>, Self>> {
        let mut stmt = conn.prepare("SELECT * FROM download_files WHERE downloaded = TRUE")?;
        let mut rows = stmt.query([])?;
        let mut files = HashMap::new();

        while let Some(row) = rows.next()? {
            let file = Self::from_row(row)?;
            files.insert(file.id.clone(), file);
        }

        Ok(files)
    }

    pub fn get_pending(
        conn: &rusqlite::Connection,
    ) -> rusqlite::Result<HashMap<Arc<String>, Self>> {
        let mut stmt = conn.prepare("SELECT * FROM download_files WHERE downloaded = FALSE")?;
        let mut rows = stmt.query([])?;
        let mut files = HashMap::new();

        while let Some(row) = rows.next()? {
            let file = Self::from_row(row)?;
            files.insert(file.id.clone(), file);
        }

        Ok(files)
    }

    pub fn get_by_models<S: AsRef<str> + rusqlite::ToSql>(
        conn: &rusqlite::Connection,
        ids: &[S],
    ) -> rusqlite::Result<HashMap<Arc<String>, Self>> {
        let placeholders = std::iter::repeat("?")
            .take(ids.len())
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!(
            "SELECT * FROM download_files WHERE model_id IN ({}) AND downloaded = TRUE",
            placeholders
        );

        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(rusqlite::params_from_iter(ids))?;

        let mut files = HashMap::new();

        while let Some(row) = rows.next()? {
            let file = Self::from_row(row)?;
            files.insert(file.id.clone(), file);
        }

        Ok(files)
    }

    pub fn get_by_id(conn: &rusqlite::Connection, id: &str) -> rusqlite::Result<Self> {
        conn.query_row("SELECT * FROM download_files WHERE id = ?1", [id], |row| {