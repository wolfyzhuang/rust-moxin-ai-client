
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    import crate::shared::styles::*;

    VerticalFiller = <View> {
        width: Fill,
        height: 1,
    }

    HorizontalFiller = <View> {
        width: 1,
        height: Fill,
    }

    Line = <View> {
        width: Fill,
        height: 1,
        show_bg: true,
        draw_bg: {
            color: #D9D9D9
        }
    }

    AttributeTag = <RoundedView> {
        width: Fit,
        height: Fit,
        padding: {top: 6, bottom: 6, left: 10, right: 10}

        spacing: 5,
        draw_bg: {
            instance radius: 2.0,
        }

        attr_name = <Label> {
            draw_text:{
                wrap: Word
                text_style: <REGULAR_FONT>{font_size: 8},
                color: #x0
            }
        }
    }

    SIDEBAR_FONT_COLOR = #344054
    SIDEBAR_FONT_COLOR_HOVER = #344054
    SIDEBAR_FONT_COLOR_SELECTED = #127487

    SIDEBAR_BG_COLOR_HOVER = #E2F1F199
    SIDEBAR_BG_COLOR_SELECTED = #E2F1F199

    SidebarMenuButton = <RadioButton> {
        width: 80,
        height: 70,
        padding: 0, margin: 0,
        flow: Down, spacing: 8.0, align: {x: 0.5, y: 0.5}

        icon_walk: {margin: 0, width: 30, height: 30}
        label_walk: {margin: 0}

        draw_radio: {
            radio_type: Tab,

            instance border_width: 0.0
            instance border_color: #0000
            instance inset: vec4(0.0, 0.0, 0.0, 0.0)
            instance radius: 2.5

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        (SIDEBAR_BG_COLOR),
                        (SIDEBAR_BG_COLOR_HOVER),
                        self.hover
                    ),
                    (SIDEBAR_BG_COLOR_SELECTED),
                    self.selected
                )
            }

            fn get_border_color(self) -> vec4 {
                return self.border_color
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box(