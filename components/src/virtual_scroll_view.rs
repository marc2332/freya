use dioxus::{
    core::UiEvent,
    events::{MouseData, WheelData},
    prelude::*,
};
use fermi::use_atom_ref;
use freya_elements as dioxus_elements;
use freya_hooks::use_node;

use crate::THEME;

#[derive(Props)]
pub struct VirtProps<'a, T> {
    length: i32,
    item_size: f32,
    builder: Box<dyn Fn((i32, i32, &'a Option<T>)) -> LazyNodes<'a, 'a>>,
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

const SCROLLBAR_SIZE: u8 = 15;

#[derive(PartialEq, Debug)]
enum Axes {
    X,
    Y,
}

// TODO(marc2332): Make this code readable.
#[allow(non_snake_case)]
pub fn VirtualScrollView<'a, T>(cx: Scope<'a, VirtProps<'a, T>>) -> Element {
    let clicking = use_state::<Option<Axes>>(&cx, || None);
    let scrolled_y = use_state(&cx, || 0);
    let scrolled_x = use_state(&cx, || 0);
    let virtual_scroll_y = use_state(&cx, || 0); // Can only be between o and the item size
    let (node_ref, size) = use_node(&cx);
    let theme = use_atom_ref(&cx, THEME);
    let scrollbar_theme = &theme.read().scrollbar;

    let user_container_width = cx.props.width.unwrap_or("100%");
    let user_container_height = cx.props.height.unwrap_or("100%");
    let user_direction = cx.props.direction.unwrap_or("vertical");
    let max_size = cx.props.item_size * cx.props.length as f32;

    let vertical_scrollbar_is_visible = if cx.props.show_scrollbar.unwrap_or(false) {
        size.height < max_size
    } else {
        false
    };

    let horizontal_scrollbar_is_visible = if cx.props.show_scrollbar.unwrap_or(false) {
        size.width < size.inner_width
    } else {
        false
    };

    let width = if vertical_scrollbar_is_visible {
        format!("calc(100% - {SCROLLBAR_SIZE})")
    } else {
        "100%".to_string()
    };
    let height = if horizontal_scrollbar_is_visible {
        format!("calc(100% - {SCROLLBAR_SIZE})")
    } else {
        "100%".to_string()
    };
    let padding = cx.props.padding.unwrap_or("0");

    let viewableRatioHeight = size.height / max_size;
    let scrollbar_height = if size.height >= max_size {
        max_size
    } else {
        size.height * viewableRatioHeight
    };
    let scroll_pos_y = (100.0 / max_size) * -(*scrolled_y.get()) as f32;
    let scrollbar_y = (scroll_pos_y / 100.0) * size.height;

    let viewableRatioWidth = size.width / size.inner_width;
    let scrollbar_width = if size.width >= size.inner_width {
        size.inner_width
    } else {
        size.width * viewableRatioWidth
    };
    let scroll_pos_x = (100.0 / size.inner_width) * -(*scrolled_x.get()) as f32;
    let scrollbar_x = (scroll_pos_x / 100.0) * size.width;

    let onwheel = move |e: UiEvent<WheelData>| {
        let wheel_y = e.delta().strip_units().y;
        let new_y = (*scrolled_y.get() as f32) + (wheel_y as f32 * 20.0);
        if size.height >= max_size {
            scrolled_y.with_mut(|y| *y = 0);
            return;
        }
        if new_y >= 0.0 && wheel_y > 0.0 {
            scrolled_y.with_mut(|y| *y = 0);
            return;
        }
        if new_y <= -(max_size - size.height) && wheel_y < 0.0 {
            scrolled_y.with_mut(|y| *y = -(max_size - size.height) as i32);
            return;
        }
        scrolled_y.with_mut(|y| *y = new_y as i32);
    };

    let onmouseover = move |e: UiEvent<MouseData>| {
        if *clicking.get() == Some(Axes::Y) {
            let coordinates = e.coordinates().element();
            let cursor_y = coordinates.y - 11.0;
            let per = 100.0 / size.height * cursor_y as f32;
            let new_y = -(max_size / 100.0 * per);
            if size.height >= max_size {
                scrolled_y.with_mut(|y| *y = 0);
                return;
            }
            if new_y >= 0.0 {
                scrolled_y.with_mut(|y| *y = 0);
                return;
            }
            if new_y <= -(max_size - size.height) {
                scrolled_y.with_mut(|y| *y = -(max_size - size.height) as i32);
                return;
            }
            scrolled_y.with_mut(|y| *y = new_y as i32);
        } else if *clicking.get() == Some(Axes::X) {
            let coordinates = e.coordinates().element();
            let cursor_x = coordinates.x - 11.0;
            let per = 100.0 / size.width * cursor_x as f32;
            let new_x = -(size.inner_width / 100.0 * per);
            if size.width >= size.inner_width {
                scrolled_x.with_mut(|x| *x = 0);
                return;
            }
            if new_x >= 0.0 {
                scrolled_x.with_mut(|x| *x = 0);
                return;
            }
            if new_x <= -(size.inner_width - size.width) {
                scrolled_x.with_mut(|x| *x = -(size.inner_width - size.width) as i32);
                return;
            }
            scrolled_x.with_mut(|x| *x = new_x as i32);
        }
    };

    let onmousedown_y = |_: UiEvent<MouseData>| {
        clicking.set(Some(Axes::Y));
    };

    let onmousedown_x = |_: UiEvent<MouseData>| {
        clicking.set(Some(Axes::X));
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(None);
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

    let render_index_start = (-*scrolled_y.get()) / cx.props.item_size as i32;
    let potentially_visible_length = (size.height / cx.props.item_size) as i32;
    let remaining_length = cx.props.length - render_index_start;

    let render_index_end = if remaining_length <= potentially_visible_length {
        cx.props.length
    } else {
        render_index_start + potentially_visible_length
    };

    let render_range = render_index_start..(render_index_end);

    let mut key_index = 0;
    let children = render_range
        .map(|i| {
            key_index += 1;
            (cx.props.builder)((key_index, i, &cx.props.builder_values))
        })
        .collect::<Vec<LazyNodes>>();

    render!(
        rect {
            direction: "horizontal",
            width: "{user_container_width}",
            height: "{user_container_height}",
            onclick: onclick, // TODO(marc2332): mouseup would be better
            onmouseover: onmouseover,
            rect {
                direction: "vertical",
                width: "{width}",
                height: "{height}",
                container {
                    padding: "{padding}",
                    height: "100%",
                    width: "100%",
                    direction: "{user_direction}",
                    scroll_y: "{virtual_scroll_y}",
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
                    }
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
