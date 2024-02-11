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
                theme: theme_with!(ScrollViewTheme {
                    height: "calc(100% - 75 - 75)".into(),
                }),
                show_scrollbar: true,
                Card {
                    title: "Card 0",
                    content: "Content 0",
                }
                ScrollView {
                    theme: theme_with!(ScrollViewTheme {
                        height: "200".into(),
                        padding: "0 20".into(),
                    }),
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
                {props.navbar},
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
            Area {

            }
        }
    )
}

#[allow(non_snake_case)]
fn Area() -> Element {
    let mut cursor_pos_over = use_signal(|| (0f64, 0f64));
    let mut cursor_pos_click = use_signal(|| (0f64, 0f64));

    let cursor_moved = move |e: MouseEvent| {
        let pos = e.get_screen_coordinates();
        cursor_pos_over.with_mut(|cursor_pos| {
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        });
    };

    let cursor_clicked = move |e: MouseEvent| {
        let pos = e.get_screen_coordinates();
        cursor_pos_click.with_mut(|cursor_pos| {
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        })
    };

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            background: "rgb(228, 143, 69)",
            padding: "10",
            corner_radius: "12",
            main_align: "center",
            cross_align: "center",
            onmouseover: cursor_moved,
            onclick: cursor_clicked,
            label {
                "Mouse is at [x: {cursor_pos_over.read().0}, y: {cursor_pos_over.read().1}] ",
            },
            label {
                "Mouse clicked at [x: {cursor_pos_click.read().0}, y: {cursor_pos_click.read().1}]"
            }
        }
    )
}
