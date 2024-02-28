
use makepad_widgets::*;
use moxin_protocol::data::{File, FileID};

use crate::shared::actions::DownloadAction;

use super::modal::ModalAction;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::resource_imports::*;
    import crate::landing::shared::*;
    import makepad_draw::shader::std::*;

    SUCCESS_ICON = dep("crate://self/resources/images/success_icon.png")
    FAILURE_ICON = dep("crate://self/resources/images/failure_icon.png")

    PRIMARY_LINK_FONT_COLOR = #x0E7090
    SECONDARY_LINK_FONT_COLOR = #667085

    PopupActionLink = <LinkLabel> {
        width: Fit,
        draw_text: {
            text_style: <BOLD_FONT>{font_size: 9},
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        PRIMARY_LINK_FONT_COLOR,
                        PRIMARY_LINK_FONT_COLOR,
                        self.hover
                    ),
                    PRIMARY_LINK_FONT_COLOR,
                    self.pressed
                )
            }
        }
    }

    PopupSecondaryActionLink = <LinkLabel> {
        width: Fit,
        draw_text: {
            text_style: <BOLD_FONT>{font_size: 9},
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        SECONDARY_LINK_FONT_COLOR,
                        SECONDARY_LINK_FONT_COLOR,
                        self.hover
                    ),
                    SECONDARY_LINK_FONT_COLOR,
                    self.pressed
                )
            }
        }
    }

    PopupDialog = <RoundedView> {
        width: 350
        height: Fit
        margin: {top: 20, right: 20}
        padding: {top: 20, right: 20 bottom: 20 left: 20}
        spacing: 15

        show_bg: true
        draw_bg: {
            color: #fff
            instance border_radius: 4.0
            fn pixel(self) -> vec4 {
                let border_color = #d4;
                let border_width = 1;
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let body = #fff

                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    self.border_radius
                )
                sdf.fill_keep(body)

                sdf.stroke(
                    border_color,
                    border_width
                )
                return sdf.result
            }
        }
    }

    PopupCloseButton = <RoundedView> {
        width: Fit,
        height: Fit,
        align: {x: 0.5, y: 0.5}
        cursor: Hand

        button_icon = <Icon> {
            draw_icon: {
                svg_file: (ICON_CLOSE),
                fn get_color(self) -> vec4 {
                    return #000;
                }
            }
            icon_walk: {width: 10, height: 10}
        }
    }

    NotificationIcons = <View> {
        width: Fit,
        height: Fit,
        margin: {top: -10, left: -10}
        success_icon = <View> {
            width: Fit,
            height: Fit,
            <Image> {
                source: (SUCCESS_ICON),
                width: 35,
                height: 35,
            }
        }
        failure_icon = <View> {
            visible: false,
            width: Fit,
            height: Fit,
            <Image> {
                source: (FAILURE_ICON),
                width: 35,
                height: 35,
            }
        }
    }

    NotificationContent = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 10

        title = <Label> {
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 9},
                word: Wrap,
                color: #000
            }
            text: "Model Downloaded Successfully"
        }

        summary = <Label> {
            width: Fill,
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 9},
                word: Wrap,
                color: #000
            }
            text: ""
        }

        success_actions = <View> {
            width: Fit,
            height: Fit,
            view_in_my_models_link = <PopupActionLink> {
                text: "View in My Models"
            }
        }

        failure_actions = <View> {
            width: Fit,
            height: Fit,
            spacing: 10,

            retry_link = <PopupActionLink> {
                text: "Retry"
            }

            cancel_link = <PopupSecondaryActionLink> {
                text: "Cancel"
            }
        }
    }

    DownloadNotificationPopup = {{DownloadNotificationPopup}} {
        width: Fit
        height: Fit

        <PopupDialog> {