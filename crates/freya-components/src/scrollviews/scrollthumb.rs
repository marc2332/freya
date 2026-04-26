use freya_core::prelude::*;
use torin::size::Size;

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
}

impl ComponentOwned for ScrollThumb {
    fn render(mut self) -> impl IntoElement {
        let scrollbar_theme = get_theme!(&self.theme, ScrollBarThemePreference, "scrollbar");
        let mut state = use_state(|| ScrollThumbState::Idle);

        let (width, height) = match self.axis {
            Axis::X => (Size::px(self.size), Size::fill()),
            Axis::Y => (Size::fill(), Size::px(self.size)),
        };
        let thumb_background = match *state.read() {
            _ if self.clicking_scrollbar.read().is_some() => {
                scrollbar_theme.active_thumb_background
            }
            ScrollThumbState::Idle => scrollbar_theme.thumb_background,
            ScrollThumbState::Hovering => scrollbar_theme.hover_thumb_background,
        };

        let on_pointer_over = move |_| state.set(ScrollThumbState::Hovering);
        let on_pointer_out = move |_| state.set(ScrollThumbState::Idle);

        rect()
            .width(width)
            .height(height)
            .padding(4.)
            .on_pointer_over(on_pointer_over)
            .on_pointer_out(on_pointer_out)
            .on_pointer_down(move |e: Event<PointerEventData>| {
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
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .background(thumb_background)
                    .corner_radius(8.),
            )
    }
}
