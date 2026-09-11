use freya_core::prelude::*;
use torin::{
    prelude::{
        Alignment,
        Gaps,
    },
    size::Size,
};

use crate::{
    get_theme,
    scrollviews::{
        ScrollBarThemePartial,
        ScrollBarThemePreference,
        shared::Axis,
    },
};

enum ScrollThumbState {
    Idle,
    Hovering,
}

#[derive(Clone, PartialEq)]
pub struct ScrollThumb {
    pub(crate) theme: Option<ScrollBarThemePartial>,
    pub clicking_scrollbar: State<Option<(Axis, f64)>>,
    pub axis: Axis,
    pub size: f32,
    pub cross_size: f32,
    pub cross_gap: f32,
    pub bar_hovered: bool,
}

impl ComponentOwned for ScrollThumb {
    fn render(mut self) -> impl IntoElement {
        let scrollbar_theme = get_theme!(&self.theme, ScrollBarThemePreference, "scrollbar");
        let mut state = use_state(|| ScrollThumbState::Idle);

        let (width, height, pill_width, pill_height, padding, main_align, cross_align) =
            match self.axis {
                Axis::X => (
                    Size::px(self.size),
                    Size::fill(),
                    Size::fill(),
                    Size::px(self.cross_size),
                    Gaps::new(0., 0., self.cross_gap, 0.),
                    Alignment::end(),
                    Alignment::start(),
                ),
                Axis::Y => (
                    Size::fill(),
                    Size::px(self.size),
                    Size::px(self.cross_size),
                    Size::fill(),
                    Gaps::new(0., self.cross_gap, 0., 0.),
                    Alignment::start(),
                    Alignment::end(),
                ),
            };
        let thumb_background = match *state.read() {
            _ if self.clicking_scrollbar.read().is_some() => {
                scrollbar_theme.active_thumb_background
            }
            ScrollThumbState::Hovering => scrollbar_theme.hover_thumb_background,
            ScrollThumbState::Idle if self.bar_hovered => scrollbar_theme.visible_thumb_background,
            ScrollThumbState::Idle => scrollbar_theme.thumb_background,
        };

        let on_pointer_over = move |_| state.set(ScrollThumbState::Hovering);
        let on_pointer_out = move |_| state.set(ScrollThumbState::Idle);

        rect()
            .width(width)
            .height(height)
            .on_pointer_over(on_pointer_over)
            .on_pointer_out(on_pointer_out)
            .on_pointer_down(move |e: Event<PointerEventData>| {
                if !e.data().is_primary() {
                    return;
                }
                if self.axis == Axis::X {
                    self.clicking_scrollbar
                        .set(Some((self.axis, e.element_location().x)));
                } else {
                    self.clicking_scrollbar
                        .set(Some((self.axis, e.element_location().y)));
                }
            })
            .on_pointer_press(move |e: Event<PointerEventData>| {
                e.prevent_default();
                e.stop_propagation();
                self.clicking_scrollbar.set(None);
            })
            .padding(padding)
            .main_align(main_align)
            .cross_align(cross_align)
            .child(
                rect()
                    .width(pill_width)
                    .height(pill_height)
                    .background(thumb_background)
                    .corner_radius(8.),
            )
    }
}
