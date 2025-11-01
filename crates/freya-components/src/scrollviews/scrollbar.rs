use freya_core::prelude::*;
use torin::{
    prelude::{
        Alignment,
        Direction,
    },
    size::Size,
};
use typed_builder::TypedBuilder;

use crate::{
    get_theme,
    scrollviews::{
        ScrollThumb,
        shared::Axis,
    },
    theming::component_themes::ScrollBarThemePartial,
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum ScrollBarState {
    Idle,
    Hovering,
}

#[derive(Clone, PartialEq, TypedBuilder)]
pub struct ScrollBar {
    #[builder(default)]
    pub(crate) theme: Option<ScrollBarThemePartial>,
    clicking_scrollbar: State<Option<(Axis, f64)>>,
    axis: Axis,
    offset: f32,
    thumb: ScrollThumb,
}

impl RenderOwned for ScrollBar {
    fn render(self) -> Element {
        let scrollbar_theme = get_theme!(&self.theme, scrollbar);

        let mut state = use_state(|| ScrollBarState::Idle);

        let (size, opacity) = match *state.read() {
            _ if self.clicking_scrollbar.read().is_some() => (15., 225),
            ScrollBarState::Idle => (5., 0),
            ScrollBarState::Hovering => (15., 225),
        };

        let (
            width,
            height,
            offset_x,
            offset_y,
            inner_offset_x,
            inner_offset_y,
            inner_width,
            inner_height,
        ) = match self.axis {
            Axis::X => (
                Size::fill(),
                Size::px(15.),
                0.,
                -15.,
                self.offset,
                0.,
                Size::fill(),
                Size::px(size),
            ),
            Axis::Y => (
                Size::px(15.),
                Size::fill(),
                -15.,
                0.,
                0.,
                self.offset,
                Size::px(size),
                Size::fill(),
            ),
        };

        let on_pointer_enter = move |_| state.set(ScrollBarState::Hovering);
        let on_pointer_leave = move |_| state.set(ScrollBarState::Idle);

        rect()
            .width(width)
            .height(height)
            .offset_x(offset_x)
            .offset_y(offset_y)
            .layer(999)
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .direction(if self.axis == Axis::Y {
                        Direction::vertical()
                    } else {
                        Direction::Horizontal
                    })
                    .cross_align(Alignment::end())
                    .background(scrollbar_theme.background.with_a(opacity))
                    .on_pointer_enter(on_pointer_enter)
                    .on_pointer_leave(on_pointer_leave)
                    .child(
                        rect()
                            .width(inner_width)
                            .height(inner_height)
                            .offset_x(inner_offset_x)
                            .offset_y(inner_offset_y)
                            .child(self.thumb),
                    ),
            )
            .into()
    }
}

pub fn scrollbar() -> ScrollBarBuilder<((), (), (), (), ())> {
    ScrollBar::builder()
}
