use std::ops::Range;

use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_elements::{MouseEvent, WheelEvent};
use freya_hooks::{use_get_theme, use_node};

use crate::{
    get_container_size, get_corrected_scroll_position, get_scroll_position_from_cursor,
    get_scroll_position_from_wheel, get_scrollbar_pos_and_size, is_scrollbar_visible, Axis,
    SCROLLBAR_SIZE,
};

type BuilderFunction<'a, T> = dyn Fn((i32, i32, &'a Option<T>)) -> LazyNodes<'a, 'a>;

/// Properties for the VirtualScrollView component.
#[derive(Props)]
pub struct VirtualScrollViewProps<'a, T: 'a> {
    length: i32,
    item_size: f32,
    builder: Box<BuilderFunction<'a, T>>,
    #[props(optional)]
    pub builder_values: Option<T>,
    #[props(optional)]
    pub direction: Option<&'a str>,
    #[props(optional)]
    pub height: Option<&'a str>,
    #[props(optional)]
    pub width: Option<&'a str>,
    #[props(optional)]
    pub padding: Option<&'a str>,
    #[props(optional)]
    pub show_scrollbar: Option<bool>,
}

fn get_render_range(
    viewport_size: f32,
    scroll_position: f32,
    item_size: f32,
    item_length: f32,
) -> Range<i32> {
    let render_index_start = (-scroll_position) / item_size;
    let potentially_visible_length = viewport_size / item_size;
    let remaining_length = item_length - render_index_start;

    let render_index_end = if remaining_length <= potentially_visible_length {
        item_length
    } else {
        render_index_start + potentially_visible_length
    };

    render_index_start as i32..(render_index_end as i32)
}

/// A ScrollView with virtual scrolling.
#[allow(non_snake_case)]
pub fn VirtualScrollView<'a, T>(cx: Scope<'a, VirtualScrollViewProps<'a, T>>) -> Element {
    let theme = use_get_theme(cx);
    let clicking_scrollbar = use_state::<Option<(Axis, f64)>>(cx, || None);
    let scrolled_y = use_state(cx, || 0);
    let scrolled_x = use_state(cx, || 0);
    let (node_ref, size) = use_node(cx);

    let scrollbar_theme = &theme.scrollbar;

    let padding = cx.props.padding.unwrap_or("0");
    let user_container_width = cx.props.width.unwrap_or("100%");
    let user_container_height = cx.props.height.unwrap_or("100%");
    let user_direction = cx.props.direction.unwrap_or("vertical");
    let show_scrollbar = cx.props.show_scrollbar.unwrap_or_default();
    let items_length = cx.props.length;
    let items_size = cx.props.item_size;

    let inner_size = items_size + (items_size * items_length as f32);

    let vertical_scrollbar_is_visible = user_direction != "horizontal"
        && is_scrollbar_visible(show_scrollbar, inner_size, size.height);
    let horizontal_scrollbar_is_visible = user_direction != "vertical"
        && is_scrollbar_visible(show_scrollbar, inner_size, size.width);

    let container_width = get_container_size(vertical_scrollbar_is_visible);
    let container_height = get_container_size(horizontal_scrollbar_is_visible);

    let corrected_scrolled_y =
        get_corrected_scroll_position(inner_size, size.height, *scrolled_y.get() as f32);
    let corrected_scrolled_x =
        get_corrected_scroll_position(inner_size, size.width, *scrolled_x.get() as f32);

    let (scrollbar_y, scrollbar_height) =
        get_scrollbar_pos_and_size(inner_size, size.height, corrected_scrolled_y);
    let (scrollbar_x, scrollbar_width) =
        get_scrollbar_pos_and_size(inner_size, size.width, corrected_scrolled_x);

    // Moves the Y axis when the user scrolls in the container
    let onwheel = move |e: WheelEvent| {
        let wheel_y = e.get_delta_y();

        let scroll_position = get_scroll_position_from_wheel(
            wheel_y as f32,
            inner_size,
            size.height,
            *scrolled_y.get() as f32,
        );

        scrolled_y.with_mut(|y| *y = scroll_position);
    };

    // Drag the scrollbars
    let onmouseover = move |e: MouseEvent| {
        if let Some((Axis::Y, y)) = clicking_scrollbar.get() {
            let coordinates = e.get_element_coordinates();
            let cursor_y = coordinates.y - y;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_y as f32, inner_size, size.height);

            scrolled_y.with_mut(|y| *y = scroll_position);
        } else if let Some((Axis::X, x)) = clicking_scrollbar.get() {
            let coordinates = e.get_element_coordinates();
            let cursor_x = coordinates.x - x;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_x as f32, inner_size, size.width);

            scrolled_x.with_mut(|x| *x = scroll_position);
        }
    };

    // Mark the Y axis scrollbar as the one being dragged
    let onmousedown_y = |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        clicking_scrollbar.set(Some((Axis::Y, coordinates.y)));
    };

    // Mark the X axis scrollbar as the one being dragged
    let onmousedown_x = |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        clicking_scrollbar.set(Some((Axis::X, coordinates.x)));
    };

    // Unmark any scrollbar
    let onclick = |_: MouseEvent| {
        clicking_scrollbar.set(None);
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
        (size.height, corrected_scrolled_y)
    } else {
        (size.width, corrected_scrolled_x)
    };

    // Calculate from what to what items must be rendered
    let render_range = get_render_range(
        viewport_size,
        scroll_position,
        items_size,
        items_length as f32,
    );

    let children = render_range.map(|i| (cx.props.builder)((i + 1, i, &cx.props.builder_values)));

    render!(
        container {
            direction: "horizontal",
            width: "{user_container_width}",
            height: "{user_container_height}",
            onclick: onclick, // TODO(marc2332): mouseup would be better
            onmouseover: onmouseover,
            rect {
                direction: "vertical",
                width: "{container_width}",
                height: "{container_height}",
                container {
                    padding: "{padding}",
                    height: "100%",
                    width: "100%",
                    direction: "{user_direction}",
                    reference: node_ref,
                    onwheel: onwheel,
                    children
                }
                container {
                    width: "100%",
                    height: "{horizontal_scrollbar_size}",
                    scroll_x: "{scrollbar_x}",
                    onmouseleave: |_| {},
                    background: "{scrollbar_theme.background}",
                    rect {
                        onmousedown: onmousedown_x,
                        width: "{scrollbar_width}",
                        height: "100%",
                        radius: "10",
                        background: "{scrollbar_theme.thumb_background}",
                    },
                }
            }
            container {
                width: "{vertical_scrollbar_size}",
                height: "100%",
                scroll_y: "{scrollbar_y}",
                onmouseleave: |_| {},
                background: "{scrollbar_theme.background}",
                rect {
                    onmousedown: onmousedown_y,
                    width: "100%",
                    height: "{scrollbar_height}",
                    radius: "10",
                    background: "{scrollbar_theme.thumb_background}",
                }
            }
        }
    )
}
