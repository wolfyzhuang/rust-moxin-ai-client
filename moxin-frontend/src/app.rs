
use crate::chat::chat_panel::ChatPanelAction;
use crate::data::store::*;
use crate::landing::model_card::{ModelCardViewAllModalWidgetRefExt, ViewAllModalAction};
use crate::landing::model_files_item::ModelFileItemAction;
use crate::my_models::delete_model_modal::{DeleteModelAction, DeleteModelModalWidgetRefExt};
use crate::my_models::model_info_modal::{ModelInfoAction, ModelInfoModalWidgetRefExt};
use crate::shared::actions::{ChatAction, DownloadAction};
use crate::shared::download_notification_popup::{
    DownloadNotificationPopupWidgetRefExt, DownloadResult, PopupAction,
};
use crate::shared::modal::ModalWidgetRefExt;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::modal::*;
    import crate::shared::widgets::SidebarMenuButton;
    import crate::shared::download_notification_popup::DownloadNotificationPopup;
    import crate::landing::landing_screen::LandingScreen;
    import crate::landing::model_card::ModelCardViewAllModal;
    import crate::chat::chat_screen::ChatScreen;
    import crate::my_models::my_models_screen::MyModelsScreen;
    import crate::my_models::delete_model_modal::DeleteModelModal;
    import crate::my_models::model_info_modal::ModelInfoModal;


    ICON_DISCOVER = dep("crate://self/resources/icons/discover.svg")
    ICON_CHAT = dep("crate://self/resources/icons/chat.svg")
    ICON_MY_MODELS = dep("crate://self/resources/icons/my_models.svg")

    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(1440, 1024)},
            pass: {clear_color: #fff}

            body = {
                flow: Overlay
                width: Fill,
                height: Fill,

                root = <View> {
                    width: Fill,
                    height: Fill,

                    sidebar_menu = <RoundedView> {
                        width: 100,