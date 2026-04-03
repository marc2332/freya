use freya_core::prelude::*;
use torin::{
    prelude::{
        Alignment,
        Direction,
        Position,
    },
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
    scrollviews::{
        ScrollThumb,
        shared::Axis,
    },
};

define_theme! {
    %[component]
    pub ScrollBar {
        %[fields]
        background: Color,
        thumb_background: Color,
        hover_thumb_background: Color,
        active_thumb_background: Color,
        size: f32,
    }
}

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
    pub size: Size,
    pub thumb: ScrollThumb,
}

impl ComponentOwned for ScrollBar {
    fn render(self) -> impl IntoElement {
        let scrollbar_theme = get_theme!(&self.theme, ScrollBarThemePreference, "scrollbar");

        let mut state = use_state(|| ScrollBarState::Idle);

        let (cross_size, cross_offset, opacity) = match *state.read() {
            _ if self.clicking_scrollbar.read().is_some() => (16., 0., 160),
            ScrollBarState::Idle => (12., 3., 0),
            ScrollBarState::Hovering => (16., 0., 160),
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
                self.size.clone(),
                Size::px(16.),
                0.,
                -16.,
                self.offset,
                cross_offset,
                self.size.clone(),
                Size::px(cross_size),
            ),
            Axis::Y => (
                Size::px(16.),
                self.size.clone(),
                -16.,
                0.,
                cross_offset,
                self.offset,
                Size::px(cross_size),
                self.size.clone(),
            ),
        };

        let on_pointer_over = move |_| {
            if !cfg!(target_os = "android") {
                state.set(ScrollBarState::Hovering);
            }
        };
        let on_pointer_out = move |_| {
            if !cfg!(target_os = "android") {
                state.set(ScrollBarState::Idle);
            }
        };

        rect()
            .position(Position::new_absolute())
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
                    .on_pointer_over(on_pointer_over)
                    .on_pointer_out(on_pointer_out)
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
