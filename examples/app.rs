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
                )
                navbar: render!(
                    Navbar {
                        title: render!(
                            label {
                                "Hello, Freya!"
                            }
                        )
                    }
                )
                Card {
                    title: "Hello World",
                    content: "whoooah some content"
                }
                Card {
                    title: "Hello World",
                    content: "whoooah some content"
                }
                Card {
                    title: "Hello World",
                    content: "whoooah some content"
                }
            }
        )
    });
}

#[derive(Props)]
struct FloatingButtonProps<'a> {
    children: Element<'a>
}

fn FloatingButton<'a>(cx: Scope<'a, FloatingButtonProps<'a>>) -> Element<'a> {
    render!(
        rect {
            height: "100%",
            width: "100%",
            display: "center",
            direction: "both",
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
    children: Element<'a>
}

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
                offset_y: "-90",
                layer: "-100",
                direction: "horizontal",
                rect {
                    width: "calc(100% - 100)",
                }
                cx.props.floating_button.as_ref()
            }
        }
    )
}

#[derive(Props)]
struct NavbarProps<'a> {
    title: Option<Element<'a>>
}

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