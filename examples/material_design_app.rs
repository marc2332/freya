#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(|cx: Scope| -> Element {
        render!(
            Scaffold {
                floating_button: render!(
                    FloatingButton {
                        label {
                            "+"
                        }
                    }
                ),
                navbar: render!(
                    Navbar {
                        title: render!(
                            label {
                                "Hello, Freya!"
                            }
                        )
                    }
                ),
                Card {
                    title: "Rust",
                    content: "Rust is nice!"
                }
                Card {
                    title: "I like it a lot",
                    content: "A language empowering everyone to build reliable and efficient software. "
                }
                Card {
                    title: "Why Rust?",
                    content: "Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages. "
                }
            }
        )
    });
}

#[derive(Props)]
struct FloatingButtonProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
fn FloatingButton<'a>(cx: Scope<'a, FloatingButtonProps<'a>>) -> Element<'a> {
    render!(
        rect {
            height: "100%",
            width: "100%",
            main_alignment: "center",
            cross_alignment: "center",
            height: "50",
            width: "50",
            background: "rgb(104, 24, 245)",
            shadow: "0 0 15 3 rgb(0, 0, 0, 60)",
            padding: "15",
            color: "white",
            font_size: "22",
            corner_radius: "50",
            &cx.props.children
        }
    )
}

#[derive(Props)]
struct ScaffoldProps<'a> {
    navbar: Option<Element<'a>>,
    floating_button: Option<Element<'a>>,
    children: Element<'a>,
}

const FLOATING_BUTTON_BOTTOM_MARGIN: i32 = 85;
const FLOATING_BUTTON_RIGHT_MARGIN: i32 = 100;

#[allow(non_snake_case)]
fn Scaffold<'a>(cx: Scope<'a, ScaffoldProps<'a>>) -> Element<'a> {
    let height = if cx.props.navbar.is_some() {
        "calc(100% - 50)"
    } else {
        "100%"
    };

    render!(
        rect {
            direction: "vertical",
            height: "100%",
            width: "100%",
            cx.props.navbar.as_ref(),
            ScrollView {
                height: "{height}",
                width: "100%",
                padding: "3 10 0 10",
                &cx.props.children
            }
            rect {
                width: "100%",
                height: "0",
                offset_y: "-{FLOATING_BUTTON_BOTTOM_MARGIN}",
                layer: "-100",
                direction: "horizontal",
                rect {
                    width: "calc(100% - {FLOATING_BUTTON_RIGHT_MARGIN})",
                }
                cx.props.floating_button.as_ref()
            }
        }
    )
}

#[derive(Props)]
struct NavbarProps<'a> {
    title: Option<Element<'a>>,
}

#[allow(non_snake_case)]
fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element<'a> {
    render!(
        rect {
            height: "50",
            width: "100%",
            background: "rgb(104, 24, 245)",
            direction: "vertical",
            color: "white",
            display: "center",
            rect {
                width: "100%",
                direction: "horizontal",
                padding: "0 20",
                font_size: "20",
                cx.props.title.as_ref()
            }
        }
    )
}

#[derive(Props)]
struct CardProps<'a> {
    title: &'a str,
    content: &'a str,
}

#[allow(non_snake_case)]
fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    render!(
        rect {
            margin: "7 0",
            width: "100%",
            height: "200",
            background: "rgb(240, 240, 240)",
            shadow: "0 0 15 3 rgb(0, 0, 0, 60)",
            padding: "15",
            corner_radius: "8",
            label {
                font_size: "20",
                "{&cx.props.title}"
            }
            label {
                "{&cx.props.content}"
            }
        }
    )
}
