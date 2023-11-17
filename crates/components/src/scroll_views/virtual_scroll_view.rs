use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{keyboard::Key, KeyboardEvent, MouseEvent, WheelEvent};
use freya_hooks::{use_focus, use_node};
use std::ops::Range;

use crate::{
    get_container_size, get_corrected_scroll_position, get_scroll_position_from_cursor,
    get_scroll_position_from_wheel, get_scrollbar_pos_and_size, is_scrollbar_visible,
    manage_key_event, Axis, ScrollBar, ScrollThumb, SCROLLBAR_SIZE, SCROLL_SPEED_MULTIPLIER,
};

type BuilderFunction<'a, T> = dyn Fn(
    (
        usize,
        usize,
        Scope<'a, VirtualScrollViewProps<'a, T>>,
        &'a Option<T>,
    ),
) -> LazyNodes<'a, 'a>;

/// [`VirtualScrollView`] component properties.
#[derive(Props)]
pub struct VirtualScrollViewProps<'a, T: 'a> {
    /// Quantity of items in the VirtualScrollView.
    length: usize,
    /// Size of the items, height for vertical direction and width for horizontal.
    item_size: f32,
    /// The item builder function.
    builder: Box<BuilderFunction<'a, T>>,
    /// Custom values to pass to the builder function.
    #[props(optional)]
    pub builder_values: Option<T>,
    /// Direction of the VirtualScrollView, `vertical` or `horizontal`.
    #[props(default = "vertical".to_string(), into)]
    pub direction: String,
    /// Height of the VirtualScrollView.
    #[props(default = "100%".to_string(), into)]
    pub height: String,
    /// Width of the VirtualScrollView.
    #[props(default = "100%".to_string(), into)]
    pub width: String,
    /// Padding of the VirtualScrollView.
    #[props(default = "0".to_string(), into)]
    pub padding: String,
    /// Show the scrollbar, visible by default.
    #[props(default = true, into)]
    pub show_scrollbar: bool,
    /// Enable scrolling with arrow keys.
    #[props(default = true, into)]
    pub scroll_with_arrows: bool,
}

fn get_render_range(
    viewport_size: f32,
    scroll_position: f32,
    item_size: f32,
    item_length: f32,
) -> Range<usize> {
    let render_index_start = (-scroll_position) / item_size;
    let potentially_visible_length = viewport_size / item_size;
    let remaining_length = item_length - render_index_start;

    let render_index_end = if remaining_length <= potentially_visible_length {
        item_length
    } else {
        render_index_start + potentially_visible_length
    };

    render_index_start as usize..(render_index_end as usize)
}

/// `VirtualScrollView` component.
///
/// # Props
/// See [`VirtualScrollViewProps`](VirtualScrollViewProps).
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     render!(
///         VirtualScrollView {
///             width: "100%",
///             height: "100%",
///             show_scrollbar: true,
///             length: 5,
///             item_size: 80.0,
///             builder_values: (),
///             direction: "vertical",
///             builder: Box::new(move |(k, i, _, _)| {
///                 rsx! {
///                     label {
///                         key: "{k}",
///                         height: "80",
///                         "Number {i}"
///                     }
///                 }
///             })
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn VirtualScrollView<'a, T>(cx: Scope<'a, VirtualScrollViewProps<'a, T>>) -> Element {
    let clicking_scrollbar = use_ref::<Option<(Axis, f64)>>(cx, || None);
    let clicking_shift = use_ref(cx, || false);
    let clicking_alt = use_ref(cx, || false);
    let scrolled_y = use_ref(cx, || 0);
    let scrolled_x = use_ref(cx, || 0);
    let (node_ref, size) = use_node(cx);
    let focus = use_focus(cx);

    let padding = &cx.props.padding;
    let user_container_width = &cx.props.width;
    let user_container_height = &cx.props.height;
    let user_direction = &cx.props.direction;
    let show_scrollbar = cx.props.show_scrollbar;
    let items_length = cx.props.length;
    let items_size = cx.props.item_size;
    let scroll_with_arrows = cx.props.scroll_with_arrows;

    let inner_size = items_size + (items_size * items_length as f32);

    let vertical_scrollbar_is_visible = user_direction != "horizontal"
        && is_scrollbar_visible(show_scrollbar, inner_size, size.area.height());
    let horizontal_scrollbar_is_visible = user_direction != "vertical"
        && is_scrollbar_visible(show_scrollbar, inner_size, size.area.width());

    let container_width = get_container_size(vertical_scrollbar_is_visible);
    let container_height = get_container_size(horizontal_scrollbar_is_visible);

    let corrected_scrolled_y =
        get_corrected_scroll_position(inner_size, size.area.height(), *scrolled_y.read() as f32);
    let corrected_scrolled_x =
        get_corrected_scroll_position(inner_size, size.area.width(), *scrolled_x.read() as f32);

    let (scrollbar_y, scrollbar_height) =
        get_scrollbar_pos_and_size(inner_size, size.area.height(), corrected_scrolled_y);
    let (scrollbar_x, scrollbar_width) =
        get_scrollbar_pos_and_size(inner_size, size.area.width(), corrected_scrolled_x);

    // Moves the Y axis when the user scrolls in the container
    let onwheel = move |e: WheelEvent| {
        let speed_multiplier = if *clicking_alt.read() {
            SCROLL_SPEED_MULTIPLIER
        } else {
            1.0
        };

        if !*clicking_shift.read() {
            let wheel_y = e.get_delta_y() as f32 * speed_multiplier;

            let scroll_position_y = get_scroll_position_from_wheel(
                wheel_y,
                inner_size,
                size.area.height(),
                corrected_scrolled_y,
            );

            scrolled_y.with_mut(|y| *y = scroll_position_y);
        }

        let wheel_x = if *clicking_shift.read() {
            e.get_delta_y() as f32
        } else {
            e.get_delta_x() as f32
        } * speed_multiplier;

        let scroll_position_x = get_scroll_position_from_wheel(
            wheel_x,
            inner_size,
            size.area.width(),
            corrected_scrolled_x,
        );

        scrolled_x.with_mut(|x| *x = scroll_position_x);

        focus.focus();
    };

    // Drag the scrollbars
    let onmouseover = move |e: MouseEvent| {
        let clicking_scrollbar = clicking_scrollbar.read();

        if let Some((Axis::Y, y)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_y = coordinates.y - y - size.area.min_y() as f64;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_y as f32, inner_size, size.area.height());

            scrolled_y.with_mut(|y| *y = scroll_position);
        } else if let Some((Axis::X, x)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_x = coordinates.x - x - size.area.min_x() as f64;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_x as f32, inner_size, size.area.width());

            scrolled_x.with_mut(|x| *x = scroll_position);
        }

        if clicking_scrollbar.is_some() {
            focus.focus();
        }
    };

    let onkeydown = move |e: KeyboardEvent| {
        if !focus.is_focused() {
            return;
        }

        match &e.key {
            Key::Shift => {
                clicking_shift.set(true);
            }
            Key::Alt => {
                clicking_alt.set(true);
            }
            k => {
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
                let inner_height = inner_size;
                let inner_width = inner_size;
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

    let onkeyup = |e: KeyboardEvent| {
        if e.key == Key::Shift {
            clicking_shift.set(false);
        } else if e.key == Key::Alt {
            clicking_alt.set(false);
        }
    };

    // Mark the Y axis scrollbar as the one being dragged
    let onmousedown_y = |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        *clicking_scrollbar.write() = Some((Axis::Y, coordinates.y));
    };

    // Mark the X axis scrollbar as the one being dragged
    let onmousedown_x = |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        *clicking_scrollbar.write() = Some((Axis::X, coordinates.x));
    };

    // Unmark any scrollbar
    let onclick = |_: MouseEvent| {
        if clicking_scrollbar.read().is_some() {
            *clicking_scrollbar.write() = None;
        }
    };

    let horizontal_scrollbar_size = if horizontal_scrollbar_is_visible {
        SCROLLBAR_SIZE
    } else {
        0
    };

    let vertical_scrollbar_size = if vertical_scrollbar_is_visible {
        SCROLLBAR_SIZE
    } else {
        0
    };

    let (viewport_size, scroll_position) = if user_direction == "vertical" {
        (size.area.height(), corrected_scrolled_y)
    } else {
        (size.area.width(), corrected_scrolled_x)
    };

    // Calculate from what to what items must be rendered
    let render_range = get_render_range(
        viewport_size,
        scroll_position,
        items_size,
        items_length as f32,
    );

    let children =
        render_range.map(|i| (cx.props.builder)((i + 1, i, cx, &cx.props.builder_values)));

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

    render!(
        rect {
            role: "scrollView",
            overflow: "clip",
            direction: "horizontal",
            width: "{user_container_width}",
            height: "{user_container_height}",
            onglobalclick: onclick, // TODO(marc2332): mouseup would be better
            onglobalmouseover: onmouseover,
            onkeydown: onkeydown,
            onkeyup: onkeyup,
            rect {
                direction: "vertical",
                width: "{container_width}",
                height: "{container_height}",
                rect {
                    overflow: "clip",
                    padding: "{padding}",
                    height: "100%",
                    width: "100%",
                    direction: "{user_direction}",
                    reference: node_ref,
                    onwheel: onwheel,
                    children
                }
                ScrollBar {
                    width: "100%",
                    height: "{horizontal_scrollbar_size}",
                    offset_x: "{scrollbar_x}",
                    clicking_scrollbar: is_scrolling_x,
                    ScrollThumb {
                        clicking_scrollbar: is_scrolling_x,
                        onmousedown: onmousedown_x,
                        width: "{scrollbar_width}",
                        height: "100%",
                    },
                }
            }
            ScrollBar {
                width: "{vertical_scrollbar_size}",
                height: "100%",
                offset_y: "{scrollbar_y}",
                clicking_scrollbar: is_scrolling_y,
                ScrollThumb {
                    clicking_scrollbar: is_scrolling_y,
                    onmousedown: onmousedown_y,
                    width: "100%",
                    height: "{scrollbar_height}",
                }
            }
        }
    )
}
