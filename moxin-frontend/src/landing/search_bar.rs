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
                    return self.color;
                } else {
                    return mix(self.color2, self.color, pow(norm_distance, 1.4));
                }
            }

            fn pixel(self) -> vec4 {
                return Pal::premul(self.get_color());
            }
        }

        title = <View> {
            width: Fit,
            height: Fit,
            <Label> {
                draw_text:{
                    text_style: <REGULAR_FONT>{font_size: 13},
                    color: #000
                }
                text: "Discover, download, and run local LLMs"
            }
        }

        input_container = <RoundedView> {
            width: 800,
            height: Fit,

            show_bg: true,
            draw_bg: {
                color: #fff
            }

            padding: {top: 3, bottom: 3, left: 20, right: 20}
            margin: {left: 30, right: 30}

            spacing: 4,
            align: {x: 0.0, y: 0.5},

            draw_bg: {
                radius: 9.0,
                border_color: #D0D5DD,
                border_width: 1.0,
            }

            <Icon> {
                draw_icon: {
                    svg_file: (ICON_SEARCH),
                    fn get_color(self) -> vec4 {
                        return #666;
                    }
                }
                icon_walk: {width: 17, height: 17}
            }

            input = <MoxinTextInput> {
                width: Fill,
                height: Fit,
                empty_message: "Search Model by Keyword"
            }
        }

        search_sorting = <View> {
            visible: false,
            width: 300,
            height: Fit,
            margin: {left: 30, right: 30},
            <Sorting> {}
        }

        animator: {
            search_bar = {
                default: expanded,
                collapsed = {
                    redraw: true,
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: { height: 100 }
                }
          