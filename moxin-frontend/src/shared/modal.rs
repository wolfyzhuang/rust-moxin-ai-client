
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;

    ModalView = {{ModalView}} {
        width: Fill
        height: Fill
        flow: Overlay
        align: {x: 0.5, y: 0.5}

        bg_view = <View> {
            width: Fill
            height: Fill
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return vec4(0., 0., 0., 0.7)
                }
            }
        }

        content = <View> {
            flow: Overlay
            width: Fit
            height: Fit
            // This assures that the hit events gets consummed when clicking the content, so it closes when clicking outside of it.
            cursor: Arrow
        }
    }

    Modal = {{Modal}} {
        width: Fill
        height: Fill

        flow: Right
    }
}

// TODO: look for a way of being able to add custom "ModalView" inside de model
// (which should become something like "Portal") maybe with traits.
#[derive(Live, LiveHook, LiveRegisterWidget, WidgetRef)]
pub struct ModalView {
    #[deref]
    view: View,
}

impl Widget for ModalView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let widget_uid = self.widget_uid();

        self.view(id!(content)).handle_event(cx, event, scope);

        match event.hits(cx, self.view(id!(bg_view)).area()) {
            Hit::FingerUp(_fe) => {
                cx.widget_action(widget_uid, &scope.path, ModalAction::CloseModal);
            }
            _ => (),
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view
            .draw_walk(cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }))
    }
}

impl WidgetNode for ModalView {
    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }
}

impl ModalView {
    pub fn show(&mut self, cx: &mut Cx) {
        self.apply_over(cx, live! {visible: true});
        self.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.apply_over(cx, live! {visible: false});
        self.redraw(cx);
    }

    pub fn is_showing(&self) -> bool {
        self.view.is_visible()
    }
}

impl ModalViewRef {
    pub fn show(&mut self, cx: &mut Cx) {