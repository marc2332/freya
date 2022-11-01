#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::{prelude::*, core::UiEvent, events::{WheelData, MouseData}};
use fermi::use_atom_ref;
use freya::{dioxus_elements, *};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        container {
            background: "rgb(15, 15, 15)",
            padding: "50",
            direction: "both",
            width: "auto",
            height: "auto",
            VirtualScrollView {
                length: 6000,
                item_size: 20.0,
                show_scrollbar: true,
                builder: Box::new(|(k, i)| {
                    rsx!{ 
                        label {
                            key: "{k}",
                            "Number {i}"
                        }
                    }
                })
            }
        }
    )
}

#[derive(Props)]
struct VirtProps<'a> {
    length: i32,
    item_size: f32,
    builder: Box<dyn Fn((i32, i32)) -> LazyNodes<'a, 'a>>,
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
fn VirtualScrollView<'a>(cx: Scope<'a, VirtProps<'a>>) -> Element {
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
    let mut scrollbar_height = size.height * viewableRatioHeight;
    if size.height >= max_size {
        scrollbar_height = max_size;
    }
    let scroll_pos_y = (100.0 / max_size) * -(*scrolled_y.get()) as f32;
    let scrollbar_y = (scroll_pos_y / 100.0) * size.height;

    let viewableRatioWidth = size.width / size.inner_width;
    let mut scrollbar_width = size.width * viewableRatioWidth;
    if size.width >= size.inner_width {
        scrollbar_width = size.inner_width;
    }
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

    // For now instead of not rendering the scrollbars they will simply have a size of 0 because I have some issues.
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

    let render_index = (-*scrolled_y.get()) / cx.props.item_size  as i32;
    let how_many_items_can_i_render = (size.height / cx.props.item_size)  as i32;
    let range = render_index..(render_index+how_many_items_can_i_render);

    let mut k = 0;
    let children = range.map(|i|  {
        k += 1;
        (cx.props.builder)((k, i))
    }).collect::<Vec<LazyNodes>>();

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

