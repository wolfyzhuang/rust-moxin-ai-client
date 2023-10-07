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
    import crate::landing::mo