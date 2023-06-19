
use crate::chat::chat_line_loading::ChatLineLoadingWidgetExt;
use makepad_widgets::markdown::MarkdownWidgetExt;
use makepad_widgets::*;

use makepad_markdown::parse_markdown;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import makepad_draw::shader::std::*;
    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::shared::resource_imports::*;
    import crate::chat::chat_line_loading::ChatLineLoading;

    ICON_EDIT = dep("crate://self/resources/icons/edit.svg")
    ICON_DELETE = dep("crate://self/resources/icons/delete.svg")

    ChatLineEditButton = <RoundedView> {
        width: 56,
        height: 31,
        align: {x: 0.5, y: 0.5}
        spacing: 6,

        cursor: Hand,

        draw_bg: { color: #099250 }

        button_label = <Label> {
            draw_text: {
                text_style: <REGULAR_FONT>{font_size: 9},
                fn get_color(self) -> vec4 {
                    return #fff;
                }
            }
        }
    }

    SaveButton = <ChatLineEditButton> {
        button_label = {
            text: "Save"
        }
    }

    SaveAndRegerateButton = <ChatLineEditButton> {
        width: 130,
        button_label = {
            text: "Save & Regenerate"
        }
    }

    CancelButton = <ChatLineEditButton> {