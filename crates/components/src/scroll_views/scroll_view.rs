use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{keyboard::Key, KeyboardEvent, MouseEvent, WheelEvent};
use freya_hooks::{use_applied_theme, use_focus, use_node, ScrollViewThemeWith};

use crate::{
    get_container_size, get_corrected_scroll_position, get_scroll_position_from_cursor,
    get_scroll_position_from_wheel, get_scrollbar_pos_and_size, is_scrollbar_visible,
    manage_key_event, Axis, ScrollBar, ScrollThumb, SCROLLBAR_SIZE, SCROLL_SPEED_MULTIPLIER,
};

/// [`ScrollView`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollViewProps {
    /// Theme override.
    pub theme: Option<ScrollViewThemeWith>,
    /// Inner children for the ScrollView.
    pub children: Element,
    /// Direction of the ScrollView, `vertical` or `horizontal`.
    #[props(default = "vertical".to_string(), into)]
    pub direction: String,
    /// Show the scrollbar, visible by default.
    #[props(default = true, into)]
    pub show_scrollbar: bool,
    /// Enable scrolling with arrow keys.
    #[props(default = true, into)]
    pub scroll_with_arrows: bool,
}

/// `ScrollView` component.
///
/// # Props
/// See [`ScrollViewProps`](ScrollViewProps).
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         ScrollView {
///              theme: theme_with!(ScrollViewTheme {
///                 width: "100%".into(),
///                 height: "300".into(),
///              }),
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
pub fn ScrollView(props: ScrollViewProps) -> Element {
    let mut clicking_scrollbar = use_signal::<Option<(Axis, f64)>>(|| None);
    let mut clicking_shift = use_signal(|| false);
    let mut clicking_alt = use_signal(|| false);
    let mut scrolled_y = use_signal(|| 0);
    let mut scrolled_x = use_signal(|| 0);
    let (node_ref, size) = use_node();
    let mut focus = use_focus();
    let theme = use_applied_theme!(&props.theme, scroll_view);

    let padding = &theme.padding;
    let user_container_width = &theme.width;
    let user_container_height = &theme.height;
    let user_direction = &props.direction;
    let show_scrollbar = props.show_scrollbar;
    let scroll_with_arrows = props.scroll_with_arrows;

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
                corrected_scrolled_y,
            );

            // Only scroll when there is still area to scroll
            if *scrolled_y.peek() != scroll_position_y {
                e.stop_propagation();
                *scrolled_y.write() = scroll_position_y;
            } else {
                return;
            }
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
            corrected_scrolled_x,
        );

        // Only scroll when there is still area to scroll
        if *scrolled_x.peek() != scroll_position_x {
            e.stop_propagation();
            *scrolled_x.write() = scroll_position_x;
        } else {
            return;
        }

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

            *scrolled_y.write() = scroll_position;
        } else if let Some((Axis::X, x)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_x = coordinates.x - x - size.area.min_x() as f64;

            let scroll_position = get_scroll_position_from_cursor(
                cursor_x as f32,
                size.inner.width,
                size.area.width(),
            );

            *scrolled_x.write() = scroll_position;
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

    let onkeyup = move |e: KeyboardEvent| {
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
    let onclick = move |_: MouseEvent| {
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

    rsx!(
        rect {
            role: "scrollView",
            overflow: "clip",
            direction: "horizontal",
            width: "{user_container_width}",
            height: "{user_container_height}",
            onglobalclick: onclick,
            onglobalmouseover: onmouseover,
            onkeydown,
            onkeyup,
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
                    {props.children}
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
                        height: "100%"
                    }
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
                    height: "{scrollbar_height}"
                }
            }
        }
    )
}
