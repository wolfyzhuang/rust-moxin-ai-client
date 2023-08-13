use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use moxin_protocol::data::FileID;
use serde::{Deserialize, Serialize};

use super::filesystem::moxin_home_dir;
const PREFERENCES_FILE: &str = "preferences.json";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Preferences {
    pub current_chat_model: Option<File