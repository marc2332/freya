#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(App {
        navbar: rsx!(Navbar {
            title: "Top Navbar"
        }),
        body: rsx!(
            ScrollView {
                height: "calc(100% - 75 - 75)",
                show_scrollbar: true,
                Card {
                    title: "Card 0",
                    content: "Content 0",
                }
                ScrollView {
                    height: "200",
                    padding: "0 20",
                    Card {
                        title: "Card 1",
                        content: "Content 1",
                    }
                    Card {
                        title: "Card 2",
                        content: "Content 2",
                    }
                    Card {
                        title: "Card 3",
                        content: "Content 3",
                    }
                    Card {
                        title: "Card 4",
                        content: "Content 4",
                    }
                }
                Card {
                    title: "Card 5",
                    content: "Content 5",
                }
                Card {
                    title: "Card 6",
                    content: "Content 6",
                }
                Card {
                    title: "Card 7",
                    content: "Content 7",
                }
            }
            Navbar {
                title: "Bottom Bar"
            }
        )
    })
}

#[derive(Props, Clone, PartialEq)]
struct NavbarProps {
    title: String,
}

#[allow(non_snake_case)]
fn Navbar(NavbarProps { title }: NavbarProps) -> Element {
    rsx!(
        rect {
            overflow: "clip",
            height: "75",
            width: "100%",
            background: "rgb(20, 20, 20)",
            padding: "15",
            main_align: "center",
            cross_align: "center",
            label {
                "{title}"
            }
        }
    )
}

#[allow(dead_code)]
#[derive(Props, PartialEq, Clone)]
struct AppProps {
    body: Element,
    navbar: Element,
}

#[allow(non_snake_case)]
fn App(props: AppProps) -> Element {
    rsx!(
        rect {
            color: "white",
            width: "100%",
            height: "100%",
            rect {
                width: "100%",
                height: "100%",
                {props.navbar}
                {props.body}
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
fn Card(props: CardProps) -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "200",
            padding: "10",
            background: "rgb(35, 35, 35)",
            rect {
                width: "100%",
                height: "50%",
                padding: "5",
                corner_radius: "10",
                main_align: "center",
                label {
                    height: "auto",
                    "{props.title}"
                }
                label {
                    height: "auto",
                    "{props.content}"
                }
            }
            rect {
                width: "fill",
                height: "fill",
                main_align: "center",
                cross_align: "center",
                background: "rgb(228, 143, 69)",
                padding: "10",
                corner_radius: "12",
                Button {
                    label {
                        "Hello, World!"
                    }
                }
            }
        }
    )
}
