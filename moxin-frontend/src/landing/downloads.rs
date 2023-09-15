
use crate::data::{download::DownloadState, store::Store};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::shared::icon::*;

    import crate::landing::download_item::DownloadItem;

    ICON_COLLAPSE = dep("crate://self/resources/icons/collapse.svg")

    CollapseButton = <View> {
        cursor: Hand
        width: Fit, height: Fit
        icon = <Icon> {
            draw_icon: {
                svg_file: (ICON_COLLAPSE)
                fn get_color(self) -> vec4 {
                    return #667085;
                }
            }
            icon_walk: {width: 18, height: Fit}
        }
    }

    Header = <View> {
        width: Fill,
        height: Fit,
        spacing: 25,
        padding: {right: 43},

        <Label> {
            margin: {right: 20.0},
            draw_text:{