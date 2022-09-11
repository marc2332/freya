#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use components::ScrollView;
use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use elements_namespace as dioxus_elements;
use freya::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        App {
            title: "My App",
            navbar:  cx.render(rsx!(
                Navbar {
                    title: "Top navbar"
                }
            ))
            body: cx.render(rsx!(
                ScrollView {
                    height: "70%",
                    Card {
                        title: "Another title",
                        content: "Some content",
                    }
                    ScrollView {
                        height: "200",
                        padding: "40",
                        Card {
                            title: "Lalala",
                            content: "Wooow",
                        }
                        Card {
                            title: "Lalala",
                            content: "Wooow",
                        }
                        ScrollView {
                            height: "200",
                            padding: "40",
                            Card {
                                title: "Lalala",
                                content: "Wooow",
                            }
                            Card {
                                title: "Lalala",
                                content: "Wooow",
                            }
                            ScrollView {
                                height: "200",
                                padding: "40",
                                Card {
                                    title: "Lalala",
                                    content: "Wooow",
                                }
                                Card {
                                    title: "Lalala",
                                    content: "Wooow",
                                }
                                Card {
                                    title: "Lalala",
                                    content: "Wooow",
                                }
                            }
                        }
                    }
                    Card {
                        title: "Another title",
                        content: "Some content",
                    }
                    Card {
                        title: "Another title",
                        content: "Some content",
                    }
                    Card {
                        title: "Lalala",
                        content: "Wooow",
                    }
                    Card {
                        title: "Another title",
                        content: "Some content",
                    }
                }
                Navbar {
                    title: "Bottom bar"
                }
            ))
        }
    })
}

#[derive(Props)]
struct NavbarProps<'a> {
    title: &'a str,
}

#[allow(non_snake_case)]
fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element {
    cx.render(rsx!(
        rect {
            height: "15%",
            width: "stretch",
            background: "rgb(20, 20, 20)",
            padding: "30",
            label {
                "{&cx.props.title}"
            }
        }
    ))
}

#[allow(dead_code)]
#[derive(Props)]
struct AppProps<'a> {
    title: &'a str,
    body: Element<'a>,
    navbar: Element<'a>,
}

#[allow(non_snake_case)]
fn App<'a>(cx: Scope<'a, AppProps<'a>>) -> Element {
    cx.render(rsx!(
        rect {
            width: "stretch",
            height: "stretch",
            rect {
                width: "stretch",
                height: "stretch",
                &cx.props.navbar,
                &cx.props.body
            }
        }
    ))
}

#[derive(Props)]
struct CardProps<'a> {
    title: &'a str,
    content: &'a str,
}

#[allow(non_snake_case)]
fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    cx.render(rsx!(
        rect {
            width: "stretch",
            height: "200",
            padding: "20",
            background: "rgb(45, 45, 45)",
            rect {
                width: "stretch",
                height: "50%",
                padding: "10",
                radius: "10",
                label {
                    height: "auto",
                    "{&cx.props.title}"
                }
                label {
                    height: "auto",
                    "{&cx.props.content}"
                }
            }
            Area {

            }
        }
    ))
}

#[allow(non_snake_case)]
fn Area<'a>(cx: Scope<'a>) -> Element {
    let cursor_pos_over = use_state(&cx, || (0f64, 0f64));
    let cursor_pos_click = use_state(&cx, || (0f64, 0f64));

    let cursor_moved = |ev: UiEvent<MouseData>| {
        cursor_pos_over.with_mut(|cursor_pos| {
            let pos = ev.data.element_coordinates();
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        });
    };

    let cursor_clicked = |ev: UiEvent<MouseData>| {
        cursor_pos_click.with_mut(|cursor_pos| {
            let pos = ev.data.element_coordinates();
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        })
    };

    cx.render(rsx! {
        rect {
            height: "50%",
            width: "100%",
            background: "blue",
            padding: "20",
            radius: "10",
            onmouseover: cursor_moved,
            onclick: cursor_clicked,
            label {
                "Mouse is at [x: {cursor_pos_over.0}, y: {cursor_pos_over.1}] ",
            },
            label {
                "Mouse clicked at [x: {cursor_pos_click.0}, y: {cursor_pos_click.1}]"
            }
        }
    })
}
