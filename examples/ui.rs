use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use trev::launch;
mod mouse;
use mouse::Area;

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
                    body:  cx.render(rsx!(
                        Card {
                            title: "Another title",
                            content: "Some content",
                            background: "yellow"
                        }
                        Card {
                            title: "Lalala",
                            content: "Wooow",
                            background: "green"
                        }
                        Card {
                            title: "Another title",
                            content: "Some content",
                            background: "red"
                        }
                        Card {
                            title: "Another title",
                            content: "Some content",
                            background: "yellow"
                        }
                        Card {
                            title: "Lalala",
                            content: "Wooow",
                            background: "blue"
                        }
                        Card {
                            title: "Another title",
                            content: "Some content",
                            background: "yellow"
                        }
                     ))
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
        div {
            height: "15%",
            width: "stretch",
            background: "black",
            padding: "30",
            p {
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
        div {
            width: "stretch",
            height: "stretch",
            &cx.props.navbar,
            div {
                width: "stretch",
                height: "stretch",
                &cx.props.body
            }
        }
    ))
}

#[derive(Props)]
struct CardProps<'a> {
    title: &'a str,
    content: &'a str,
    background: &'a str,
}

#[allow(non_snake_case)]
fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            width: "stretch",
            height: "200",
            padding: "10",
            background: "{cx.props.background}",
            div {
                width: "stretch",
                height: "50%",
                background: "gray",
                padding: "20",
                p {
                    height: "auto",
                    "{&cx.props.title}"
                }
                p {
                    height: "auto",
                    "{&cx.props.content}"
                }
            }
            Area {

            }
        }
    ))
}

#[derive(Props)]
struct ScrollViewProps<'a> {
    body: Element<'a>,
}

#[allow(non_snake_case)]
fn ScrollView<'a>(cx: Scope<'a, ScrollViewProps<'a>>) -> Element {
    let mut height = use_state(&cx, || 0);

    let onscroll = move |e: UiEvent<MouseData>| {
        let page = e.coordinates().page();
        height += (page.y as i32) * 10;
    };

    cx.render(rsx!(
        div {
            width: "100%",
            height: "70%",
            overflow: "{height}",
            onscroll: onscroll,
            &cx.props.body
        }
    ))
}
