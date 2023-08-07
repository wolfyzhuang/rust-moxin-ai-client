
use crate::{
    data::store::Store,
    shared::{actions::ChatAction, utils::format_model_size},
};
use makepad_widgets::*;
use moxin_protocol::data::DownloadedFile;

use super::model_selector_list::{ModelSelectorAction, ModelSelectorListWidgetExt};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;

    import crate::chat::model_info::ModelInfo;
    import crate::chat::model_selector_list::ModelSelectorList;

    ModelSelectorButton = <RoundedView> {
        width: Fill,
        height: 54,

        align: {x: 0.0, y: 0.5},
        padding: 16,

        draw_bg: {
            instance radius: 3.0,
            color: #F9FAFB,
            border_color: #DFDFDF,
            border_width: 1.0,
        }

        cursor: Hand,

        choose = <View> {
            width: Fill,
            height: Fit,

            align: {x: 0.5, y: 0.5},

            label = <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 11},
                    color: #000
                }
                text: "Choose a model"
            }
        }
        selected = <ModelInfo> {
            width: Fit,
            height: Fit,
            show_bg: false,
            visible: false,

            label = {
                draw_text: {
                    text_style: <BOLD_FONT>{font_size: 11},
                }
            }
        }
    }

    ModelSelectorOptions = <RoundedView> {
        width: Fill,
        height: Fit,

        margin: { top: 5 },
        padding: 5,

        draw_bg: {
            instance radius: 3.0,
            color: #fff,
            border_color: #B6B6B6,
            border_width: 1.0,
        }

        list_container = <View> {
            width: Fill,
            height: 0,
            scroll_bars: <ScrollBars> {}

            list = <ModelSelectorList> {
                width: Fill,
                height: Fit,
            }
        }
    }

    ModelSelector = {{ModelSelector}} {
        width: Fill,
        height: Fit,

        flow: Down,

        button = <ModelSelectorButton> {}
        options = <ModelSelectorOptions> {}

        open_animation_progress: 0.0,
        animator: {
            open = {
                default: hide,
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80, d2: 0.97}