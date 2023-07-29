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
        align: