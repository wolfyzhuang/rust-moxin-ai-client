
use makepad_widgets::*;
use moxin_protocol::data::{DownloadedFile, FileID};

use crate::{
    chat::{
        chat_history::ChatHistoryAction,
        chat_line::{ChatLineAction, ChatLineWidgetRefExt},
        model_selector::ModelSelectorWidgetExt,
        model_selector_list::ModelSelectorAction,
    },
    data::store::Store,
    shared::actions::ChatAction,
};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import makepad_draw::shader::std::*;

    import crate::chat::model_selector::ModelSelector;
    import crate::chat::chat_line::ChatLine;

    ICON_PROMPT = dep("crate://self/resources/icons/prompt.svg")
    ICON_STOP = dep("crate://self/resources/icons/stop.svg")
    ICON_JUMP_TO_BOTTOM = dep("crate://self/resources/icons/jump_to_bottom.svg")

    ChatAgentAvatar = <RoundedView> {
        width: 20,
        height: 20,

        show_bg: true,
        draw_bg: {
            color: #444D9A
        }

        align: {x: 0.5, y: 0.5},

        avatar_label = <Label> {
            width: Fit,
            height: Fit,
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 10},
                color: #fff,
            }
            text: "P"
        }
    }

    UserChatLine = <ChatLine> {
        avatar_section = {
            <Image> {
                source: dep("crate://self/resources/images/chat_user_icon.png"),
                width: 20,
                height: 20,
            }
        }
    }

    ModelChatLine = <ChatLine> {
        avatar_section = {
            <ChatAgentAvatar> {}
        }
    }

    JumpToButtom = <View> {
        width: Fill,
        height: Fill,
        align: {x: 0.5, y: 1.0},

        jump_to_bottom = <CircleView> {
            width: 34,
            height: 34,
            align: {x: 0.5, y: 0.5},
            margin: {bottom: 10},

            cursor: Hand,

            show_bg: true,

            draw_bg: {
                radius: 14.0,
                color: #fff,
                border_width: 1.0,
                border_color: #EAECF0,
            }

            <Icon> {
                padding: 0,
                // These margins are used to center the icon inside the circle
                // Not sure why the icon is not centered by default
                margin: { top: 6, right: 4 },
                draw_icon: {
                    svg_file: (ICON_JUMP_TO_BOTTOM),
                    fn get_color(self) -> vec4 {
                        return #1C1B1F;
                    }
                }
                icon_walk: {width: 12, height: 12}
            }
        }
    }

    ChatPromptInput = <RoundedView> {
        width: Fill,
        height: Fit,

        show_bg: true,
        draw_bg: {
            color: #fff
        }

        padding: {top: 6, bottom: 6, left: 4, right: 10}

        spacing: 4,
        align: {x: 0.0, y: 1.0},

        draw_bg: {
            radius: 2.0,
            border_color: #D0D5DD,
            border_width: 1.0,
        }

        prompt = <MoxinTextInput> {
            width: Fill,
            height: Fit,

            empty_message: "Enter a message"
            draw_bg: {
                color: #fff
            }
            draw_text: {
                text_style:<REGULAR_FONT>{font_size: 10},

                instance prompt_enabled: 0.0
                fn get_color(self) -> vec4 {
                    return mix(
                        #D0D5DD,
                        #000,
                        self.prompt_enabled
                    )
                }
            }
        }

        prompt_icon = <RoundedView> {
            width: 28,
            height: 28,
            show_bg: true,
            draw_bg: {
                color: #D0D5DD
            }

            cursor: Hand,

            padding: {right: 4},
            margin: {bottom: 2},
            align: {x: 0.5, y: 0.5},

            icon_send = <View> {
                width: Fit,
                height: Fit,
                <Icon> {
                    draw_icon: {
                        svg_file: (ICON_PROMPT),
                        fn get_color(self) -> vec4 {
                            return #fff;
                        }
                    }
                    icon_walk: {width: 12, height: 12}
                }
            }
            icon_stop = <View> {
                width: Fit,
                height: Fit,
                visible: false,

                <Icon> {
                    draw_icon: {
                        svg_file: (ICON_STOP),
                        fn get_color(self) -> vec4 {
                            return #fff;
                        }
                    }
                    icon_walk: {width: 12, height: 12}
                }
            }
        }
    }

    ChatPanel = {{ChatPanel}} {
        width: Fill,
        height: Fill,
        margin: {top: 0, left: 20, right: 20, bottom: 20},

        flow: Overlay,

        no_downloaded_model = <View> {
            width: Fill,
            height: Fill,

            flow: Down,
            align: {x: 0.5, y: 0.5},

            <View> {
                width: Fill,
                height: Fill,
                flow: Down,
                spacing: 30,
                align: {x: 0.5, y: 0.5},

                <Label> {
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 12},
                        color: #667085
                    }
                    text: "You haven’t downloaded any models yet."
                }
                go_to_discover_button = <RoundedView> {
                    width: Fit,
                    height: Fit,
                    cursor: Arrow,

                    draw_bg: { color: #fff, border_color: #D0D5DD, border_width: 1}

                    button_label = <Label> {
                        margin: {top: 14, right: 12, bottom: 14, left: 12}
                        text: "Go To Discover"
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 12},
                            fn get_color(self) -> vec4 {
                                return #087443;
                            }
                        }
                    }
                }
            }

            <View> {
                width: Fill, height: Fit
                flow: Down,
                align: {x: 0.5, y: 0.5},
                no_downloaded_model_prompt_input = <ChatPromptInput> {}
            }

        }

        no_model = <View> {
            width: Fill,
            height: Fill,

            flow: Down,
            align: {x: 0.5, y: 0.5},

            <View> {
                width: Fill,
                height: Fill,
                flow: Down,