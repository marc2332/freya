use dioxus::prelude::*;
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        App {
            title: "My App",
            navbar:  cx.render(rsx!(
                Navbar {
                    title: "A navbar"
                }
            ))
            body: cx.render(rsx!(
                Card {
                    title: "Another title",
                    content: "Some content"
                }
                Card {
                    title: "Lalala",
                    content: "Wooow"
                }
                Card {
                    title: "Another title",
                    content: "Some content"
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
            height: "40",
            width: "stretch",
            background: "black",
            padding: "10",
            p {
                height: "auto",
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
}

#[allow(non_snake_case)]
fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            width: "stretch",
            height: "100",
            padding: "10",
            background: "yellow",
            div {
                width: "stretch",
                height: "stretch",
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
        }
    ))
}
