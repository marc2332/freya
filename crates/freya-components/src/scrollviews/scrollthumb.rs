use freya_core::prelude::*;
use torin::size::Size;
enum ScrollThumbState {
    Idle,
    Hovering,
}

use typed_builder::TypedBuilder;

use crate::{
    get_theme,
    scrollviews::shared::Axis,
    theming::component_themes::ScrollBarThemePartial,
};

#[derive(Clone, PartialEq, TypedBuilder)]
pub struct ScrollThumb {
    #[builder(default)]
    pub(crate) theme: Option<ScrollBarThemePartial>,
    clicking_scrollbar: State<Option<(Axis, f64)>>,
    axis: Axis,
    size: f32,
}

impl RenderOwned for ScrollThumb {
    fn render(mut self) -> Element {
        let scrollbar_theme = get_theme!(&self.theme, scrollbar);
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

        let on_pointer_enter = move |_| state.set(ScrollThumbState::Hovering);
        let on_pointer_leave = move |_| state.set(ScrollThumbState::Idle);

        rect()
            .width(width)
            .height(height)
            .padding(4.)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .on_pointer_down(move |e: Event<PointerEventData>| {
                if self.axis == Axis::X {
                    self.clicking_scrollbar
                        .set(Some((self.axis, e.element_location().x)));
                } else {
                    self.clicking_scrollbar
                        .set(Some((self.axis, e.element_location().y)));
                }
            })
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .background(thumb_background)
                    .corner_radius(8.),
            )
            .into()
    }
}

pub fn scrollthumb() -> ScrollThumbBuilder<((), (), (), ())> {
    ScrollThumb::builder()
}
