
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::shared::external_link::*;


    ModelLink = <View> {
        width: Fit,
        height: Fit,
        flow: Down,
        link = <LinkLabel> {
            width: Fit,
            draw_text: {
                text_style: <REGULAR_FONT>{font_size: 9},
                fn get_color(self) -> vec4 {
                    return mix(
                        mix(
                            MODEL_LINK_FONT_COLOR,
                            MODEL_LINK_FONT_COLOR,
                            self.hover
                        ),