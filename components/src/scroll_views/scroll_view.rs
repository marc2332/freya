use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{keyboard::Key, KeyboardEvent, MouseEvent, WheelEvent};
use freya_hooks::{use_focus, use_node};

use crate::{
    get_container_size, get_corrected_scroll_position, get_scroll_position_from_cursor,
    get_scroll_position_from_wheel, get_scrollbar_pos_and_size, is_scrollbar_visible,
    manage_key_event, Axis, ScrollBar, ScrollThumb, SCROLLBAR_SIZE, SCROLL_SPEED_MULTIPLIER,
};

/// [`ScrollView`] component properties.
#[derive(Props)]
pub struct ScrollViewProps<'a> {
    /// Direction of the ScrollView, `vertical` or `horizontal`.
    #[props(optional)]
    pub direction: Option<&'a str>,
    /// Inner children for the ScrollView.
    pub children: Element<'a>,
    /// Height of the ScrollView.
    #[props(optional)]
    pub height: Option<&'a str>,
    /// Width of the ScrollView.
    #[props(optional)]
    pub width: Option<&'a str>,
    /// Padding of the ScrollView.
    #[props(optional)]
    pub padding: Option<&'a str>,
    /// Show the scrollbar, by default is hidden.
    #[props(optional)]
    pub show_scrollbar: Option<bool>,
}

/// `Scrollable` container.
///
/// # Props
/// See [`ScrollViewProps`](ScrollViewProps).
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     render!(
///         ScrollView {
///              height: "300",
///              width: "100%",
///              show_scrollbar: true,
///              rect {
///                 background: "blue",
///                 height: "500",
///                 width: "100%"
///              }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn ScrollView<'a>(cx: Scope<'a, ScrollViewProps<'a>>) -> Element {
    let clicking_scrollbar = use_ref::<Option<(Axis, f64)>>(cx, || None);
    let clicking_shift = use_ref(cx, || false);
    let clicking_alt = use_ref(cx, || false);
    let scrolled_y = use_ref(cx, || 0);
    let scrolled_x = use_ref(cx, || 0);
    let (node_ref, size) = use_node(cx);
    let focus = use_focus(cx);

    let padding = cx.props.padding.unwrap_or("0");
    let user_container_width = cx.props.width.unwrap_or("100%");
    let user_container_height = cx.props.height.unwrap_or("100%");
    let user_direction = cx.props.direction.unwrap_or("vertical");
    let show_scrollbar = cx.props.show_scrollbar.unwrap_or_default();

    let vertical_scrollbar_is_visible =
        is_scrollbar_visible(show_scrollbar, size.inner.height, size.area.height());
    let horizontal_scrollbar_is_visible =
        is_scrollbar_visible(show_scrollbar, size.inner.width, size.area.width());

    let container_width = get_container_size(vertical_scrollbar_is_visible);
    let container_height = get_container_size(horizontal_scrollbar_is_visible);

    let corrected_scrolled_y = get_corrected_scroll_position(
        size.inner.height,
        size.area.height(),
        *scrolled_y.read() as f32,
    );
    let corrected_scrolled_x = get_corrected_scroll_position(
        size.inner.width,
        size.area.width(),
        *scrolled_x.read() as f32,
    );

    let (scrollbar_y, scrollbar_height) =
        get_scrollbar_pos_and_size(size.inner.height, size.area.height(), corrected_scrolled_y);
    let (scrollbar_x, scrollbar_width) =
        get_scrollbar_pos_and_size(size.inner.width, size.area.width(), corrected_scrolled_x);

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
                size.inner.height,
                size.area.height(),
                *scrolled_y.read() as f32,
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
            size.inner.width,
            size.area.width(),
            *scrolled_x.read() as f32,
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

            let scroll_position = get_scroll_position_from_cursor(
                cursor_y as f32,
                size.inner.height,
                size.area.height(),
            );

            scrolled_y.with_mut(|y| *y = scroll_position);
        } else if let Some((Axis::X, x)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_x = coordinates.x - x - size.area.min_x() as f64;

            let scroll_position = get_scroll_position_from_cursor(
                cursor_x as f32,
                size.inner.width,
                size.area.width(),
            );

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

        match e.key {
            Key::Shift => {
                clicking_shift.set(true);
            }
            Key::Alt => {
                clicking_alt.set(true);
            }
            _ => {
                let x = *scrolled_x.read() as f32;
                let y = *scrolled_y.read() as f32;
                let inner_height = size.inner.height;
                let inner_width = size.inner.width;
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
        *clicking_scrollbar.write_silent() = Some((Axis::Y, coordinates.y));
    };

    // Mark the X axis scrollbar as the one being dragged
    let onmousedown_x = |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        *clicking_scrollbar.write_silent() = Some((Axis::X, coordinates.x));
    };

    // Unmark any scrollbar
    let onclick = |_: MouseEvent| {
        *clicking_scrollbar.write_silent() = None;
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
                    offset_y: "{corrected_scrolled_y}",
                    offset_x: "{corrected_scrolled_x}",
                    reference: node_ref,
                    onwheel: onwheel,
                    &cx.props.children
                }
                ScrollBar {
                    width: "100%",
                    height: "{horizontal_scrollbar_size}",
                    offset_x: "{scrollbar_x}",
                    ScrollThumb {
                        onmousedown: onmousedown_x,
                        width: "{scrollbar_width}",
                        height: "100%",
                    }
                }
            }
            ScrollBar {
                width: "{vertical_scrollbar_size}",
                height: "100%",
                offset_y: "{scrollbar_y}",
                ScrollThumb {
                    onmousedown: onmousedown_y,
                    width: "100%",
                    height: "{scrollbar_height}",
                }
            }
        }
    )
}
