use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use moxin_protocol::data::FileID;
use serde::{Deserialize, Serialize};

use super::fil