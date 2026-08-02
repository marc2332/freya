use std::time::{
    Duration,
    Instant,
};

use freya_core::prelude::*;
use freya_sdk::timeout::use_timeout;
use torin::{
    geometry::CursorPoint,
    node::Node,
    prelude::{
        Direction,
        Length,
    },
    size::Size,
};

use crate::scrollviews::{
    ScrollBar,
    ScrollConfig,
    ScrollController,
    ScrollThumb,
    shared::{
        Axis,
        WheelGestureClock,
        accelerate_wheel_movement,
        get_container_sizes,
        get_corrected_scroll_position,
        get_scroll_position_from_cursor,
        get_scroll_position_from_wheel,
        get_scrollbar_pos_and_size,
        handle_key_event,
        is_scrollable,
        is_scrollbar_visible,
    },
    use_scroll_controller,
};

/// Scrollable area with bidirectional support and scrollbars.
///
/// It renders all of its children and scrolls over them, which makes it a good fit for small or
/// medium amounts of content. For large data sets prefer
/// [`VirtualScrollView`](crate::scrollviews::VirtualScrollView), which only renders the visible
/// items. It scrolls vertically by default, use [`direction`](ScrollView::direction) for a
/// horizontal layout. To drive the scroll position from code, build it with
/// [`new_controlled`](ScrollView::new_controlled) and a [`ScrollController`].
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     ScrollView::new()
///         .child("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum laoreet tristique diam, ut gravida enim. Phasellus viverra vitae risus sit amet iaculis. Morbi porttitor quis nisl eu vulputate. Etiam vitae ligula a purus suscipit iaculis non ac risus. Suspendisse potenti. Aenean orci massa, ornare ut elit id, tristique commodo dui. Vestibulum laoreet tristique diam, ut gravida enim. Phasellus viverra vitae risus sit amet iaculis. Vestibulum laoreet tristique diam, ut gravida enim. Phasellus viverra vitae risus sit amet iaculis. Vestibulum laoreet tristique diam, ut gravida enim. Phasellus viverra vitae risus sit amet iaculis.")
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # },
/// # "./images/gallery_scrollview.png")
/// #
/// # .with_hook(|t| {
/// #   t.move_cursor((125., 115.));
/// #   t.sync_and_update();
/// # });
/// ```
///
/// # Preview
/// ![ScrollView Preview][scrollview]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("scrollview", "images/gallery_scrollview.png")
)]
#[derive(Clone, PartialEq)]
pub struct ScrollView {
    children: Vec<Element>,
    layout: LayoutData,
    show_scrollbar: bool,
    scroll_with_arrows: bool,
    scroll_controller: Option<ScrollController>,
    invert_scroll_wheel: bool,
    drag_scrolling: bool,
    wheel_axis_lock: Option<f32>,
    contain_wheel: bool,
    latch_wheel: bool,
    key: DiffKey,
}

impl ChildrenExt for ScrollView {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for ScrollView {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        Self {
            children: Vec::default(),
            layout: Node {
                width: Size::fill(),
                height: Size::fill(),
                ..Default::default()
            }
            .into(),
            show_scrollbar: true,
            scroll_with_arrows: true,
            scroll_controller: None,
            invert_scroll_wheel: false,
            drag_scrolling: true,
            wheel_axis_lock: None,
            contain_wheel: false,
            latch_wheel: false,
            key: DiffKey::None,
        }
    }
}

impl ScrollView {
    /// Creates an uncontrolled scroll view that manages its own scroll position.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a scroll view driven by the given [`ScrollController`].
    pub fn new_controlled(scroll_controller: ScrollController) -> Self {
        Self {
            scroll_controller: Some(scroll_controller),
            ..Default::default()
        }
    }

    /// Toggles whether the scrollbars are shown when the content overflows.
    pub fn show_scrollbar(mut self, show_scrollbar: bool) -> Self {
        self.show_scrollbar = show_scrollbar;
        self
    }

    /// Sets the layout direction the children flow and scroll in.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout.direction = direction;
        self
    }

    /// Sets the gap between children along the scroll direction.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.layout.spacing = Length::new(spacing);
        self
    }

    /// Toggles whether the arrow keys scroll the view while it is focused.
    pub fn scroll_with_arrows(mut self, scroll_with_arrows: impl Into<bool>) -> Self {
        self.scroll_with_arrows = scroll_with_arrows.into();
        self
    }

    /// Inverts the direction of the mouse wheel relative to the content.
    pub fn invert_scroll_wheel(mut self, invert_scroll_wheel: impl Into<bool>) -> Self {
        self.invert_scroll_wheel = invert_scroll_wheel.into();
        self
    }

    /// Toggles scrolling by dragging the content, useful mainly for touch input.
    pub fn drag_scrolling(mut self, drag_scrolling: bool) -> Self {
        self.drag_scrolling = drag_scrolling;
        self
    }

    /// Locks wheel scrolling to the gesture's dominant axis: when one axis's delta exceeds the other
    /// by `threshold`×, the minor axis is suppressed. Stops a mostly-vertical (or -horizontal)
    /// wheel/trackpad gesture from drifting the cross axis — e.g. a horizontal outer scroll view that
    /// wraps a vertical inner one. `threshold` ≥ `1.0`; lower locks more aggressively (`1.0` ≈ always
    /// commit to the larger axis, like a spreadsheet). Off by default (both axes scroll freely).
    pub fn wheel_axis_lock(mut self, threshold: f32) -> Self {
        self.wheel_axis_lock = Some(threshold);
        self
    }

    /// Contains wheel scrolling to this scroll view: while the cursor is over it and its
    /// content overflows, wheel events never chain to an ancestor scrollable, not even once
    /// this view has hit the end of its range (by default the unconsumed remainder bubbles up
    /// and the ancestor takes over). A view whose content fits has nothing to scroll, so it
    /// stays transparent to the wheel. The CSS `overscroll-behavior: contain` analogue, for
    /// embedded scroll regions where the spill-over reads as a double scroll. See
    /// [`latch_wheel`](Self::latch_wheel) for the gesture-scoped alternative.
    pub fn contain_wheel(mut self) -> Self {
        self.contain_wheel = true;
        self
    }

    /// Latches wheel gestures to this scroll view, the macOS/AppKit trackpad convention. The
    /// scroll target is chosen at gesture start: if this view receives a gesture's first event
    /// and can move in its direction, the whole gesture stays here, including past the end of
    /// the range (no mid-gesture hand-off to an ancestor scrollable). Otherwise the whole
    /// gesture passes through untouched, so joining a gesture already in flight (the cursor
    /// drifting over this view mid-gesture) never steals it, and chaining only happens on a
    /// new gesture that starts with this view already at its end. Gestures are bounded by the
    /// shared wheel-gesture clock (events closer together than its window belong to the same
    /// gesture, momentum tails included); slow discrete mouse-wheel ticks fall outside the
    /// window and so collapse to plain per-tick chaining. The middle ground between default
    /// chaining and [`contain_wheel`](Self::contain_wheel)'s hard hover trap.
    pub fn latch_wheel(mut self) -> Self {
        self.latch_wheel = true;
        self
    }

    /// Caps the width of the scroll view.
    pub fn max_width(mut self, max_width: impl Into<Size>) -> Self {
        self.layout.maximum_width = max_width.into();
        self
    }

    /// Caps the height of the scroll view.
    pub fn max_height(mut self, max_height: impl Into<Size>) -> Self {
        self.layout.maximum_height = max_height.into();
        self
    }
}

impl LayoutExt for ScrollView {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for ScrollView {}
impl ContainerPositionExt for ScrollView {}

impl Component for ScrollView {
    fn render(self: &ScrollView) -> impl IntoElement {
        let a11y_id = use_a11y();
        let mut timeout = use_timeout(|| Duration::from_millis(800));
        let mut pressing_shift = use_state(|| false);
        let mut clicking_scrollbar = use_state::<Option<(Axis, f64)>>(|| None);
        let mut size = use_state(SizedEventData::default);
        let mut scroll_controller = self
            .scroll_controller
            .unwrap_or_else(|| use_scroll_controller(ScrollConfig::default));
        let mut dragging_content = use_state::<Option<CursorPoint>>(|| None);
        let mut drag_origin = use_state::<Option<CursorPoint>>(|| None);
        let (scrolled_x, scrolled_y) = scroll_controller.into();
        let layout = &self.layout.layout;
        let direction = layout.direction;
        let drag_scrolling = self.drag_scrolling;
        let wheel_axis_lock = self.wheel_axis_lock;
        let contain_wheel = self.contain_wheel;
        let latch_wheel = self.latch_wheel;
        let wheel_gesture_clock = WheelGestureClock::get();
        // This view's latch decision for a wheel gesture, keyed by the gesture's identity.
        // Only ever peeked, so handler writes never re-render the view per wheel tick.
        let mut latch = use_state(|| None::<(Instant, bool)>);

        scroll_controller.use_apply(
            size.read().inner_sizes.width,
            size.read().inner_sizes.height,
        );
        // Publish the viewport rectangle so `ScrollController::scroll_to_item` can reveal a target
        // from its own measured rectangle. `size.area` is the content box, which is fill-sized to the
        // viewport (its own offset scrolls its children, not itself), so it's the fixed visible frame.
        scroll_controller.set_viewport(size.read().area);

        let corrected_scrolled_x = get_corrected_scroll_position(
            size.read().inner_sizes.width,
            size.read().area.width(),
            scrolled_x as f32,
        );

        let corrected_scrolled_y = get_corrected_scroll_position(
            size.read().inner_sizes.height,
            size.read().area.height(),
            scrolled_y as f32,
        );
        let horizontal_scrollbar_is_visible = !timeout.elapsed()
            && is_scrollbar_visible(
                self.show_scrollbar,
                size.read().inner_sizes.width,
                size.read().area.width(),
            );
        let vertical_scrollbar_is_visible = !timeout.elapsed()
            && is_scrollbar_visible(
                self.show_scrollbar,
                size.read().inner_sizes.height,
                size.read().area.height(),
            );

        let (scrollbar_x, scrollbar_width) = get_scrollbar_pos_and_size(
            size.read().inner_sizes.width,
            size.read().area.width(),
            corrected_scrolled_x,
        );
        let (scrollbar_y, scrollbar_height) = get_scrollbar_pos_and_size(
            size.read().inner_sizes.height,
            size.read().area.height(),
            corrected_scrolled_y,
        );

        let (container_width, content_width) = get_container_sizes(layout.width.clone());
        let (container_height, content_height) = get_container_sizes(layout.height.clone());

        let scroll_with_arrows = self.scroll_with_arrows;
        let invert_scroll_wheel = self.invert_scroll_wheel;

        let on_capture_global_pointer_press = move |e: Event<PointerEventData>| {
            if clicking_scrollbar.read().is_some() {
                e.prevent_default();
                clicking_scrollbar.set(None);
            }

            if drag_scrolling && (dragging_content().is_some() || drag_origin().is_some()) {
                dragging_content.set(None);
                drag_origin.set(None);
            }
        };

        let on_wheel = move |e: Event<WheelEventData>| {
            // Advance the shared wheel-gesture clock, which both reads this event's acceleration
            // and keeps the clock honest for latching descendants.
            let gesture = wheel_gesture_clock.advance(e.timestamp, e.granularity);
            // Only invert direction on deviced-sourced wheel events
            let invert_direction = e.source == WheelSource::Device
                && (*pressing_shift.read() || invert_scroll_wheel)
                && (!*pressing_shift.read() || !invert_scroll_wheel);

            let (mut x_movement, mut y_movement) = if invert_direction {
                (e.delta_y as f32, e.delta_x as f32)
            } else {
                (e.delta_x as f32, e.delta_y as f32)
            };

            // Axis lock: keep a dominant-axis gesture from drifting the cross axis (e.g. a mostly
            // vertical trackpad scroll nudging a horizontal outer view sideways). When one axis's
            // delta exceeds the other by `threshold`×, zero the minor axis.
            if let Some(threshold) = wheel_axis_lock {
                let (ax, ay) = (x_movement.abs(), y_movement.abs());
                if ay > ax * threshold {
                    x_movement = 0.;
                } else if ax > ay * threshold {
                    y_movement = 0.;
                }
            }

            // Acceleration, so a long list can be crossed with the wheel. Applied before the
            // latch decision so the delta the decision is taken on is the delta that will move.
            (x_movement, y_movement) = accelerate_wheel_movement(
                (x_movement, y_movement),
                e.granularity,
                gesture,
                size.read().area,
            );

            // Gesture latching: a gesture belongs to a view that saw its FIRST event
            // (`gesture.start == e.timestamp`) and could move in its direction. Every view that
            // saw that first event is a candidate, and the innermost one able to move takes it
            // and stops propagation, so an outer latching view only gets the gesture when the
            // inner declined it. A view the gesture only reaches mid-flight is never a candidate
            // and passes the whole gesture through untouched. A latched gesture is consumed below
            // even once it pins at an end. Plain views still advance the shared clock so in-flight
            // gestures that start over them are recognisable.
            if latch_wheel {
                let decision = *latch.peek();
                let latched = match decision {
                    Some((owned, latched)) if owned == gesture.start => latched,
                    _ => {
                        let latched = gesture.start == e.timestamp && {
                            let s = size.read();
                            get_scroll_position_from_wheel(
                                y_movement,
                                s.inner_sizes.height,
                                s.area.height(),
                                corrected_scrolled_y,
                            ) != corrected_scrolled_y as i32
                                || get_scroll_position_from_wheel(
                                    x_movement,
                                    s.inner_sizes.width,
                                    s.area.width(),
                                    corrected_scrolled_x,
                                ) != corrected_scrolled_x as i32
                        };
                        latch.set(Some((gesture.start, latched)));
                        latched
                    }
                };
                if !latched {
                    return;
                }
            }

            // Vertical scroll
            let scroll_position_y = get_scroll_position_from_wheel(
                y_movement,
                size.read().inner_sizes.height,
                size.read().area.height(),
                corrected_scrolled_y,
            );
            scroll_controller.scroll_to_y(scroll_position_y).then(|| {
                e.stop_propagation();
            });

            // Horizontal scroll
            let scroll_position_x = get_scroll_position_from_wheel(
                x_movement,
                size.read().inner_sizes.width,
                size.read().area.width(),
                corrected_scrolled_x,
            );
            scroll_controller.scroll_to_x(scroll_position_x).then(|| {
                e.stop_propagation();
            });
            // A latched gesture owns the event end-to-end (reaching here means this gesture
            // bound to this view), so swallow it even once the position pins at an end.
            if latch_wheel {
                e.stop_propagation();
            }
            // Containment: swallow the event even when neither axis moved (position at an end)
            // so the leftover delta cannot chain to an ancestor scrollable, but only while the
            // content actually overflows: a view with nothing to scroll must stay transparent
            // to the wheel or it would dead-zone the ancestor under the cursor.
            else if contain_wheel {
                let size = size.read();
                let overflows = is_scrollable(size.inner_sizes.height, size.area.height())
                    || is_scrollable(size.inner_sizes.width, size.area.width());
                if overflows {
                    e.stop_propagation();
                }
            }
            timeout.reset();
        };

        let on_mouse_move = move |_| {
            timeout.reset();
        };

        let on_capture_global_pointer_move = move |e: Event<PointerEventData>| {
            if drag_scrolling {
                if let Some(prev) = dragging_content() {
                    let coords = e.global_location();
                    let delta = prev - coords;

                    scroll_controller.scroll_to_y((corrected_scrolled_y - delta.y as f32) as i32);
                    scroll_controller.scroll_to_x((corrected_scrolled_x - delta.x as f32) as i32);

                    dragging_content.set(Some(coords));
                    e.prevent_default();
                    timeout.reset();
                    a11y_id.request_focus();
                    return;
                } else if let Some(origin) = drag_origin() {
                    let coords = e.global_location();
                    let distance = (origin - coords).abs();

                    // Small threshold so taps can reach children (e.g. hover on buttons)
                    // without being immediately consumed by drag scrolling.
                    const DRAG_THRESHOLD: f64 = 2.0;

                    if distance.x > DRAG_THRESHOLD || distance.y > DRAG_THRESHOLD {
                        let delta = origin - coords;

                        scroll_controller
                            .scroll_to_y((corrected_scrolled_y - delta.y as f32) as i32);
                        scroll_controller
                            .scroll_to_x((corrected_scrolled_x - delta.x as f32) as i32);

                        dragging_content.set(Some(coords));
                        e.prevent_default();
                        timeout.reset();
                        a11y_id.request_focus();
                    }
                    return;
                }
            }

            let clicking_scrollbar = clicking_scrollbar.peek();

            if let Some((Axis::Y, y)) = *clicking_scrollbar {
                let coordinates = e.element_location();
                let cursor_y = coordinates.y - y - size.read().area.min_y() as f64;

                let scroll_position = get_scroll_position_from_cursor(
                    cursor_y as f32,
                    size.read().inner_sizes.height,
                    size.read().area.height(),
                );

                scroll_controller.scroll_to_y(scroll_position);
            } else if let Some((Axis::X, x)) = *clicking_scrollbar {
                let coordinates = e.element_location();
                let cursor_x = coordinates.x - x - size.read().area.min_x() as f64;

                let scroll_position = get_scroll_position_from_cursor(
                    cursor_x as f32,
                    size.read().inner_sizes.width,
                    size.read().area.width(),
                );

                scroll_controller.scroll_to_x(scroll_position);
            }

            if clicking_scrollbar.is_some() {
                e.prevent_default();
                timeout.reset();
                a11y_id.request_focus();
            }
        };

        let on_key_down = move |e: Event<KeyboardEventData>| {
            if !scroll_with_arrows
                && (e.key == Key::Named(NamedKey::ArrowUp)
                    || e.key == Key::Named(NamedKey::ArrowRight)
                    || e.key == Key::Named(NamedKey::ArrowDown)
                    || e.key == Key::Named(NamedKey::ArrowLeft))
            {
                return;
            }
            let x = corrected_scrolled_x;
            let y = corrected_scrolled_y;
            let inner_height = size.read().inner_sizes.height;
            let inner_width = size.read().inner_sizes.width;
            let viewport_height = size.read().area.height();
            let viewport_width = size.read().area.width();
            if let Some((x, y)) = handle_key_event(
                &e.key,
                (x, y),
                inner_height,
                inner_width,
                viewport_height,
                viewport_width,
                direction,
            ) {
                scroll_controller.scroll_to_x(x as i32);
                scroll_controller.scroll_to_y(y as i32);
                e.stop_propagation();
                timeout.reset();
            }
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            let data = e;
            if data.key == Key::Named(NamedKey::Shift) {
                pressing_shift.set(true);
            }
        };

        let on_global_key_up = move |e: Event<KeyboardEventData>| {
            let data = e;
            if data.key == Key::Named(NamedKey::Shift) {
                pressing_shift.set(false);
            }
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            if drag_scrolling && matches!(e.data(), PointerEventData::Touch(_)) {
                drag_origin.set(Some(e.global_location()));
            }
        };

        rect()
            .width(layout.width.clone())
            .height(layout.height.clone())
            .max_width(layout.maximum_width.clone())
            .max_height(layout.maximum_height.clone())
            .a11y_id(a11y_id)
            .a11y_focusable(false)
            .a11y_role(AccessibilityRole::ScrollView)
            .a11y_builder(move |node| {
                node.set_scroll_x(corrected_scrolled_x as f64);
                node.set_scroll_y(corrected_scrolled_y as f64)
            })
            .scrollable(true)
            .on_wheel(on_wheel)
            .on_capture_global_pointer_press(on_capture_global_pointer_press)
            .on_mouse_move(on_mouse_move)
            .on_capture_global_pointer_move(on_capture_global_pointer_move)
            .on_key_down(on_key_down)
            .on_global_key_up(on_global_key_up)
            .on_global_key_down(on_global_key_down)
            .on_pointer_down(on_pointer_down)
            .child(
                rect()
                    .width(container_width)
                    .height(container_height)
                    .horizontal()
                    .child(
                        rect()
                            .direction(direction)
                            .width(content_width)
                            .height(content_height)
                            .max_width(layout.maximum_width.clone())
                            .max_height(layout.maximum_height.clone())
                            .offset_x(corrected_scrolled_x)
                            .offset_y(corrected_scrolled_y)
                            .spacing(layout.spacing.get())
                            .overflow(Overflow::Clip)
                            .on_sized(move |e: Event<SizedEventData>| {
                                size.set_if_modified(e.clone())
                            })
                            .children(self.children.clone()),
                    )
                    .maybe_child(vertical_scrollbar_is_visible.then_some({
                        rect().child(ScrollBar {
                            theme: None,
                            clicking_scrollbar,
                            axis: Axis::Y,
                            offset: scrollbar_y,
                            size: Size::px(size.read().area.height()),
                            thumb: ScrollThumb {
                                theme: None,
                                clicking_scrollbar,
                                axis: Axis::Y,
                                size: scrollbar_height,
                            },
                        })
                    })),
            )
            .maybe_child(horizontal_scrollbar_is_visible.then_some({
                rect().child(ScrollBar {
                    theme: None,
                    clicking_scrollbar,
                    axis: Axis::X,
                    offset: scrollbar_x,
                    size: Size::px(size.read().area.width()),
                    thumb: ScrollThumb {
                        theme: None,
                        clicking_scrollbar,
                        axis: Axis::X,
                        size: scrollbar_width,
                    },
                })
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
