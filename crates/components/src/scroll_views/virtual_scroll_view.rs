#![allow(clippy::type_complexity)]

use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{keyboard::Key, KeyboardEvent, MouseEvent, WheelEvent};
use freya_hooks::{use_applied_theme, use_focus, use_node, ScrollViewThemeWith};
use std::ops::Range;

use crate::{
    get_container_size, get_corrected_scroll_position, get_scroll_position_from_cursor,
    get_scroll_position_from_wheel, get_scrollbar_pos_and_size, is_scrollbar_visible,
    manage_key_event, Axis, ScrollBar, ScrollThumb, SCROLLBAR_SIZE, SCROLL_SPEED_MULTIPLIER,
};

/// [`VirtualScrollView`] component properties.
#[derive(Props, Clone)]
pub struct VirtualScrollViewProps<
    Builder: 'static + Clone + Fn(usize, &Option<BuilderArgs>) -> Element,
    BuilderArgs: Clone + 'static + PartialEq = (),
> {
    /// Theme override.
    pub theme: Option<ScrollViewThemeWith>,
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
}

impl<
        BuilderArgs: Clone + PartialEq,
        Builder: Clone + Fn(usize, &Option<BuilderArgs>) -> Element,
    > PartialEq for VirtualScrollViewProps<Builder, BuilderArgs>
{
    fn eq(&self, other: &Self) -> bool {
        self.theme == other.theme
            && self.length == other.length
            && self.item_size == other.item_size
            && self.direction == other.direction
            && self.show_scrollbar == other.show_scrollbar
            && self.scroll_with_arrows == other.scroll_with_arrows
            && self.builder_args == other.builder_args
    }
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
/// # use std::rc::Rc;
/// fn app() -> Element {
///     rsx!(
///         VirtualScrollView {
///             show_scrollbar: true,
///             length: 5,
///             item_size: 80.0,
///             direction: "vertical",
///             builder: move |i, _other_args: &Option<()>| {
///                 rsx! {
///                     label {
///                         key: "{i}",
///                         height: "80",
///                         "Number {i}"
///                     }
///                 }
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn VirtualScrollView<
    Builder: Clone + Fn(usize, &Option<BuilderArgs>) -> Element,
    BuilderArgs: Clone + PartialEq,
>(
    props: VirtualScrollViewProps<Builder, BuilderArgs>,
) -> Element {
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
    let items_length = props.length;
    let items_size = props.item_size;
    let scroll_with_arrows = props.scroll_with_arrows;

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
        let speed_multiplier = if *clicking_alt.peek() {
            SCROLL_SPEED_MULTIPLIER
        } else {
            1.0
        };

        if !*clicking_shift.peek() {
            let wheel_y = e.get_delta_y() as f32 * speed_multiplier;

            let scroll_position_y = get_scroll_position_from_wheel(
                wheel_y,
                inner_size,
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

        let wheel_x = if *clicking_shift.peek() {
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

            let scroll_position =
                get_scroll_position_from_cursor(cursor_y as f32, inner_size, size.area.height());

            *scrolled_y.write() = scroll_position;
        } else if let Some((Axis::X, x)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_x = coordinates.x - x - size.area.min_x() as f64;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_x as f32, inner_size, size.area.width());

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

    let children = use_memo_with_dependencies(
        (&render_range, &props.builder_args),
        move |(render_range, builder_args)| {
            render_range
                .map(|i| (props.builder)(i, &builder_args))
                .collect::<Vec<Element>>()
        },
    );

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
                    reference: node_ref,
                    onwheel: onwheel,
                    {children.read().iter()}
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
