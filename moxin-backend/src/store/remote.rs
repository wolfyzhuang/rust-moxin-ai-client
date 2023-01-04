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

#[derive(Debug