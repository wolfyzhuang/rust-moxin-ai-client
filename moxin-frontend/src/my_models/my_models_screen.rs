
use makepad_widgets::*;
use moxin_protocol::data::DownloadedFile;

use crate::{data::store::Store, shared::utils::BYTES_PER_MB};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    import crate::my_models::downloaded_files_table::DownloadedFilesTable;

    BG_IMAGE = dep("crate://self/resources/images/my_models_bg_image.png")
    ICON_EDIT_FOLDER = dep("crate://self/resources/icons/edit_folder.svg")
    ICON_SEARCH = dep("crate://self/resources/icons/search.svg")
    ICON_SHOW_IN_FILES = dep("crate://self/resources/icons/visibility.svg")

    DownloadLocationButton = <MoxinButton> {
        width: Fit,
        height: 28,
        padding: {top: 6, bottom: 6, left: 14, right: 14}

        draw_bg: {
            radius: 2.0,
            color: #FEFEFE,
        }

        draw_icon: {
            svg_file: (ICON_EDIT_FOLDER),
            color: #000,
        }
        icon_walk: { margin: { top: 2 } }

        draw_text:{
            text_style: <REGULAR_FONT>{font_size: 11},
            color: #000
        }
        text: "Change Download Location"
        enabled: false
    }

    ShowInFilesButton = <MoxinButton> {
        width: Fit,
        height: 28,
        margin: {left: 10}
        padding: {top: 6, bottom: 6, left: 14, right: 14}

        draw_bg: {
            radius: 2.0,
            color: #FEFEFE,
            color_hover: #999,
        }

        draw_icon: {
            svg_file: (ICON_SHOW_IN_FILES),
            color: #000,
        }
        icon_walk: { margin: { top: 4 } }

        draw_text:{
            text_style: <REGULAR_FONT>{font_size: 11},
            color: #000
        }
        text: "Show in finder"
    }

    SearchBar = <RoundedView> {
        width: Fit,
        height: Fit,

        show_bg: true,
        draw_bg: {
            color: #fff
        }

        padding: {top: 3, bottom: 3, left: 20, right: 20}

        spacing: 4,
        align: {x: 0.0, y: 0.5},

        draw_bg: {
            radius: 9.0,
            border_color: #D0D5DD,
            border_width: 1.0,
        }

        <Icon> {
            draw_icon: {
                svg_file: (ICON_SEARCH),
                fn get_color(self) -> vec4 {
                    return #666;
                }
            }
            icon_walk: {width: 14, height: Fit}
        }

        input = <MoxinTextInput> {
            width: 260,
            height: Fit,
