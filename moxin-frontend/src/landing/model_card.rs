
use crate::data::store::{ModelWithPendingDownloads, Store};
use crate::shared::external_link::ExternalLinkWidgetExt;
use crate::shared::modal::ModalAction;
use crate::shared::utils::hugging_face_model_url;
use makepad_widgets::*;
use moxin_protocol::data::ModelID;
use unicode_segmentation::UnicodeSegmentation;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::resource_imports::*;
    import crate::shared::widgets::*;
    import crate::landing::shared::*;
    import crate::landing::model_files::ModelFiles;
    import crate::shared::external_link::*;

    ICON_DOWNLOADS = dep("crate://self/resources/icons/downloads.svg")
    ICON_FAVORITE = dep("crate://self/resources/icons/favorite.svg")
    ICON_EXTERNAL_LINK = dep("crate://self/resources/icons/external_link.svg")

    ModelHeading = <View> {
        flow: Down,
        width: Fill,
        height: Fit,

        spacing: 10,

        <View> {
            width: Fill,
            height: Fit,

            spacing: 10,
            align: {x: 0.0, y: 0.5},

            <View> {
                width: Fit,
                height: Fit,
                model_name = <Label> {
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 16},
                        color: #000
                    }
                }
            }


            <VerticalFiller> {}

            model_like_count = <ModelAttributeTag> {
                width: Fit,
                height: Fit,

                padding: {top: 6, bottom: 6, left: 6, right: 10}

                draw_bg: {
                    color: #0000,
                    border_color: #98A2B3,
                    border_width: 1.0,
                },
                attr_name = <Icon> {
                    draw_icon: {
                        svg_file: (ICON_FAVORITE),
                        fn get_color(self) -> vec4 {
                            return #000;
                        }
                    }
                    icon_walk: {width: 14, height: 14}
                }

                attr_value = {
                    margin: {left: 5},
                    draw_text: {
                        color: #000
                        text_style: <REGULAR_FONT>{font_size: 9},
                    }
                }
            }

            model_download_count = <ModelAttributeTag> {
                width: Fit,
                height: Fit,

                padding: {top: 6, bottom: 6, left: 6, right: 10}

                draw_bg: {
                    color: #0000,
                    border_color: #98A2B3,
                    border_width: 1.0,
                },
                attr_name = <Icon> {
                    draw_icon: {
                        svg_file: (ICON_DOWNLOADS),
                        fn get_color(self) -> vec4 {
                        return #000;
                        }
                    }
                    icon_walk: {width: 12, height: 12}
                }

                attr_value = {
                    margin: {left: 5},
                    draw_text: {
                        color: #000
                        text_style: <REGULAR_FONT>{font_size: 9},
                    }
                }
            }


            <View> {
                width: 260,
                height: Fit,
                model_released_at_tag = <ModelAttributeTag> {
                    width: Fit,
                    height: Fit,

                    draw_bg: {
                        color: #0000,
                        border_color: #98A2B3,
                        border_width: 1.0,
                    },
                    attr_name = {
                        draw_text: { color: #000 }
                        text: "Released"
                    }
                    attr_value = {
                        margin: {left: 10},
                        draw_text: { color: #000 }
                    }
                }
            }
        }
        <ModelAttributes> {}
    }

    ModelSummary = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 20,
        padding: { right: 100 }

        <Label> {
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 11},
                color: #000
            }
            text: "Model Summary"
        }
        model_summary = <Label> {
            width: Fill,
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 9},
                word: Wrap,
                color: #000
            }
        }

        view_all_button = <ModelLink> {
            link = { text: "View All" }
        }
    }

    ExternalLinkIcon = <Icon> {
        draw_icon: {
            svg_file: (ICON_EXTERNAL_LINK),
            fn get_color(self) -> vec4 {
                return (MODEL_LINK_FONT_COLOR);
            }
        }
        icon_walk: {width: 14, height: 14}
    }

    ModelDetails = <View> {
        width: 400,
        height: Fit,
        flow: Down,
        spacing: 20,