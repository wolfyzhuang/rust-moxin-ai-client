use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::landing::model_card::ModelCard;

    ANIMATION_SPEED = 0.33

    Bar = <View> {
        width: Fill,
        height: 16,
        show_bg: true,
        draw_bg: {
            instance dither: 0.3

            fn get_color(self) -> vec4 {
                return mix(
                    #F3FFA2,
                    #E3FBFF,
                    self.pos.x + self.dither
                )
            }

            fn p