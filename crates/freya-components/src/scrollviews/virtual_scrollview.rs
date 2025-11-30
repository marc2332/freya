use std::{
    ops::Range,
    time::Duration,
};

use freya_core::prelude::*;
use freya_sdk::timeout::use_timeout;
use torin::{
    prelude::Direction,
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

/// One-direction scrollable area that dynamically builds and renders items based in their size and current available size,
/// this is intended for apps using large sets of data that need good performance.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     rect().child(
///         VirtualScrollView::new(|i, _| {
///             rect()
///                 .key(i)
///                 .height(Size::px(25.))
///                 .padding(4.)
///                 .child(format!("Item {i}"))
///                 .into()
///         })
///         .length(300)
///         .item_size(25.),
///     )
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc_hook(|| {
/// #   rect().center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_virtual_scrollview.png", |t| {
/// #   t.move_cursor((125., 115.));
/// #   t.sync_and_update();
/// # });
/// ```
///
/// # Preview
/// ![VirtualScrollView Preview][virtual_scrollview]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("virtual_scrollview", "images/gallery_virtual_scrollview.png")
)]
#[derive(Clone)]
pub struct VirtualScrollView<D, B: Fn(usize, &D) -> Element> {
    builder: B,
    builder_data: D,
    item_size: f32,
    length: i32,
    width: Size,
    height: Size,
    show_scrollbar: bool,
    direction: Direction,
    scroll_with_arrows: bool,
    scroll_controller: Option<ScrollController>,
    invert_scroll_wheel: bool,
}

impl<D: PartialEq, B: Fn(usize, &D) -> Element> PartialEq for VirtualScrollView<D, B> {
    fn eq(&self, other: &Self) -> bool {
        self.builder_data == other.builder_data
            && self.item_size == other.item_size
            && self.length == other.length
            && self.width == other.width
            && self.height == other.height
            && self.show_scrollbar == other.show_scrollbar
            && self.direction == other.direction
            && self.scroll_with_arrows == other.scroll_with_arrows
            && self.scroll_controller == other.scroll_controller
            && self.invert_scroll_wheel == other.invert_scroll_wheel
    }
}

impl<B: Fn(usize, &()) -> Element> VirtualScrollView<(), B> {
    pub fn new(builder: B) -> Self {
        Self {
            builder,
            builder_data: (),
            item_size: 0.,
            length: 0,
            width: Size::fill(),
            height: Size::fill(),
            show_scrollbar: true,
            direction: Direction::Vertical,
            scroll_with_arrows: true,
            scroll_controller: None,
            invert_scroll_wheel: false,
        }
    }

    pub fn new_controlled(
        builder: B,
        scroll_controller: ScrollController,
    ) -> VirtualScrollView<(), B> {
        VirtualScrollView::<(), B> {
            builder,
            builder_data: (),
            item_size: 0.,
            length: 0,
            width: Size::fill(),
            height: Size::fill(),
            show_scrollbar: true,
            direction: Direction::Vertical,
            scroll_with_arrows: true,
            scroll_controller: Some(scroll_controller),
            invert_scroll_wheel: false,
        }
    }
}

impl<D, B: Fn(usize, &D) -> Element> VirtualScrollView<D, B> {
    pub fn new_with_data(builder_data: D, builder: B) -> Self {
        Self {
            builder,
            builder_data,
            item_size: 0.,
            length: 0,
            width: Size::fill(),
            height: Size::fill(),
            show_scrollbar: true,
            direction: Direction::Vertical,
            scroll_with_arrows: true,
            scroll_controller: None,
            invert_scroll_wheel: false,
        }
    }

    pub fn new_with_data_controlled(
        builder_data: D,
        builder: B,
        scroll_controller: ScrollController,
    ) -> Self {
        Self {
            builder,
            builder_data,
            item_size: 0.,
            length: 0,
            width: Size::fill(),
            height: Size::fill(),
            show_scrollbar: true,
            direction: Direction::Vertical,
            scroll_with_arrows: true,
            scroll_controller: Some(scroll_controller),
            invert_scroll_wheel: false,
        }
    }

    pub fn show_scrollbar(mut self, show_scrollbar: bool) -> Self {
        self.show_scrollbar = show_scrollbar;
        self
    }

    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn scroll_with_arrows(mut self, scroll_with_arrows: impl Into<bool>) -> Self {
        self.scroll_with_arrows = scroll_with_arrows.into();
        self
    }

    pub fn item_size(mut self, item_size: impl Into<f32>) -> Self {
        self.item_size = item_size.into();
        self
    }

    pub fn length(mut self, length: impl Into<i32>) -> Self {
        self.length = length.into();
        self
    }

    pub fn invert_scroll_wheel(mut self, invert_scroll_wheel: impl Into<bool>) -> Self {
        self.invert_scroll_wheel = invert_scroll_wheel.into();
        self
    }

    pub fn scroll_controller(
        mut self,
        scroll_controller: impl Into<Option<ScrollController>>,
    ) -> Self {
        self.scroll_controller = scroll_controller.into();
        self
    }
}

impl<D: 'static, B: Fn(usize, &D) -> Element + 'static> Render for VirtualScrollView<D, B> {
    fn render(self: &VirtualScrollView<D, B>) -> impl IntoElement {
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

        let (inner_width, inner_height) = match self.direction {
            Direction::Vertical => (
                size.read().inner_sizes.width,
                self.item_size * self.length as f32,
            ),
            Direction::Horizontal => (
                self.item_size * self.length as f32,
                size.read().inner_sizes.height,
            ),
        };

        scroll_controller.use_apply(inner_width, inner_height);

        let corrected_scrolled_x =
            get_corrected_scroll_position(inner_width, size.read().area.width(), scrolled_x as f32);

        let corrected_scrolled_y = get_corrected_scroll_position(
            inner_height,
            size.read().area.height(),
            scrolled_y as f32,
        );
        let horizontal_scrollbar_is_visible = !timeout.elapsed()
            && is_scrollbar_visible(self.show_scrollbar, inner_width, size.read().area.width());
        let vertical_scrollbar_is_visible = !timeout.elapsed()
            && is_scrollbar_visible(self.show_scrollbar, inner_height, size.read().area.height());

        let (scrollbar_x, scrollbar_width) =
            get_scrollbar_pos_and_size(inner_width, size.read().area.width(), corrected_scrolled_x);
        let (scrollbar_y, scrollbar_height) = get_scrollbar_pos_and_size(
            inner_height,
            size.read().area.height(),
            corrected_scrolled_y,
        );

        let (container_width, content_width) = get_container_sizes(self.width.clone());
        let (container_height, content_height) = get_container_sizes(self.height.clone());

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
                inner_height,
                size.read().area.height(),
                corrected_scrolled_y,
            );
            scroll_controller.scroll_to_y(scroll_position_y).then(|| {
                e.stop_propagation();
            });

            // Horizontal scroll
            let scroll_position_x = get_scroll_position_from_wheel(
                x_movement,
                inner_width,
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
                    inner_height,
                    size.read().area.height(),
                );

                scroll_controller.scroll_to_y(scroll_position);
            } else if let Some((Axis::X, x)) = *clicking_scrollbar {
                let coordinates = e.element_location;
                let cursor_x = coordinates.x - x - size.read().area.min_x() as f64;

                let scroll_position = get_scroll_position_from_cursor(
                    cursor_x as f32,
                    inner_width,
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
                && (e.key == Key::ArrowUp
                    || e.key == Key::ArrowRight
                    || e.key == Key::ArrowDown
                    || e.key == Key::ArrowLeft)
            {
                return;
            }
            let x = corrected_scrolled_x;
            let y = corrected_scrolled_y;
            let inner_height = inner_height;
            let inner_width = inner_width;
            let viewport_height = size.read().area.height();
            let viewport_width = size.read().area.width();
            if let Some((x, y)) = handle_key_event(
                &e.key,
                (x, y),
                inner_height,
                inner_width,
                viewport_height,
                viewport_width,
            ) {
                scroll_controller.scroll_to_x(x as i32);
                scroll_controller.scroll_to_y(y as i32);
                e.stop_propagation();
                timeout.reset();
            }
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            let data = e;
            if data.key == Key::Shift {
                pressing_shift.set(true);
            } else if data.key == Key::Alt {
                pressing_alt.set(true);
            }
        };

        let on_global_key_up = move |e: Event<KeyboardEventData>| {
            let data = e;
            if data.key == Key::Shift {
                pressing_shift.set(false);
            } else if data.key == Key::Alt {
                pressing_alt.set(false);
            }
        };

        let (viewport_size, scroll_position) = if self.direction == Direction::vertical() {
            (size.read().area.height(), corrected_scrolled_y)
        } else {
            (size.read().area.width(), corrected_scrolled_x)
        };

        let render_range = get_render_range(
            viewport_size,
            scroll_position,
            self.item_size,
            self.length as f32,
        );

        let children = render_range
            .clone()
            .map(|i| (self.builder)(i, &self.builder_data))
            .collect::<Vec<Element>>();

        let (offset_x, offset_y) = match self.direction {
            Direction::Vertical => {
                let offset_y_min =
                    (-corrected_scrolled_y / self.item_size).floor() * self.item_size;
                let offset_y = -(-corrected_scrolled_y - offset_y_min);

                (corrected_scrolled_x, offset_y)
            }
            Direction::Horizontal => {
                let offset_x_min =
                    (-corrected_scrolled_x / self.item_size).floor() * self.item_size;
                let offset_x = -(-corrected_scrolled_x - offset_x_min);

                (offset_x, corrected_scrolled_y)
            }
        };

        rect()
            .width(self.width.clone())
            .height(self.height.clone())
            .a11y_id(focus.a11y_id())
            .a11y_focusable(false)
            .a11y_role(AccessibilityRole::ScrollView)
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
                            .direction(self.direction)
                            .width(content_width)
                            .height(content_height)
                            .offset_x(offset_x)
                            .offset_y(offset_y)
                            .overflow(Overflow::Clip)
                            .on_sized(move |e: Event<SizedEventData>| {
                                size.set_if_modified(e.clone())
                            })
                            .children(children),
                    )
                    .maybe_child(vertical_scrollbar_is_visible.then_some({
                        ScrollBar {
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
                        }
                    })),
            )
            .maybe_child(horizontal_scrollbar_is_visible.then_some({
                ScrollBar {
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
                }
            }))
    }
}

fn get_render_range(
    viewport_size: f32,
    scroll_position: f32,
    item_size: f32,
    item_length: f32,
) -> Range<usize> {
    let render_index_start = (-scroll_position) / item_size;
    let potentially_visible_length = (viewport_size / item_size) + 1.0;
    let remaining_length = item_length - render_index_start;

    let render_index_end = if remaining_length <= potentially_visible_length {
        item_length
    } else {
        render_index_start + potentially_visible_length
    };

    render_index_start as usize..(render_index_end as usize)
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_core::prelude::Label;
    use freya_testing::prelude::*;

    #[test]
    pub fn virtual_scroll_view_wheel() {
        fn virtual_scroll_view_wheel_app() -> impl IntoElement {
            VirtualScrollView::new(|i, _| {
                label()
                    .key(i)
                    .height(Size::px(50.))
                    .text(format!("{i} Hello, World!"))
                    .into()
            })
            .length(30)
            .item_size(50.)
        }

        let mut test = launch_test(virtual_scroll_view_wheel_app);
        test.sync_and_update();
        let scrollview = test
            .find(|node, element| {
                Rect::try_downcast(element)
                    .filter(|rect| {
                        rect.accessibility.builder.role() == AccessibilityRole::ScrollView
                    })
                    .map(move |_| node)
            })
            .unwrap();
        let content = scrollview.children()[0].children()[0].children();

        assert_eq!(content.len(), 11);

        // Check that visible items are from indexes 0 to 10, because 500 / 50 = 10 + 1 (for smooth scrolling) = 11.
        for (n, i) in (0..11).enumerate() {
            let child = &content[n];
            assert_eq!(
                Label::try_downcast(&*child.element()).unwrap().text,
                format!("{i} Hello, World!").as_str()
            );
        }

        test.scroll((5., 5.), (0., -300.));

        let content = scrollview.children()[0].children()[0].children();
        assert_eq!(content.len(), 11);

        // It has scrolled 300 pixels, which equals to 6 items because 300 / 50 = 6
        // So we must start checking from 6 to +10, 16 in this case because 6 + 10 = 16 + 1 (for smooth scrolling) = 17.
        for (n, i) in (6..17).enumerate() {
            let child = &content[n];
            assert_eq!(
                Label::try_downcast(&*child.element()).unwrap().text,
                format!("{i} Hello, World!").as_str()
            );
        }
    }

    #[test]
    pub fn virtual_scroll_view_scrollbar() {
        fn virtual_scroll_view_scrollbar_app() -> impl IntoElement {
            VirtualScrollView::new(|i, _| {
                label()
                    .key(i)
                    .height(Size::px(50.))
                    .text(format!("{i} Hello, World!"))
                    .into()
            })
            .length(30)
            .item_size(50.)
        }

        let mut test = launch_test(virtual_scroll_view_scrollbar_app);
        test.sync_and_update();
        let scrollview = test
            .find(|node, element| {
                Rect::try_downcast(element)
                    .filter(|rect| {
                        rect.accessibility.builder.role() == AccessibilityRole::ScrollView
                    })
                    .map(move |_| node)
            })
            .unwrap();
        let content = scrollview.children()[0].children()[0].children();

        assert_eq!(content.len(), 11);

        // Check that visible items are from indexes 0 to 10, because 500 / 50 = 10 + 1 (for smooth scrolling) = 11.
        for (n, i) in (0..11).enumerate() {
            let child = &content[n];
            assert_eq!(
                Label::try_downcast(&*child.element()).unwrap().text,
                format!("{i} Hello, World!").as_str()
            );
        }

        // Simulate the user dragging the scrollbar
        test.move_cursor((495., 20.));
        test.sync_and_update();
        test.press_cursor((495., 20.));
        test.sync_and_update();
        test.move_cursor((495., 320.));
        test.sync_and_update();
        test.release_cursor((495., 320.));
        test.sync_and_update();

        let content = scrollview.children()[0].children()[0].children();
        assert_eq!(content.len(), 11);

        // It has dragged the scrollbar 300 pixels
        for (n, i) in (18..29).enumerate() {
            let child = &content[n];
            assert_eq!(
                Label::try_downcast(&*child.element()).unwrap().text,
                format!("{i} Hello, World!").as_str()
            );
        }

        // Scroll up with arrows
        for _ in 0..11 {
            test.press_key(Key::ArrowUp);
        }

        let content = scrollview.children()[0].children()[0].children();
        assert_eq!(content.len(), 11);

        for (n, i) in (0..11).enumerate() {
            let child = &content[n];
            assert_eq!(
                Label::try_downcast(&*child.element()).unwrap().text,
                format!("{i} Hello, World!").as_str()
            );
        }

        // Scroll to the bottom with arrows
        test.press_key(Key::End);

        let content = scrollview.children()[0].children()[0].children();
        assert_eq!(content.len(), 10);

        for (n, i) in (20..30).enumerate() {
            let child = &content[n];
            assert_eq!(
                Label::try_downcast(&*child.element()).unwrap().text,
                format!("{i} Hello, World!").as_str()
            );
        }
    }
}
