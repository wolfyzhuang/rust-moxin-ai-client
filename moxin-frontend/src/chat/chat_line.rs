
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
        draw_bg: { border_color: #D0D5DD, border_width: 1.0, color: #fff }

        button_label = {
            draw_text: {
                text_style: <REGULAR_FONT>{font_size: 9},
                fn get_color(self) -> vec4 {
                    return #000;
                }
            }
            text: "Cancel"
        }
    }

    MESSAGE_TEXT_COLOR = #000
    TEXT_HEIGHT_FACTOR = 1.3
    LINE_SPACING = 8.0
    BLOCK_LINE_SPACING = 12.0

    MessageText = <Markdown>{
        padding: 0,
        line_spacing: (LINE_SPACING),
        paragraph_spacing: 20.0,
        width: Fill, height: Fit,
        font_size: 10.0,
        draw_normal: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_italic: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_bold: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_bold_italic: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_fixed: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_block: {
            line_color: (MESSAGE_TEXT_COLOR)
            sep_color: (#EDEDED)
            quote_bg_color: (#EDEDED)
            quote_fg_color: (#969696)
            block_color: (#EDEDED)
            code_color: (#EDEDED)
        }
        list_item_layout: { line_spacing: 5.0, padding: {left: 10.0, right:10, top: 6.0, bottom: 0}, }
        list_item_walk:{margin:0, height:Fit, width:Fill}
        code_layout: { line_spacing: (BLOCK_LINE_SPACING), padding: {top: 10.0, bottom: 10.0}}
        quote_layout: { line_spacing: (BLOCK_LINE_SPACING), padding: {top: 10.0, bottom: 10.0}}
    }

    EditTextInput = <MoxinTextInput> {
        width: Fill,
        height: Fit,
        padding: 0,
        empty_message: ""

        draw_bg: {
            color: #fff
        }
        draw_text: {
            text_style:<REGULAR_FONT>{height_factor: (1.3*1.3), font_size: 10},
            word: Wrap,

            instance prompt_enabled: 0.0
            fn get_color(self) -> vec4 {
                return #000;
            }
        }
    }

    ChatLineBody = <View> {
        width: Fill,
        height: Fit,
        spacing: 5,
        flow: Down,

        <View> {
            height: 20,
            align: {x: 0.0, y: 0.5},

            role = <Label> {
                width: Fit,
                height: Fit,
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 10},
                    color: #000
                }
            }
        }

        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            padding: {top: 12, bottom: 12},
            align: {x: 0.5, y: 0.0},

            input_container = <View> {
                visible: false,
                width: Fill,
                height: Fit,
                input = <EditTextInput> {
                }
            }

            loading_container = <View> {
                width: Fill,
                height: Fit,
                loading = <ChatLineLoading> {}
            }

            markdown_message_container = <View> {
                width: Fill,
                height: Fit,
                markdown_message = <MessageText> {}
            }

            plain_text_message_container = <View> {
                width: Fill,
                height: Fit,
                plain_text_message = <Label> {
                    width: Fill,
                    height: Fit,
                    draw_text: {
                        text_style: <REGULAR_FONT>{height_factor: (1.3*1.3), font_size: 10},
                        color: #000
                    }
                }
            }

            edit_buttons = <View> {
                visible: false,
                width: Fit,
                height: Fit,
                margin: {top: 10},
                spacing: 6,
                save = <SaveButton> {}
                save_and_regenerate = <SaveAndRegerateButton> {}
                cancel = <CancelButton> {}
            }
        }
    }

    ChatLineActionButton = <Button> {
        width: 14
        height: 14
        draw_icon: {
            fn get_color(self) -> vec4 {
                return #BDBDBD;
            }
        }
        padding: 0,
        icon_walk: {width: 14, height: 14}
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        text: ""
    }

    ChatLine = {{ChatLine}} {
        padding: {top: 10, bottom: 3},
        width: Fill,
        height: Fit,

       // cursor: Default,

        avatar_section = <View> {
            width: Fit,
            height: Fit,
            margin: {left: 20, right: 20},
        }

        main_section = <View> {
            width: Fill,
            height: Fit,

            flow: Down,
            spacing: 8,

            body_section = <ChatLineBody> {}

            actions_section = <View> {
                width: Fill,
                height: 16,
                actions = <View> {
                    width: Fill,
                    height: Fit,
                    visible: false,
                    spacing: 6,

                    copy_button = <ChatLineActionButton> {
                        draw_icon: { svg_file: (ICON_COPY) }
                    }
                    edit_button = <ChatLineActionButton> {
                        draw_icon: { svg_file: (ICON_EDIT) }
                    }
                    delete_button = <ChatLineActionButton> {