use freya_animation::prelude::{
    AnimNum,
    AnimatedValue,
    Ease,
    Function,
    OnChange,
    OnCreation,
    ReadAnimatedValue,
    use_animation_with_dependencies,
};
use freya_core::prelude::*;
use torin::{
    prelude::{
        Alignment,
        Direction,
        Gaps,
        Position,
    },
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
    scrollviews::{
        ScrollThumb,
        shared::{
            Axis,
            SCROLLBAR_MARGIN,
        },
    },
};

define_theme! {
    %[component]
    pub ScrollBar {
        %[fields]
        background: Color,
        thumb_background: Color,
        visible_thumb_background: Color,
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
    pub thumb_size: f32,
}

impl ComponentOwned for ScrollBar {
    fn render(self) -> impl IntoElement {
        let scrollbar_theme = get_theme!(&self.theme, ScrollBarThemePreference, "scrollbar");

        let mut state = use_state(|| ScrollBarState::Idle);

        let is_expanded =
            self.clicking_scrollbar.read().is_some() || *state.read() == ScrollBarState::Hovering;

        let animation = use_animation_with_dependencies(&is_expanded, |conf, is_expanded| {
            conf.on_creation(OnCreation::Finish);
            conf.on_change(OnChange::Rerun);

            let value = (
                AnimNum::new(5., 8.)
                    .time(207)
                    .function(Function::Expo)
                    .ease(Ease::Out),
                AnimNum::new(0., 220.)
                    .time(207)
                    .function(Function::Expo)
                    .ease(Ease::Out),
                AnimNum::new(0., SCROLLBAR_MARGIN)
                    .time(207)
                    .function(Function::Expo)
                    .ease(Ease::Out),
            );

            if *is_expanded {
                value
            } else {
                value.into_reversed()
            }
        });
        let (cross_size, opacity, cross_gap) = animation.get().value();

        let (
            width,
            height,
            offset_x,
            offset_y,
            bar_width,
            bar_height,
            bar_margin,
            thumb_width,
            thumb_height,
            thumb_offset_x,
            thumb_offset_y,
        ) = match self.axis {
            Axis::X => (
                self.size.clone(),
                Size::px(20.),
                0.,
                -20.,
                Size::fill(),
                Size::px(cross_size),
                Gaps::new(0., SCROLLBAR_MARGIN, cross_gap, SCROLLBAR_MARGIN),
                Size::Inner,
                Size::px(20.),
                self.offset + SCROLLBAR_MARGIN,
                0.,
            ),
            Axis::Y => (
                Size::px(20.),
                self.size.clone(),
                -20.,
                0.,
                Size::px(cross_size),
                Size::fill(),
                Gaps::new(SCROLLBAR_MARGIN, cross_gap, SCROLLBAR_MARGIN, 0.),
                Size::px(20.),
                Size::Inner,
                0.,
                self.offset + SCROLLBAR_MARGIN,
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
            .cursor(CursorIcon::Default)
            .on_pointer_down(|e: Event<PointerEventData>| {
                e.stop_propagation();
            })
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
                    .on_pointer_over(on_pointer_over)
                    .on_pointer_out(on_pointer_out)
                    .child(
                        rect()
                            .width(bar_width)
                            .height(bar_height)
                            .margin(bar_margin)
                            .background(scrollbar_theme.background.with_a(opacity as u8))
                            .corner_radius(8.)
                            .shadow(Shadow::new().blur(6.).color((
                                0,
                                0,
                                0,
                                (opacity * 0.28) as u8,
                            ))),
                    )
                    .child(
                        rect()
                            .position(Position::new_absolute())
                            .width(thumb_width)
                            .height(thumb_height)
                            .offset_x(thumb_offset_x)
                            .offset_y(thumb_offset_y)
                            .child(ScrollThumb {
                                theme: self.theme.clone(),
                                clicking_scrollbar: self.clicking_scrollbar,
                                axis: self.axis,
                                size: self.thumb_size,
                                cross_size,
                                cross_gap,
                                bar_hovered: is_expanded,
                            }),
                    ),
            )
    }
}
