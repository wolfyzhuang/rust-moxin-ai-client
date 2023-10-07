use makepad_widgets::*;
use moxin_protocol::data::{File, FileID};

use super::model_files_tags::ModelFilesTagsWidgetExt;
use crate::shared::actions::{ChatAction, DownloadAction};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::MoxinButton;
    import crate::landing::model_files_tags::ModelFilesTags;

    ICON_DOWNLOAD = dep("crate://self/resources/icons/download.svg")
    START_CHAT = dep("crate://self/resources/icons/start_chat.svg")
    RESUME_CHAT = dep("crate:/