
use crate::{
    data::store::Store,
    shared::{modal::ModalAction, utils::hugging_face_model_url},
};
use makepad_widgets::*;
use moxin_protocol::data::{FileID, ModelID};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::resource_imports::*;

    MoxinHtml = <Html> {
        draw_fixed: { color: #x0 }
        draw_block: {
            code_color: (#EAECF0)
        }
        font_size: 10
        code_layout: { line_spacing: (5.0), padding: 15, }
    }

    ModelInfoModal = {{ModelInfoModal}} {
        width: Fit
        height: Fit

        wrapper = <RoundedView> {
            flow: Down
            width: 800
            height: Fit
            padding: {top: 50, right: 30 bottom: 30 left: 50}
            spacing: 5

            show_bg: true
            draw_bg: {
                color: #fff
                radius: 3
            }

            <View> {
                width: Fill,
                height: Fit,
                flow: Right

                title = <View> {
                    width: Fit,
                    height: Fit,
                    padding: {bottom: 20}

                    filename = <Label> {
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 13},
                            color: #000
                        }
                    }
                }

                filler_x = <View> {width: Fill, height: Fit}

                close_button = <RoundedView> {
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
                        icon_walk: {width: 12, height: 12}
                    }
                }
            }

            file_dir = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 8
                // Hack to align the text with the html block, 0.5 it not visually centered
                align: {x: 0.0, y: 0.6}

                <Label> {
                    text: "Read from"
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 10},
                        color: #344054
                    }
                }
                path = <MoxinHtml> {
                    width: Fill
                    font_size: 10
                    code_layout: { line_spacing: (5.0), padding: 9 }
                }
            }

            body = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 20,

                metadata = <MoxinHtml> {}
                actions = <View> {
                    width: Fill, height: Fit
                    flow: Right,
                    align: {x: 0.0, y: 0.5}
                    spacing: 20

                    copy_button = <RoundedView> {
                        width: Fit,
                        height: Fit,
                        padding: {top: 10, bottom: 10, left: 14, right: 14}
                        cursor: Hand
                        flow: Right,
                        spacing: 10

                        icon = <Icon> {
                            draw_icon: {
                                svg_file: (ICON_COPY)
                                fn get_color(self) -> vec4 {
                                    return #x0;
                                }
                            }
                            icon_walk: {width: 14, height: 14}
                        }

                        draw_bg: {
                            instance radius: 2.0,
                            border_color: #D0D5DD,
                            border_width: 1.2,
                            color: #EDFCF2,
                        }

                        <Label> {
                            text: "Copy to Clipboard"
                            draw_text:{
                                text_style: <REGULAR_FONT>{font_size: 10},
                                color: #x0
                            }
                        }
                    }
                    external_link = <RoundedView> {
                        width: Fit,
                        height: Fit,
                        padding: {top: 10, bottom: 10, left: 14, right: 14}
                        cursor: Hand

                        draw_bg: {
                            instance radius: 2.0,
                            border_color: #D0D5DD,
                            border_width: 1.2,
                            color: #F5FEFF,
                        }

                        <Label> {
                            text: "Model Card on Hugging Face"
                            draw_text:{
                                text_style: <REGULAR_FONT>{font_size: 10},
                                color: #x0
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ModelInfoModal {
    #[deref]
    view: View,
    #[rust]
    file_id: String,
    #[rust]
    model_id: String,
    #[rust]
    stringified_model_data: String,
}

impl Widget for ModelInfoModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let downloaded_files = &scope.data.get::<Store>().unwrap().downloaded_files;
        let downloaded_file = downloaded_files
            .iter()
            .find(|f| f.file.id.eq(&self.file_id))
            .expect("Downloaded file not found");

        self.model_id = downloaded_file.model.id.clone();

        // filename
        self.label(id!(title.filename))
            .set_text(&downloaded_file.file.name);

        // file path
        if let Some(path) = &downloaded_file.file.downloaded_path {
            self.html(id!(file_dir.path))
                .set_text(&format!("<pre>{}</pre>", path));
        } else {
            self.view(id!(file_dir)).set_visible(false);
        }

        // metadata
        self.stringified_model_data = serde_json::to_string_pretty(&downloaded_file.model)
            .expect("Could not serialize model data into json");
        let metadata = format!("<pre>{}</pre>", self.stringified_model_data);

        self.html(id!(wrapper.body.metadata)).set_text(&metadata);

        self.view
            .draw_walk(cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }))
    }
}

impl WidgetMatchEvent for ModelInfoModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let widget_uid = self.widget_uid();

        if let Some(fe) = self.view(id!(close_button)).finger_up(actions) {
            if fe.was_tap() {
                cx.widget_action(widget_uid, &scope.path, ModalAction::CloseModal);
            }
        }

        if let Some(fe) = self
            .view(id!(wrapper.body.actions.copy_button))
            .finger_up(actions)
        {
            if fe.was_tap() {
                cx.copy_to_clipboard(&self.stringified_model_data);
            }
        }

        if let Some(fe) = self
            .view(id!(wrapper.body.actions.external_link))
            .finger_up(actions)
        {
            if fe.was_tap() {
                let model_url = hugging_face_model_url(&self.model_id);
                if let Err(e) = robius_open::Uri::new(&model_url).open() {
                    error!("Error opening URL: {:?}", e);
                }
            }
        }
    }
}

impl ModelInfoModalRef {
    pub fn set_file_id(&mut self, file_id: ModelID) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.file_id = file_id;
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ModelInfoAction {
    FileSelected(FileID),
    None,
}