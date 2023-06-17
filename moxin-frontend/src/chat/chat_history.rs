
use crate::data::{chat::ChatID, store::Store};

use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import makepad_draw::shader::std::*;

    ChatCard = {{ChatCard}} {
        content = <RoundedView> {
            flow: Down
            width: Fill
            height: Fit
            padding: 20
            spacing: 12

            cursor: Hand

            draw_bg: { color: #fff }

            title = <Label> {
                width: Fit,
                height: Fit,
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 10},
                    color: #000,
                }
                text: ""
            }

            <View> {
                width: Fill
                height: Fit
                align: {x: 1}

                date = <Label> {
                    width: Fit,
                    height: Fit,
                    draw_text:{
                        text_style: <REGULAR_FONT>{font_size: 8},
                        color: #667085,
                    }
                    text: "5:29 PM, 5/12/24"
                }
            }


        }
    }

    ChatHistory = {{ChatHistory}} {
        flow: Down
        width: Fill
        height: Fill
        padding: 10

        list = <PortalList> {
            ChatCard = <ChatCard> {margin: {top: 20}}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatHistory {
    #[deref]
    view: View,
}

impl Widget for ChatHistory {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get::<Store>().unwrap();

        let mut saved_chat_ids = store
            .saved_chats
            .iter()
            .map(|c| c.borrow().id)
            .collect::<Vec<_>>();

        // Reverse sort chat ids.
        saved_chat_ids.sort_by(|a, b| b.cmp(a));

        let chats_count = store.saved_chats.len();

        while let Some(view_item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = view_item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, chats_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < chats_count {
                        let mut item = list
                            .item(cx, item_id, live_id!(ChatCard))
                            .unwrap()
                            .as_chat_card();
                        let _ = item.set_chat_id(saved_chat_ids[item_id]);
                        item.draw_all(cx, scope);