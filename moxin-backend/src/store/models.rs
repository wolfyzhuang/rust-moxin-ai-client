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
            prompt