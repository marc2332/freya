use freya_core::prelude::*;
use torin::{
    prelude::{
        Alignment,
        Direction,
    },
    size::Size,
};

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

#[derive(Clone, PartialEq)]
pub struct ScrollBar {
    pub(crate) theme: Option<ScrollBarThemePartial>,
    pub clicking_scrollbar: State<Option<(Axis, f64)>>,
    pub axis: Axis,
    pub offset: f32,
    pub thumb: ScrollThumb,
}

impl RenderOwned for ScrollBar {
    fn render(self) -> impl IntoElement {
        let scrollbar_theme = get_theme!(&self.theme, scrollbar);

        let mut state = use_state(|| ScrollBarState::Idle);

        let (size, opacity) = match *state.read() {
            _ if self.clicking_scrollbar.read().is_some() => (16., 160),
            ScrollBarState::Idle => (5., 0),
            ScrollBarState::Hovering => (16., 160),
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
                Size::px(16.),
                0.,
                -16.,
                self.offset,
                0.,
                Size::fill(),
                Size::px(size),
            ),
            Axis::Y => (
                Size::px(16.),
                Size::fill(),
                -16.,
                0.,
                0.,
                self.offset,
                Size::px(size),
                Size::fill(),
            ),
        };

        let on_pointer_enter = move |_| {
            state.set(ScrollBarState::Hovering);
        };
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
                        Direction::horizontal()
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
    }
}
