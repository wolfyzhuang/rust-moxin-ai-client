
use crate::data::store::{Store, StoreAction};
use crate::landing::search_loading::SearchLoadingWidgetExt;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::landing::model_card::ModelCard;
    import crate::landing::search_loading::SearchLoading;

    ModelList = {{ModelList}} {
        width: Fill,
        height: Fill,

        flow: Overlay,

        content = <View> {
            width: Fill,
            height: Fill,
            list = <PortalList> {
                width: Fill,
                height: Fill,

                // We need this setting because we will have modal dialogs that should
                // "capture" the events, so we don't want to handle them here.
                capture_overload: false,

                Model = <ModelCard> {
                    margin: {bottom: 30},
                }
            }
        }

        loading = <View> {
            width: Fill,
            height: Fill,
            visible: false,

            show_bg: true,
            draw_bg: {
                color: #FFFE,