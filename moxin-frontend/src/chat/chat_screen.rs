use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::chat::chat_panel::ChatPanel;
    import crate::chat::chat_history::ChatHistory;

    ChatScreen = {{ChatScreen}} {
        width: Fill,
        height: Fill,
        margin: {top: 48, right: 48, bottom: 48, left: 20},
        spacing: 50,

        <View> {
            width: 270,
            height: Fill,

            chat_history = <ChatHistory> {}
        }

        chat_panel = <ChatPanel> {
       