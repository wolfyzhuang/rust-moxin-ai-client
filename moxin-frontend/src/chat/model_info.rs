use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;

    ModelAttributeTag = <RoundedView> {
        width: Fit,
        height: Fit,
        padding: {top: 6, bottom: 6, left: 10, right: 10}

        spacing: 5,
        draw_bg: {
            radius: 2.0,
        }

        caption = <Label> {
            draw_text: {
                text_style: <REGULAR_FONT>{font_size: 9},
                color: #1D2939
            }
        }
    }

    ModelInfo = <View> {
        width: Fill,
        height: Fit,
        padding: 16,
        spacing: 10,
        align: {x: 0.0, y: 0.5},

        show_bg: true,
        draw_bg: {
            instance hover: 0.0,
            instance down: 0.0,
            color: #fff,
            instance color_hover: #F9FAFB,

            fn pixel(self) -> vec4 {
                return mix(self.color, self.color_hover, self.hover);
            }
        }

        label = <Label> {
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 11},
                color: #000
            }
        }

        architecture_tag = <ModelAttributeTag> {
            draw_bg: {
                color: #DDD7FF,
            }
        }

        params_size_tag = <ModelAttributeTag> {
            draw_bg: {
                color: #D1F4FC,
            }
        }

        file_size_tag = <ModelAttributeTag> {
            caption = {
                draw_text:{
                    color: #000
                }
            }
            draw_bg: {
                color: #fff,
                border