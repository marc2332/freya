use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use trev::launch;
mod mouse;
fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        ScrollView {
            body:  cx.render(rsx!(
                Card {

                }
                Card {

                }
                Card {

                }
                Card {

                }
                Card {

                }
                Card {

                }
             ))
        }
        ScrollView {
            body:  cx.render(rsx!(
                Card {

                }
                Card {

                }
                Card {

                }
                Card {

                }
                Card {

                }
                Card {

                }
             ))
        }
    })
}

#[derive(PartialEq, Props)]
struct CardProps {}

#[allow(non_snake_case)]
fn Card<'a>(cx: Scope<'a, CardProps>) -> Element<'a> {
    cx.render(rsx!(
        div {
            height: "35",
            width: "100%",
            background: "white",
            padding: "20",
            div {
                height: "100%",
                width: "100%",
                background: "blue",
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
            height: "50%",
            background: "black",
            overflow: "{height}",
            onscroll: onscroll,
            &cx.props.body
        }
    ))
}
