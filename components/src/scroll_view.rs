use dioxus::{
    core::UiEvent,
    events::{MouseData, WheelData},
    prelude::*,
};
use freya_elements as dioxus_elements;
use freya_hooks::use_node;

// TODO(marc2332): Make this code readable.

#[allow(non_snake_case)]
pub fn ScrollView<'a>(cx: Scope<'a, ScrollViewProps<'a>>) -> Element {
    let hovering = use_state(&cx, || false);
    let clicking = use_state(&cx, || false);
    let scrolled_y = use_state(&cx, || 0);
    let (node_ref, size) = use_node(&cx);

    let onwheel = move |e: UiEvent<WheelData>| {
        let wheel_y = e.delta().strip_units().y;
        let new_y = (*scrolled_y.get() as f32) + (wheel_y as f32 * 20.0);
        if size.height >= size.inner_height {
            scrolled_y.with_mut(|y| *y = 0);
            return;
        }
        if new_y >= 0.0 && wheel_y > 0.0 {
            scrolled_y.with_mut(|y| *y = 0);
            return;
        }
        if new_y <= -(size.inner_height - size.height) && wheel_y < 0.0 {
            scrolled_y.with_mut(|y| *y = -(size.inner_height - size.height) as i32);
            return;
        }
        scrolled_y.with_mut(|y| *y = new_y as i32);
    };

    let container_width = cx.props.width.unwrap_or("100%");
    let container_height = cx.props.height.unwrap_or("100%");

    let scrollbar_is_visible = if cx.props.show_scrollbar.unwrap_or(false) {
        !(size.height >= size.inner_height)
    } else {
        false
    };

    // I am currently getting the size of the viewport *asynchronously* so then I can calculate the proper size for the scroll content minus the scrollbar width.
    // This could be avoided if I implemented something like a CSS's calc() function
    let width = if scrollbar_is_visible {
        "calc(100% - 13)"
    } else {
        "100%"
    };
    let padding = cx.props.padding.unwrap_or("0");

    let viewableRatio = size.height / size.inner_height;
    let mut scrollbar_height = size.height * viewableRatio;
    if size.height >= size.inner_height {
        scrollbar_height = size.inner_height;
    }
    let scroll_pos = (100.0 / size.inner_height) * -(*scrolled_y.get()) as f32;
    let scrollbar_y = (scroll_pos / 100.0) * size.height;

    let onmouseleave = |_: UiEvent<MouseData>| {
        if *clicking.get() == false {
            hovering.set(false);
        }
    };

    let onmouseover = move |e: UiEvent<MouseData>| {
        hovering.set(true);
        if *clicking.get() {
            let coordinates = e.coordinates().element();
            let cursor_y = coordinates.y - 11.0;
            let per = 100.0 / size.height * cursor_y as f32;
            let new_y = -(size.inner_height / 100.0 * per);
            if size.height >= size.inner_height {
                scrolled_y.with_mut(|y| *y = 0);
                return;
            }
            if new_y >= 0.0 {
                scrolled_y.with_mut(|y| *y = 0);
                return;
            }
            if new_y <= -(size.inner_height - size.height) {
                scrolled_y.with_mut(|y| *y = -(size.inner_height - size.height) as i32);
                return;
            }
            scrolled_y.with_mut(|y| *y = new_y as i32);
        }
    };

    let onmousedown = |_: UiEvent<MouseData>| {
        clicking.set(true);
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(false);
    };

    render!(
        rect {
            direction: "horizontal",
            width: "{container_width}",
            height: "{container_height}",
            onclick: onclick, // TODO(marc2332): mouseup would be better
            onmouseover: onmouseover,
            container {
                padding: "{padding}",
                width: "{width}",
                height: "100%",
                scroll_y: "{scrolled_y}",
                reference: node_ref,
                onwheel: onwheel,
                &cx.props.children
            }
            cx.render(if scrollbar_is_visible {
                rsx!{
                    container {
                        width: "13",
                        height: "100%",
                        scroll_y: "{scrollbar_y}",
                        onmouseleave: onmouseleave,
                        rect {
                            onmousedown: onmousedown,
                            width: "100%",
                            height: "{scrollbar_height}",
                            radius: "10",
                            background: "rgb(135, 135, 135)",
                        }
                    }
                }
            } else {
               rsx!{ container {} }
            })
        }
    )
}

#[derive(Props)]
pub struct ScrollViewProps<'a> {
    pub children: Element<'a>,
    #[props(optional)]
    pub height: Option<&'a str>,
    #[props(optional)]
    pub width: Option<&'a str>,
    #[props(optional)]
    pub padding: Option<&'a str>,
    #[props(optional)]
    pub show_scrollbar: Option<bool>,
}
