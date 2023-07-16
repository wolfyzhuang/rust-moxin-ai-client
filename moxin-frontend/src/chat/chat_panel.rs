
use makepad_widgets::*;
use moxin_protocol::data::{DownloadedFile, FileID};

use crate::{
    chat::{
        chat_history::ChatHistoryAction,
        chat_line::{ChatLineAction, ChatLineWidgetRefExt},
        model_selector::ModelSelectorWidgetExt,
        model_selector_list::ModelSelectorAction,
    },
    data::store::Store,
    shared::actions::ChatAction,
};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import makepad_draw::shader::std::*;

    import crate::chat::model_selector::ModelSelector;
    import crate::chat::chat_line::ChatLine;

    ICON_PROMPT = dep("crate://self/resources/icons/prompt.svg")
    ICON_STOP = dep("crate://self/resources/icons/stop.svg")
    ICON_JUMP_TO_BOTTOM = dep("crate://self/resources/icons/jump_to_bottom.svg")

    ChatAgentAvatar = <RoundedView> {
        width: 20,
        height: 20,

        show_bg: true,
        draw_bg: {
            color: #444D9A
        }

        align: {x: 0.5, y: 0.5},

        avatar_label = <Label> {
            width: Fit,
            height: Fit,
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 10},
                color: #fff,
            }
            text: "P"