use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import makepad_draw::shader::std::*;
    import makepad_draw::shader::draw_color::DrawColor;
    import crate::shared::widgets::*;
    import crate::shared::styles::*;

    ExternalLink = {{ExternalLink}} {
        width: Fit,
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
                        MODEL_LINK_FONT_COLOR,
                        self.pressed
                    )
                }
            }
        }
        underline = <Line> {
            width: Fill,
            height: 1,
            show_bg: true,
            draw_bg: {
                color: (MODEL_LINK_FONT_COLOR)
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ExternalLink {
    #[deref]
    view: View,

    #[rust]
    url: String,
}

impl Widget for ExternalLink {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope