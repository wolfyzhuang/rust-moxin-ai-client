
use crate::{
    data::store::Store,
    shared::{modal::ModalAction, utils::hugging_face_model_url},
};
use makepad_widgets::*;
use moxin_protocol::data::{FileID, ModelID};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::resource_imports::*;

    MoxinHtml = <Html> {
        draw_fixed: { color: #x0 }
        draw_block: {
            code_color: (#EAECF0)
        }
        font_size: 10
        code_layout: { line_spacing: (5.0), padding: 15, }
    }

    ModelInfoModal = {{ModelInfoModal}} {
        width: Fit
        height: Fit

        wrapper = <RoundedView> {
            flow: Down
            width: 800
            height: Fit
            padding: {top: 50, right: 30 bottom: 30 left: 50}
            spacing: 5

            show_bg: true
            draw_bg: {
                color: #fff
                radius: 3
            }

            <View> {
                width: Fill,
                height: Fit,
                flow: Right

                title = <View> {
                    width: Fit,
                    height: Fit,
                    padding: {bottom: 20}

                    filename = <Label> {
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 13},
                            color: #000
                        }
                    }
                }

                filler_x = <View> {width: Fill, height: Fit}

                close_button = <RoundedView> {
                    width: Fit,
                    height: Fit,
                    align: {x: 0.5, y: 0.5}
                    cursor: Hand

                    button_icon = <Icon> {
                        draw_icon: {
                            svg_file: (ICON_CLOSE),
                            fn get_color(self) -> vec4 {
                                return #000;
                            }
                        }
                        icon_walk: {width: 12, height: 12}
                    }
                }
            }

            file_dir = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 8
                // Hack to align the text with the html block, 0.5 it not visually centered
                align: {x: 0.0, y: 0.6}

                <Label> {
                    text: "Read from"
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 10},
                        color: #344054
                    }
                }
                path = <MoxinHtml> {
                    width: Fill
                    font_size: 10
                    code_layout: { line_spacing: (5.0), padding: 9 }
                }
            }

            body = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 20,

                metadata = <MoxinHtml> {}
                actions = <View> {
                    width: Fill, height: Fit
                    flow: Right,
                    align: {x: 0.0, y: 0.5}
                    spacing: 20

                    copy_button = <RoundedView> {
                        width: Fit,
                        height: Fit,
                        padding: {top: 10, bottom: 10, left: 14, right: 14}
                        cursor: Hand
                        flow: Right,
                        spacing: 10

                        icon = <Icon> {
                            draw_icon: {
                                svg_file: (ICON_COPY)
                                fn get_color(self) -> vec4 {
                                    return #x0;
                                }
                            }
                            icon_walk: {width: 14, height: 14}
                        }

                        draw_bg: {
                            instance radius: 2.0,
                            border_color: #D0D5DD,
                            border_width: 1.2,
                            color: #EDFCF2,
                        }

                        <Label> {
                            text: "Copy to Clipboard"
                            draw_text:{
                                text_style: <REGULAR_FONT>{font_size: 10},
                                color: #x0
                            }
                        }
                    }
                    external_link = <RoundedView> {
                        width: Fit,
                        height: Fit,
                        padding: {top: 10, bottom: 10, left: 14, right: 14}
                        cursor: Hand

                        draw_bg: {
                            instance radius: 2.0,
                            border_color: #D0D5DD,