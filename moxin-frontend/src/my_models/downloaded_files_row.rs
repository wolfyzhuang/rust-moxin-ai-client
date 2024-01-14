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
    ICON_PLAY = dep("crate://self/resources/icons/play_arrow.svg")
    ICON_INFO = dep("crate://self/resources/icons/info.svg")
    ICON_DELETE = dep("crate://self/resources/icons/delete.svg")
    MODEL_CTA_COLOR = #127487

    DownloadedFilesRowButton = <MoxinButton> {
        height: 40

        draw_bg: {
            border_color: #ccc,
        }

        draw_icon: {
            color: #087443;
        }
    }

    DownloadedFilesRow = {{DownloadedFilesRow}} {
        // Heads-up: rows break the Portal List without fixed height
        height: 85,
        flow: Down
        width: Fill
        align: {x: 0.0, y: 0.5}

        show_bg: true
        draw_bg: {
            color: #FFF;
        }

        separator_line = <Line> {}
        h_wrapper = <View> {
            flow: Right
            width: Fit
            padding: {top: 10, bottom: 10, left: 20, right: 20}
            spacing: 30
            show_bg: true
            draw_bg: {
                color: #FFF;
            }

            model_file = <View> {
                flow: Down
                width: 600

                h_wrapper = <View> {
                    flow: Right
                    width: Fill
                    spacing: 15
                    name_tag = <View> {
                        width: Fit
                        align: {x: 0.0, y: 0.5}
                        name = <Label> {
                            width: Fit
                            draw_text: {
                                text_style: <BOLD_FONT>{font_size: 9}
                                color: #x0
                            }
                        }
                    }

                    base_model_tag = <View> {
                        width: Fit
                        align: {x: 0.0, y: 0.5}
                        base_model = <AttributeTag> {
                            draw_bg: { color: #F0D6F5 },
                        }
                    }
                    parameters_tag = <View> {
                        width: Fit
                        align: {x: 0.0, y: 0.5}
                        parameters = <AttributeTag> {
                            draw_bg: { color: #D4E6F7 },
                        }
                    }
                }
                model_version_tag = <View> {
                    width: Fit
                    align: {x: 0.0, y: 0.5}
                    version = <Label> {
                        width: Fit
                        draw_text: {
                            wrap: Ellipsis
 