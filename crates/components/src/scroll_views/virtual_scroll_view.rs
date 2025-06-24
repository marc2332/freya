#![allow(clippy::type_complexity)]

use std::ops::Range;

use dioxus::prelude::*;
use freya_elements::{
    self as dioxus_elements,
    events::{
        keyboard::Key,
        KeyboardEvent,
        MouseEvent,
        WheelEvent,
    },
};
use freya_hooks::{
    use_applied_theme,
    use_focus,
    use_node,
    ScrollBarThemeWith,
};

use crate::{
    get_container_sizes,
    get_corrected_scroll_position,
    get_scroll_position_from_cursor,
    get_scroll_position_from_wheel,
    get_scrollbar_pos_and_size,
    is_scrollbar_visible,
    manage_key_event,
    scroll_views::use_scroll_controller,
    Axis,
    ScrollBar,
    ScrollConfig,
    ScrollController,
    ScrollThumb,
    SCROLL_SPEED_MULTIPLIER,
};

/// Properties for the [`VirtualScrollView`] component.
#[derive(Props, Clone)]
pub struct VirtualScrollViewProps<
    Builder: 'static + Clone + Fn(usize, &Option<BuilderArgs>) -> Element,
    BuilderArgs: Clone + 'static + PartialEq = (),
> {
    /// Width of the VirtualScrollView container. Default to `fill`.
    #[props(default = "fill".into())]
    pub width: String,
    /// Height of the VirtualScrollView container. Default to `fill`.
    #[props(default = "fill".into())]
    pub height: String,
    /// Padding of the VirtualScrollView container.
    #[props(default = "0".to_string())]
    pub padding: String,
    /// Theme override for the scrollbars.
    pub scrollbar_theme: Option<ScrollBarThemeWith>,
    /// Quantity of items in the VirtualScrollView.
    pub length: usize,
    /// Size of the items, height for vertical direction and width for horizontal.
    pub item_size: f32,
    /// The item builder function.
    pub builder: Builder,
    /// The values for the item builder function.
    #[props(into)]
    pub builder_args: Option<BuilderArgs>,
    /// Direction of the VirtualScrollView, `vertical` or `horizontal`.
    #[props(default = "vertical".to_string(), into)]
    pub direction: String,
    /// Show the scrollbar, visible by default.
    #[props(default = true, into)]
    pub show_scrollbar: bool,
    /// Enable scrolling with arrow keys.
    #[props(default = true, into)]
    pub scroll_with_arrows: bool,
    /// Cache elements or not, changing `builder_args` will invalidate the cache if enabled.
    /// Default is `true`.
    #[props(default = true, into)]
    pub cache_elements: bool,
    /// Custom Scroll Controller for the Virtual ScrollView.
    pub scroll_controller: Option<ScrollController>,
    /// If `false` (default), wheel scroll with no shift will scroll vertically no matter the direction.
    /// If `true`, wheel scroll with no shift will scroll horizontally.
    #[props(default = false)]
    pub invert_scroll_wheel: bool,
}

impl<
        BuilderArgs: Clone + PartialEq,
        Builder: Clone + Fn(usize, &Option<BuilderArgs>) -> Element,
    > PartialEq for VirtualScrollViewProps<Builder, BuilderArgs>
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.padding == other.padding
            && self.length == other.length
            && self.item_size == other.item_size
            && self.direction == other.direction
            && self.show_scrollbar == other.show_scrollbar
            && self.scroll_with_arrows == other.scroll_with_arrows
            && self.builder_args == other.builder_args
            && self.scroll_controller == other.scroll_controller
            && self.invert_scroll_wheel == other.invert_scroll_wheel
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

/// One-direction scrollable area that dynamically builds and renders items based in their size and current available size,
/// this is intended for apps using large sets of data that need good performance.
///
/// Use cases: text editors, chats, etc.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(VirtualScrollView {
///         length: 35,
///         item_size: 20.0,
///         direction: "vertical",
///         builder: move |i, _other_args: &Option<()>| {
///             rsx! {
///                 label {
///                     key: "{i}",
///                     height: "20",
///                     "Number {i}"
///                 }
///             }
///         }
///     })
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           {app()}
/// #       }
/// #   )
/// # }, (250., 250.).into(), "./images/gallery_virtual_scroll_view.png");
/// ```
///
/// # With a Scroll Controller
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut scroll_controller = use_scroll_controller(|| ScrollConfig::default());
///
///     rsx!(VirtualScrollView {
///         scroll_controller,
///         length: 35,
///         item_size: 20.0,
///         direction: "vertical",
///         builder: move |i, _other_args: &Option<()>| {
///             rsx! {
///                 label {
///                     key: "{i}",
///                     height: "20",
///                     onclick: move |_| {
///                          scroll_controller.scroll_to(ScrollPosition::Start, ScrollDirection::Vertical);
///                     },
///                     "Number {i}"
///                 }
///             }
///         }
///     })
/// }
/// ```
///
/// # Preview
/// ![VirtualScrollView Preview][virtual_scroll_view]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("virtual_scroll_view", "images/gallery_virtual_scroll_view.png")
)]
#[allow(non_snake_case)]
pub fn VirtualScrollView<
    Builder: Clone + Fn(usize, &Option<BuilderArgs>) -> Element,
    BuilderArgs: Clone + PartialEq,
>(
    VirtualScrollViewProps {
        width,
        height,
        padding,
        scrollbar_theme,
        length,
        item_size,
        builder,
        builder_args,
        direction,
        show_scrollbar,
        scroll_with_arrows,
        cache_elements,
        scroll_controller,
        invert_scroll_wheel,
    }: VirtualScrollViewProps<Builder, BuilderArgs>,
) -> Element {
    let mut clicking_scrollbar = use_signal::<Option<(Axis, f64)>>(|| None);
    let mut clicking_shift = use_signal(|| false);
    let mut clicking_alt = use_signal(|| false);
    let mut scroll_controller =
        scroll_controller.unwrap_or_else(|| use_scroll_controller(ScrollConfig::default));
    let (mut scrolled_x, mut scrolled_y) = scroll_controller.into();
    let (node_ref, size) = use_node();
    let mut focus = use_focus();
    let applied_scrollbar_theme = use_applied_theme!(&scrollbar_theme, scroll_bar);

    let (inner_width, inner_height) = match direction.as_str() {
        "vertical" => (size.inner.width, item_size * length as f32),
        _ => (item_size * length as f32, size.inner.height),
    };

    scroll_controller.use_apply(inner_width, inner_height);

    let vertical_scrollbar_is_visible =
        is_scrollbar_visible(show_scrollbar, inner_height, size.area.height());
    let horizontal_scrollbar_is_visible =
        is_scrollbar_visible(show_scrollbar, inner_width, size.area.width());

    let (container_width, content_width) = get_container_sizes(&width);
    let (container_height, content_height) = get_container_sizes(&height);

    let corrected_scrolled_y =
        get_corrected_scroll_position(inner_height, size.area.height(), *scrolled_y.read() as f32);
    let corrected_scrolled_x =
        get_corrected_scroll_position(inner_width, size.area.width(), *scrolled_x.read() as f32);

    let (scrollbar_y, scrollbar_height) =
        get_scrollbar_pos_and_size(inner_height, size.area.height(), corrected_scrolled_y);
    let (scrollbar_x, scrollbar_width) =
        get_scrollbar_pos_and_size(inner_width, size.area.width(), corrected_scrolled_x);

    // Moves the Y axis when the user scrolls in the container
    let onwheel = move |e: WheelEvent| {
        let speed_multiplier = if *clicking_alt.peek() {
            SCROLL_SPEED_MULTIPLIER
        } else {
            1.0
        };

        let invert_direction = (clicking_shift() || invert_scroll_wheel)
            && (!clicking_shift() || !invert_scroll_wheel);

        let (x_movement, y_movement) = if invert_direction {
            (
                e.get_delta_y() as f32 * speed_multiplier,
                e.get_delta_x() as f32 * speed_multiplier,
            )
        } else {
            (
                e.get_delta_x() as f32 * speed_multiplier,
                e.get_delta_y() as f32 * speed_multiplier,
            )
        };

        let scroll_position_y = get_scroll_position_from_wheel(
            y_movement,
            inner_height,
            size.area.height(),
            corrected_scrolled_y,
        );

        // Only scroll when there is still area to scroll
        if *scrolled_y.peek() != scroll_position_y {
            e.stop_propagation();
            *scrolled_y.write() = scroll_position_y;
        }

        let scroll_position_x = get_scroll_position_from_wheel(
            x_movement,
            inner_width,
            size.area.width(),
            corrected_scrolled_x,
        );

        // Only scroll when there is still area to scroll
        if *scrolled_x.peek() != scroll_position_x {
            e.stop_propagation();
            *scrolled_x.write() = scroll_position_x;
        }
    };

    // Drag the scrollbars
    let oncaptureglobalmousemove = move |e: MouseEvent| {
        let clicking_scrollbar = clicking_scrollbar.peek();

        if let Some((Axis::Y, y)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_y = coordinates.y - y - size.area.min_y() as f64;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_y as f32, inner_height, size.area.height());

            *scrolled_y.write() = scroll_position;
        } else if let Some((Axis::X, x)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_x = coordinates.x - x - size.area.min_x() as f64;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_x as f32, inner_width, size.area.width());

            *scrolled_x.write() = scroll_position;
        }

        if clicking_scrollbar.is_some() {
            e.prevent_default();
            focus.request_focus();
        }
    };

    let onglobalkeydown = move |e: KeyboardEvent| {
        match &e.key {
            Key::Shift => {
                clicking_shift.set(true);
            }
            Key::Alt => {
                clicking_alt.set(true);
            }
            k => {
                if !focus.is_focused() {
                    return;
                }

                if !scroll_with_arrows
                    && (k == &Key::ArrowUp
                        || k == &Key::ArrowRight
                        || k == &Key::ArrowDown
                        || k == &Key::ArrowLeft)
                {
                    return;
                }

                let x = corrected_scrolled_x;
                let y = corrected_scrolled_y;
                let viewport_height = size.area.height();
                let viewport_width = size.area.width();

                let (x, y) = manage_key_event(
                    e,
                    (x, y),
                    inner_height,
                    inner_width,
                    viewport_height,
                    viewport_width,
                );

                scrolled_x.set(x as i32);
                scrolled_y.set(y as i32);
            }
        };
    };

    let onglobalkeyup = move |e: KeyboardEvent| {
        if e.key == Key::Shift {
            clicking_shift.set(false);
        } else if e.key == Key::Alt {
            clicking_alt.set(false);
        }
    };

    // Mark the Y axis scrollbar as the one being dragged
    let onmousedown_y = move |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        *clicking_scrollbar.write() = Some((Axis::Y, coordinates.y));
    };

    // Mark the X axis scrollbar as the one being dragged
    let onmousedown_x = move |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        *clicking_scrollbar.write() = Some((Axis::X, coordinates.x));
    };

    // Unmark any scrollbar
    let onglobalclick = move |_: MouseEvent| {
        if clicking_scrollbar.peek().is_some() {
            *clicking_scrollbar.write() = None;
        }
    };

    let (viewport_size, scroll_position) = if direction == "vertical" {
        (size.area.height(), corrected_scrolled_y)
    } else {
        (size.area.width(), corrected_scrolled_x)
    };

    // Calculate from what to what items must be rendered
    let render_range = get_render_range(viewport_size, scroll_position, item_size, length as f32);

    let children = if cache_elements {
        let children = use_memo(use_reactive(
            &(render_range, builder_args),
            move |(render_range, builder_args)| {
                render_range
                    .clone()
                    .map(|i| (builder)(i, &builder_args))
                    .collect::<Vec<Element>>()
            },
        ));
        rsx!({ children.read().iter() })
    } else {
        let children = render_range.map(|i| (builder)(i, &builder_args));
        rsx!({ children })
    };

    let is_scrolling_x = clicking_scrollbar
        .read()
        .as_ref()
        .map(|f| f.0 == Axis::X)
        .unwrap_or_default();
    let is_scrolling_y = clicking_scrollbar
        .read()
        .as_ref()
        .map(|f| f.0 == Axis::Y)
        .unwrap_or_default();

    let (offset_x, offset_y) = match direction.as_str() {
        "vertical" => {
            let offset_y_min = (-corrected_scrolled_y / item_size).floor() * item_size;
            let offset_y = -(-corrected_scrolled_y - offset_y_min);

            (corrected_scrolled_x, offset_y)
        }
        _ => {
            let offset_x_min = (-corrected_scrolled_x / item_size).floor() * item_size;
            let offset_x = -(-corrected_scrolled_x - offset_x_min);

            (offset_x, corrected_scrolled_y)
        }
    };
    let a11y_id = focus.attribute();

    rsx!(
        rect {
            a11y_role: "scroll-view",
            overflow: "clip",
            direction: "horizontal",
            width: "{width}",
            height: "{height}",
            onglobalclick,
            oncaptureglobalmousemove,
            onglobalkeydown,
            onglobalkeyup,
            a11y_id,
            rect {
                direction: "vertical",
                width: "{container_width}",
                height: "{container_height}",
                rect {
                    overflow: "clip",
                    padding: "{padding}",
                    height: "{content_height}",
                    width: "{content_width}",
                    direction: "{direction}",
                    offset_x: "{offset_x}",
                    offset_y: "{offset_y}",
                    reference: node_ref,
                    onwheel,
                    {children}
                }
                if show_scrollbar && horizontal_scrollbar_is_visible {
                    ScrollBar {
                        size: &applied_scrollbar_theme.size,
                        offset_x: scrollbar_x,
                        clicking_scrollbar: is_scrolling_x,
                        theme: scrollbar_theme.clone(),
                        ScrollThumb {
                            clicking_scrollbar: is_scrolling_x,
                            onmousedown: onmousedown_x,
                            width: "{scrollbar_width}",
                            height: "100%",
                            theme: scrollbar_theme.clone(),
                        }
                    }
                }

            }
            if show_scrollbar && vertical_scrollbar_is_visible {
                ScrollBar {
                    is_vertical: true,
                    size: &applied_scrollbar_theme.size,
                    offset_y: scrollbar_y,
                    clicking_scrollbar: is_scrolling_y,
                    theme: scrollbar_theme.clone(),
                    ScrollThumb {
                        clicking_scrollbar: is_scrolling_y,
                        onmousedown: onmousedown_y,
                        width: "100%",
                        height: "{scrollbar_height}",
                        theme: scrollbar_theme,
                    }
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn virtual_scroll_view_wheel() {
        fn virtual_scroll_view_wheel_app() -> Element {
            let values = use_signal(|| ["Hello, World!"].repeat(30));

            rsx!(VirtualScrollView {
                length: values.read().len(),
                item_size: 50.0,
                direction: "vertical",
                builder: move |index, _: &Option<()>| {
                    let value = values.read()[index];
                    rsx! {
                        label {
                            key: "{index}",
                            height: "50",
                            "{index} {value}"
                        }
                    }
                }
            })
        }

        let mut utils = launch_test(virtual_scroll_view_wheel_app);
        let root = utils.root();

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 11);

        // Check that visible items are from indexes 0 to 11, because 500 / 50 = 10 + 1 (for smooth scrolling) = 11.
        for (n, i) in (0..11).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }

        utils.push_event(TestEvent::Wheel {
            name: WheelEventName::Wheel,
            scroll: (0., -300.).into(),
            cursor: (5., 5.).into(),
        });

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 11);

        // It has scrolled 300 pixels, which equals to 6 items since because 300 / 50 = 6
        // So we must start checking from 6 to +10, 16 in this case because 6 + 10 = 16 + 1 (for smooths scrolling) = 17.
        for (n, i) in (6..17).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }
    }

    #[tokio::test]
    pub async fn virtual_scroll_view_scrollbar() {
        fn virtual_scroll_view_scrollar_app() -> Element {
            let values = use_signal(|| ["Hello, World!"].repeat(30));

            rsx!(VirtualScrollView {
                length: values.read().len(),
                item_size: 50.0,
                direction: "vertical",
                builder: move |index, _: &Option<()>| {
                    let value = values.read()[index];
                    rsx! {
                        label {
                            key: "{index}",
                            height: "50",
                            "{index} {value}"
                        }
                    }
                }
            })
        }

        let mut utils = launch_test(virtual_scroll_view_scrollar_app);
        let root = utils.root();

        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 11);

        // Check that visible items are from indexes 0 to 10, because 500 / 50 = 10 + 1 (for smooth scrolling) = 11.
        for (n, i) in (0..11).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }

        // Simulate the user dragging the scrollbar
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseMove,
            cursor: (495., 20.).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseDown,
            cursor: (495., 20.).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseMove,
            cursor: (495., 320.).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseUp,
            cursor: (495., 320.).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 11);

        // It has dragged the scrollbar 300 pixels
        for (n, i) in (18..29).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }

        // Scroll up with arrows
        for _ in 0..11 {
            utils.push_event(TestEvent::Keyboard {
                name: KeyboardEventName::KeyDown,
                key: Key::ArrowUp,
                code: Code::ArrowUp,
                modifiers: Modifiers::default(),
            });
            utils.wait_for_update().await;
        }

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 11);

        for (n, i) in (0..11).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }

        // Scroll to the bottom with arrows
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::End,
            code: Code::End,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 10);

        for (n, i) in (20..30).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }
    }
}
