use std::time::Duration;

use freya_core::prelude::*;
use freya_sdk::timeout::use_timeout;
use torin::{
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
        get_container_sizes,
        get_corrected_scroll_position,
        get_scroll_position_from_cursor,
        get_scroll_position_from_wheel,
        get_scrollbar_pos_and_size,
        handle_key_event,
        is_scrollbar_visible,
    },
    use_scroll_controller,
};

/// Scrollable area with bidirectional support and scrollbars.
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
        let mut layout = LayoutData::default();
        layout.layout.width = Size::fill();
        layout.layout.height = Size::fill();
        Self {
            children: Vec::default(),
            layout,
            show_scrollbar: true,
            scroll_with_arrows: true,
            scroll_controller: None,
            invert_scroll_wheel: false,
            key: DiffKey::None,
        }
    }
}

impl ScrollView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_controlled(scroll_controller: ScrollController) -> Self {
        Self {
            children: Vec::default(),
            layout: LayoutData::default(),
            show_scrollbar: true,
            scroll_with_arrows: true,
            scroll_controller: Some(scroll_controller),
            invert_scroll_wheel: false,
            key: DiffKey::None,
        }
    }

    pub fn show_scrollbar(mut self, show_scrollbar: bool) -> Self {
        self.show_scrollbar = show_scrollbar;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout.layout.direction = direction;
        self
    }

    pub fn spacing(mut self, spacing: impl Into<f32>) -> Self {
        self.layout.layout.spacing = Length::new(spacing.into());
        self
    }

    pub fn scroll_with_arrows(mut self, scroll_with_arrows: impl Into<bool>) -> Self {
        self.scroll_with_arrows = scroll_with_arrows.into();
        self
    }

    pub fn invert_scroll_wheel(mut self, invert_scroll_wheel: impl Into<bool>) -> Self {
        self.invert_scroll_wheel = invert_scroll_wheel.into();
        self
    }
}

impl LayoutExt for ScrollView {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for ScrollView {}

impl Render for ScrollView {
    fn render(self: &ScrollView) -> impl IntoElement {
        let focus = use_focus();
        let mut timeout = use_timeout(|| Duration::from_millis(800));
        let mut pressing_shift = use_state(|| false);
        let mut pressing_alt = use_state(|| false);
        let mut clicking_scrollbar = use_state::<Option<(Axis, f64)>>(|| None);
        let mut size = use_state(SizedEventData::default);
        let mut scroll_controller = self
            .scroll_controller
            .unwrap_or_else(|| use_scroll_controller(ScrollConfig::default));
        let (scrolled_x, scrolled_y) = scroll_controller.into();
        let layout = &self.layout.layout;
        let direction = layout.direction;

        scroll_controller.use_apply(
            size.read().inner_sizes.width,
            size.read().inner_sizes.height,
        );

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

        let on_global_mouse_up = move |_| {
            clicking_scrollbar.set_if_modified(None);
        };

        let on_wheel = move |e: Event<WheelEventData>| {
            // Only invert direction on deviced-sourced wheel events
            let invert_direction = e.source == WheelSource::Device
                && (*pressing_shift.read() || invert_scroll_wheel)
                && (!*pressing_shift.read() || !invert_scroll_wheel);

            let (x_movement, y_movement) = if invert_direction {
                (e.delta_y as f32, e.delta_x as f32)
            } else {
                (e.delta_x as f32, e.delta_y as f32)
            };

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
            timeout.reset();
        };

        let on_mouse_move = move |_| {
            timeout.reset();
        };

        let on_capture_global_mouse_move = move |e: Event<MouseEventData>| {
            let clicking_scrollbar = clicking_scrollbar.peek();

            if let Some((Axis::Y, y)) = *clicking_scrollbar {
                let coordinates = e.element_location;
                let cursor_y = coordinates.y - y - size.read().area.min_y() as f64;

                let scroll_position = get_scroll_position_from_cursor(
                    cursor_y as f32,
                    size.read().inner_sizes.height,
                    size.read().area.height(),
                );

                scroll_controller.scroll_to_y(scroll_position);
            } else if let Some((Axis::X, x)) = *clicking_scrollbar {
                let coordinates = e.element_location;
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
                if !focus.is_focused() {
                    focus.request_focus();
                }
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
            } else if data.key == Key::Named(NamedKey::Alt) {
                pressing_alt.set(true);
            }
        };

        let on_global_key_up = move |e: Event<KeyboardEventData>| {
            let data = e;
            if data.key == Key::Named(NamedKey::Shift) {
                pressing_shift.set(false);
            } else if data.key == Key::Named(NamedKey::Alt) {
                pressing_alt.set(false);
            }
        };

        rect()
            .width(layout.width.clone())
            .height(layout.height.clone())
            .a11y_id(focus.a11y_id())
            .a11y_focusable(false)
            .a11y_role(AccessibilityRole::ScrollView)
            .a11y_builder(move |node| {
                node.set_scroll_x(corrected_scrolled_x as f64);
                node.set_scroll_y(corrected_scrolled_y as f64)
            })
            .scrollable(true)
            .on_wheel(on_wheel)
            .on_global_mouse_up(on_global_mouse_up)
            .on_mouse_move(on_mouse_move)
            .on_capture_global_mouse_move(on_capture_global_mouse_move)
            .on_key_down(on_key_down)
            .on_global_key_up(on_global_key_up)
            .on_global_key_down(on_global_key_down)
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
