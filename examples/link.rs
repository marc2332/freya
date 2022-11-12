#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{
    dioxus_elements::{self, MouseEvent},
    *,
};

fn main() {
    launch(app);
}

#[derive(Props)]
struct LinkProps<'a> {
    children: Element<'a>,
    url: &'a str,
}

#[allow(non_snake_case)]
fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element {
    let is_hovering = use_state(&cx, || false);

    let onmouseover = |_: MouseEvent| {
        is_hovering.with_mut(|v| *v = true);
    };

    let onmouseleave = |_: MouseEvent| {
        is_hovering.with_mut(|v| *v = false);
    };

    let color = if *is_hovering.get() {
        "blue"
    } else {
        "inherit"
    };

    render!(
        rect {
            direction: "both",
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            color: "{color}",
            &cx.props.children
        }
        rect {
            height: "0",
            width: "0",
            if *is_hovering.get() {
                rsx!(
                    Tooltip {
                        url: cx.props.url
                    }
                )
            } else {
                rsx!(
                    rect {}
                )
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn Tooltip<'a>(cx: Scope<'a>, url: &'a str) -> Element {
    render!(
        rect {
            width: "170",
            height: "30",
            padding: "4",
            direction: "both",
            rect {
                width: "100%",
                height: "100%",
                shadow: "0 5 35 3 black",
                radius: "8",
                background: "rgb(230,230,230)",
                direction: "both",
                display: "center",
                label {
                    color: "rgb(25,25,25)",
                    "{url}"
                }
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    render!(
        container {
            width: "100%",
            height: "100%",
            color: "black",
            Link {
                url: "https://link1.com",
                label {
                    font_size: "25",
                    "link1"
                }
            }
            Link {
                url: "https://link2.com",
                label {
                    font_size: "25",
                    "link2"
                }
            }
            Link {
                url: "https://link3.com",
                label {
                    font_size: "25",
                    "link3"
                }
            }
        }
    )
}
