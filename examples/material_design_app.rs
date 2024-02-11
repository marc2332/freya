#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(|| -> Element {
        rsx!(
            Scaffold {
                floating_button: rsx!(
                    FloatingButton {
                        label {
                            "+"
                        }
                    }
                ),
                navbar: rsx!(
                    Navbar {
                        title: rsx!(
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

#[derive(Props, Clone, PartialEq)]
struct FloatingButtonProps {
    children: Element,
}

#[allow(non_snake_case)]
fn FloatingButton(FloatingButtonProps { children }: FloatingButtonProps) -> Element {
    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            height: "50",
            width: "50",
            background: "rgb(104, 24, 245)",
            shadow: "0 0 15 3 rgb(0, 0, 0, 60)",
            padding: "15",
            color: "white",
            font_size: "22",
            corner_radius: "50",
            {children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
struct ScaffoldProps {
    navbar: Option<Element>,
    floating_button: Option<Element>,
    children: Element,
}

#[allow(non_snake_case)]
fn Scaffold(props: ScaffoldProps) -> Element {
    let height = if props.navbar.is_some() {
        "calc(100% - 50)"
    } else {
        "100%"
    };

    rsx!(
        rect {
            direction: "vertical",
            height: "100%",
            width: "100%",
            rect {
                width: "0",
                height: "0",
                rect {
                    width: "100v",
                    height: "100v",
                    rect {
                        layer: "-999",
                        position: "absolute",
                        position_bottom: "70",
                        position_right: "70",
                        {props.floating_button}
                    }

                }
            }
            {props.navbar},
            ScrollView {
                theme: theme_with!(ScrollViewTheme {
                    height: height.into(),
                    padding: "3 10 0 10".into(),
                }),
                {props.children}
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
struct NavbarProps {
    title: Option<Element>,
}

#[allow(non_snake_case)]
fn Navbar(NavbarProps { title }: NavbarProps) -> Element {
    rsx!(
        rect {
            height: "50",
            width: "100%",
            background: "rgb(104, 24, 245)",
            direction: "vertical",
            color: "white",
            main_align: "center",
            rect {
                width: "100%",
                direction: "horizontal",
                padding: "0 20",
                font_size: "20",
                {title}
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
struct CardProps {
    title: String,
    content: String,
}

#[allow(non_snake_case)]
fn Card(CardProps { title, content }: CardProps) -> Element {
    rsx!(
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
                "{title}"
            }
            label {
                "{content}"
            }
        }
    )
}
