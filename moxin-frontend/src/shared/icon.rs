use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    Icon = {{Icon}} {
        width: Fit,
        height: Fit,

        icon_walk: {
            margin: {left: 5.0},
            width: Fit,
            height: Fit,
        }

        draw_icon: {
            uniform rotation_angl