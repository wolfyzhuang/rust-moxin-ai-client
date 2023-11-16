use crate::data::store::{SortCriteria, StoreAction};
use crate::landing::sorting::SortingWidgetExt;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import makepad_draw::shader::std::*;

    import crate::landing::sorting::Sorting;

    ICON_SEARCH = dep("crate://self/resources/icons/search.svg")

    SearchBar = {{SearchBar}} {
        width: Fill,
        height: 200,

        flow: Down,
        spacing: 30,
        align: {x: 0.5, y: 0.5},

        show_bg: true,

        draw_bg: {
            color: #EBFCFF,
            instance color2: #CBE3E8,
            fn get_color(self) -> vec4 {
                let coef = self.rect_size.y / self.rect_size.x;

                let distance_vec = self.pos - vec2(0.8, 1.1);
                let norm_distance = length(vec2(distance_vec.x, distance_vec.y * coef) * 2.2);

                if pow(norm_distance, 1.4) > 1.0 {
                    retur