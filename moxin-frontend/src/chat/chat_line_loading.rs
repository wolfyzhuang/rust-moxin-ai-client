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

            fn pixel(self) -> vec4 {
                return Pal::premul(self.get_color())
            }
        }
    }

    ChatLineLoading = {{ChatLineLoading}} {
        width: Fill,
        height: Fit,

        flow: Down,
        spacing: 4,

        line1 = <Bar> {}
        line2 = <Bar> {}
        <View> {
            width: Fill,
            height: 16,
            line3 = <Bar> {}
            <VerticalFiller> {}
        }

        animator: {
            line1 = {
                default: start,
                start = {
                    redraw: true,
                    from: {all: Forward {duration: (ANIMATION_SPEED)}}
                    apply: {line1 = { draw_bg: {dither: 0.1} }}
                }
                run = {
                    redraw: true,
                    from: {all: Forward {duration: (ANIMATION_SPEED)}}
                    apply: {line1 = { draw_bg: {dither: 0.9} }}
                }
            }

            line2 = {
                default: start,
                start = {
                    redraw: true,
                    from: {all: Forward {duration: (ANIMATION_SPEED)}}
                    apply: {line2 = { draw_bg: {dither: 0.1} }}
                }
                run = {
                    redraw: true,
                    from: {all: Forward {duration: (ANIMATION_SPEED)}}
                    apply: {line2 = { draw_bg: {dither: 0.9} }}
                }
          