
use crate::data::store::{ModelWithPendingDownloads, StoreAction};
use makepad_widgets::*;

use super::model_files_list::ModelFilesListWidgetExt;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;

    import crate::landing::model_files_item::ModelFilesRow;
    import crate::landing::model_files_list::ModelFilesList;

    ICON_ADD = dep("crate://self/resources/icons/add.svg")
    ICON_REMOVE = dep("crate://self/resources/icons/remove.svg")

    ModelFilesHeader = <ModelFilesRow> {
        show_bg: true,
        draw_bg: {
            color: #F2F4F7
            radius: vec2(3.0, 0.5)
        }

        cell1 = {
            height: 40
            <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 9},
                    color: #667085
                }
                text: "File name"
            }
        }

        cell2 = {
            height: 40
            <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 9},
                    color: #667085
                }
                text: "Full Size"
            }
        }

        cell3 = {
            height: 40
            <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 9},
                    color: #667085
                }
                text: "Quantization"
            }
        }
        cell4 = {
            height: 40
        }
    }

    FooterLink = <View> {
        cursor: Hand,
        align: {x: 0.0, y: 0.5},
        spacing: 10,
        icon = <Icon> {
            draw_icon: {
                svg_file: (ICON_ADD),
                fn get_color(self) -> vec4 {
                    return #667085;
                }
            }
            icon_walk: {width: 14, height: 14}
        }
        link = <Label> {
            width: Fit,
            draw_text: {
                text_style: <BOLD_FONT>{font_size: 9},
                color: #667085,
            }
        }
    }

    ModelFilesFooter = <RoundedYView> {
        width: Fill, height: 56, padding: 10, align: {x: 0.0, y: 0.5},

        show_bg: true,
        draw_bg: {
            color: #fff
            radius: vec2(1.0, 3.0)
        }

        all_files_link = <FooterLink> {
            icon = { draw_icon: { svg_file: (ICON_ADD) }}
            link = { text: "Show All Files (12)" }
        }
