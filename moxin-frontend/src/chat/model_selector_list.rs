
use crate::{data::store::Store, shared::utils::format_model_size};
use makepad_widgets::*;
use moxin_protocol::data::DownloadedFile;
use std::collections::HashMap;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;

    import crate::chat::model_info::ModelInfo;

    ModelSelectorList = {{ModelSelectorList}} {
        flow: Down,
        template: <ModelInfo> {
            // This is mandatory to listen for touch/click events
            cursor: Hand,

            animator: {
                hover = {
                    default: off
                    off = {
                        from: {all: Forward {duration: 0.2}}
                        apply: {
                            draw_bg: {hover: 0.0}
                        }
                    }

                    on = {
                        from: {all: Snap}
                        apply: {
                            draw_bg: {hover: 1.0}
                        },
                    }
                }
                down = {
                    default: off
                    off = {
                        from: {all: Forward {duration: 0.5}}
                        ease: OutExp
                        apply: {
                            draw_bg: {down: 0.0}
                        }
                    }
                    on = {
                        ease: OutExp
                        from: {
                            all: Forward {duration: 0.2}
                        }
                        apply: {
                            draw_bg: {down: 1.0}
                        }
                    }
                }
            }
        }

        no_models_message: <View> {
            width: Fill,
            height: Fit,
            padding: 14,
            spacing: 5,
            align: {x: 0.5, y: 0.5},

            <Label> {
                draw_text:{
                    text_style: <REGULAR_FONT>{font_size: 11},
                    color: #000
                }
                text: "No models available. Download a model to get started."
            }
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ModelSelectorAction {
    Selected(DownloadedFile),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct ModelSelectorList {
    #[redraw]
    #[rust]
    area: Area,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live]
    template: Option<LivePtr>,
    #[live]
    no_models_message: Option<LivePtr>,

    #[live(true)]
    visible: bool,

    #[rust]
    items: ComponentMap<LiveId, WidgetRef>,

    #[rust]
    map_to_downloaded_files: HashMap<LiveId, DownloadedFile>,

    #[rust]
    total_height: Option<f64>,
}

impl Widget for ModelSelectorList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let widget_uid = self.widget_uid();

        if let Event::Startup = event {
            let store = scope.data.get_mut::<Store>().unwrap();
            if let Some(filename) = &store.preferences.current_chat_model {
                if let Some(downloaded_file) = store
                    .downloaded_files
                    .iter()
                    .find(|file| &file.file.id == filename)
                {
                    cx.widget_action(
                        widget_uid,
                        &scope.path,
                        ModelSelectorAction::Selected(downloaded_file.to_owned()),
                    );
                }
            }
        }

        for (id, item) in self.items.iter_mut() {
            let actions = cx.capture_actions(|cx| item.handle_event(cx, event, scope));
            if let Some(fd) = item.as_view().finger_down(&actions) {
                if fd.tap_count == 1 {
                    cx.widget_action(
                        widget_uid,
                        &scope.path,
                        ModelSelectorAction::Selected(
                            self.map_to_downloaded_files.get(id).unwrap().clone(),
                        ),
                    );
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get::<Store>().unwrap();
        cx.begin_turtle(walk, self.layout);

        if self.visible {
            self.draw_items(cx, &store.downloaded_files);
        }

        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
    }
}

impl ModelSelectorList {
    fn draw_items(&mut self, cx: &mut Cx2d, items: &Vec<DownloadedFile>) {
        let mut items = items.clone();
        items.sort_by(|a, b| b.downloaded_at.cmp(&a.downloaded_at));

        if items.is_empty() {
            let item_widget = WidgetRef::new_from_ptr(cx, self.no_models_message);
            let _ = item_widget.draw_all(cx, &mut Scope::empty());
            return;
        }

        self.map_to_downloaded_files = HashMap::new();
        let mut total_height = 0.0;
        for i in 0..items.len() {
            let item_id = LiveId(i as u64).into();
            let item_widget = self
                .items
                .get_or_insert(cx, item_id, |cx| WidgetRef::new_from_ptr(cx, self.template));
            self.map_to_downloaded_files
                .insert(item_id, items[i].clone());

            let caption = &items[i].file.name;

            let architecture = &items[i].model.architecture;
            let architecture_visible = !architecture.trim().is_empty();

            let param_size = &items[i].model.size;
            let param_size_visible = !param_size.trim().is_empty();

            let size = format_model_size(&items[i].file.size).unwrap_or("".to_string());
            let size_visible = !size.trim().is_empty();

            item_widget.apply_over(
                cx,
                live! {
                    label = { text: (caption) }
                    architecture_tag = { visible: (architecture_visible), caption = { text: (architecture) } }
                    params_size_tag = { visible: (param_size_visible), caption = { text: (param_size) } }
                    file_size_tag = { visible: (size_visible), caption = { text: (size) } }
                },
            );

            let _ = item_widget.draw_all(cx, &mut Scope::empty());

            total_height += item_widget.as_view().area().rect(cx).size.y;
        }
        self.total_height = Some(total_height);
    }
}

impl ModelSelectorListRef {
    pub fn get_height(&self) -> f64 {
        let Some(inner) = self.borrow_mut() else {
            return 0.0;
        };
        inner.total_height.unwrap_or(0.0)
    }
}