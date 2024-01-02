use makepad_widgets::*;
use moxin_protocol::data::{DownloadedFile, FileID};

use crate::my_models::{delete_model_modal::DeleteModelAction, model_info_modal::ModelInfoAction};
use crate::shared::{actions::ChatAction, modal::ModalAction};
use crate::shared::utils::format_model_size;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    ICON_START_CHAT = dep("crate://self/resources/icons/start_chat.svg")
    ICON_PLAY = dep("c