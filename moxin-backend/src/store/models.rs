use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use rusqlite::params;

pub use super::remote::Author;

pub fn create_table_models(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS models (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            summary TEXT NOT NULL,
            size TEXT NOT NULL,
            requires TEXT NOT NULL,
            architecture TEXT NOT NULL,
            released_at TEXT NOT NULL,
            prompt_template TEXT DEFAULT '',
            reverse_prompt TEXT DEFAULT '',
            author_name TEXT NOT NULL,
            author_url TEXT NOT NULL,
            author_description TEXT NOT NULL,
            like_count INTEGER NOT NULL,
            download_count INTEGER NOT NULL
        )",
        (),
    )?;
    Ok(())
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Model {
    pub id: Arc<String>,
    pub name: String,
    pub summary: String,
    pub size: String,
    pub requires: String,
    pub architecture: String,
    pub released_at: DateTime<Utc>,
    pub prompt_te