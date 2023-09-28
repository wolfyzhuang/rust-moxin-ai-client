pub mod download_item;
pub mod downloads;
pub mod landing_screen;
pub mod model_card;
pub mod model_files;
pub mod model_files_item;
pub mod model_files_list;
pub mod model_files_tags;
pub mod model_list;
pub mod search_bar;
pub mod search_loading;
pub mod shared;
pub mod sorting;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    shared::live_design(cx);
    model_files_tags::live_design(