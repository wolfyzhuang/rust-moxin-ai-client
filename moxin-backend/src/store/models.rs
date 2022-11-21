use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use rusqlite::params;

pub use super::remote::Author;

pub fn