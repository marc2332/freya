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
use freya_sdk::timeout::Timeout;
use torin::{
    prelude::{
        Alignment,
        Direction,
        Gaps,
        Point2D,
        Position,
        Size2D,
    },
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
    scrollviews::{
        ScrollController,
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
        hover_thumb_background: Color,
        active_thumb_background: Color,
        thumb_cross_size: f32,
        expanded_thumb_cross_size: f32,
        opacity: f32,
        expanded_opacity: f32,
        cross_gap: f32,
        expanded_cross_gap: f32,
    }
}

/// Data passed to a scrollbar renderer.
#[derive(Clone, PartialEq)]
pub struct ScrollBarContext {
    pub axis: Axis,
    pub scroll_position: Point2D,
    pub viewport_size: Size2D,
    pub content_size: Size2D,
    pub scroll_controller: ScrollController,
    pub timeout: Timeout,
    pub clicking_scrollbar: State<Option<(Axis, f64)>>,
    pub thumb_events: ScrollBarThumbEvents,
    pub thumb_offset: f32,
    pub track_size: Size,
    pub thumb_length: f32,
}

/// Pointer event handlers for a custom scrollbar thumb.
#[derive(Clone, PartialEq)]
pub struct ScrollBarThumbEvents {
    pub on_pointer_down: EventHandler<Event<PointerEventData>>,
    pub on_pointer_press: EventHandler<Event<PointerEventData>>,
}

impl ScrollBarThumbEvents {
    pub(crate) fn new(axis: Axis, mut clicking_scrollbar: State<Option<(Axis, f64)>>) -> Self {
        let on_pointer_down = move |event: Event<PointerEventData>| {
            if !event.data().is_primary() {
                return;
            }
            let location = event.element_location();
            let pointer = if axis == Axis::X {
                location.x
            } else {
                location.y
            };
            clicking_scrollbar.set(Some((axis, pointer)));
        };
        let on_pointer_press = move |event: Event<PointerEventData>| {
            event.prevent_default();
            event.stop_propagation();
            clicking_scrollbar.set(None);
        };

        Self {
            on_pointer_down: on_pointer_down.into(),
            on_pointer_press: on_pointer_press.into(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ScrollBar {
    context: ScrollBarContext,
    theme: Option<ScrollBarThemePartial>,
    key: DiffKey,
}

impl ScrollBar {
    /// Creates the default scrollbar renderer.
    pub fn default_renderer() -> Callback<ScrollBarContext, Element> {
        (|context| Self::new(context).into()).into()
    }

    /// Creates a built-in scrollbar from renderer context data.
    pub fn new(context: ScrollBarContext) -> Self {
        Self {
            context,
            theme: None,
            key: DiffKey::None,
        }
    }

    /// Overrides built-in scrollbar theme values for this instance.
    pub fn theme(mut self, theme: ScrollBarThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl KeyExt for ScrollBar {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ComponentOwned for ScrollBar {
    fn render(self) -> impl IntoElement {
        let ScrollBar { context, theme, .. } = self;
        let scrollbar_theme = get_theme!(&theme, ScrollBarThemePreference, "scrollbar");
        let ScrollBarContext {
            axis,
            timeout,
            clicking_scrollbar,
            thumb_events,
            thumb_offset,
            track_size,
            thumb_length,
            ..
        } = context;
        let mut hovering = use_state(|| false);
        let is_hidden = timeout.elapsed() && clicking_scrollbar.read().is_none();
        let is_expanded = clicking_scrollbar.read().is_some() || *hovering.read();

        let animation = use_animation_with_dependencies(&is_expanded, move |conf, is_expanded| {
            conf.on_creation(OnCreation::Finish);
            conf.on_change(OnChange::Rerun);

            let expand = |from: f32, to: f32| {
                AnimNum::new(from, to)
                    .time(207)
                    .function(Function::Expo)
                    .ease(Ease::Out)
            };
            let value = (
                expand(
                    scrollbar_theme.thumb_cross_size,
                    scrollbar_theme.expanded_thumb_cross_size,
                ),
                expand(scrollbar_theme.opacity, scrollbar_theme.expanded_opacity),
                expand(
                    scrollbar_theme.cross_gap,
                    scrollbar_theme.expanded_cross_gap,
                ),
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
        ) = match axis {
            Axis::X => (
                track_size,
                Size::px(20.),
                0.,
                -20.,
                Size::fill(),
                Size::px(cross_size),
                Gaps::new(0., SCROLLBAR_MARGIN, cross_gap, SCROLLBAR_MARGIN),
                Size::Inner,
                Size::px(20.),
                thumb_offset + SCROLLBAR_MARGIN,
                0.,
            ),
            Axis::Y => (
                Size::px(20.),
                track_size,
                -20.,
                0.,
                Size::px(cross_size),
                Size::fill(),
                Gaps::new(SCROLLBAR_MARGIN, cross_gap, SCROLLBAR_MARGIN, 0.),
                Size::px(20.),
                Size::Inner,
                0.,
                thumb_offset + SCROLLBAR_MARGIN,
            ),
        };

        let on_pointer_over = move |_| {
            if !cfg!(target_os = "android") {
                hovering.set_if_modified(true);
            }
        };
        let on_pointer_out = move |_| {
            if !cfg!(target_os = "android") {
                hovering.set_if_modified(false);
            }
        };

        rect()
            .position(Position::new_absolute())
            .width(if is_hidden { Size::px(0.) } else { width })
            .height(if is_hidden { Size::px(0.) } else { height })
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
                    .direction(if axis == Axis::Y {
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
                                theme,
                                clicking_scrollbar,
                                thumb_events,
                                axis,
                                size: thumb_length,
                                cross_size,
                                cross_gap,
                            }),
                    ),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
