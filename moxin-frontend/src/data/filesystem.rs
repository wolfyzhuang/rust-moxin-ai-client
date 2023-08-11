use std::path::PathBuf;
use std::{env, fs};

// Note that .moxin will create a hidden folder in unix-like systems.
// However in Windows the folder will be visible by default.
pub const DEFAULT_DOWNLOADS_DIR: &str = ".moxin/model_downloads";
pub const MOXIN_HOME_DIR: &str = ".moxin";

pub fn setup_model_downloads_folder() -> String {
    let home_dir = home_dir();
    let downloads_dir = PathBuf::from(home_dir).join(DEFAULT_DOWNLOADS_DIR);

    if fs::create_dir_all(&downloads_dir).is_err() {
        eprintln!(
            "Faile