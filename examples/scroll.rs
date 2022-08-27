use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use elements_namespace as dioxus_elements;

use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {

    cx.render(rsx!(
        ScrollView {
            body: cx.render(rsx! {
                view {
                    height: "200",
                    width: "100%",
                    background: "red",
                    padding: "20",
                    view {
                        height: "100%",
                        width: "100%",
                        background: "blue",
                        text { "hi" }
                    }
                }
                view {
                    height: "200",
                    width: "100%",
                    background: "red",
                    text { "hi" }
                }
                view {
                    height: "200",
                    width: "100%",
                    background: "red",
                    text { "hi" }
                }
                view {
                    height: "200",
                    width: "100%",
                    background: "red",
                    text { "hi" }
                }
                view {
                    height: "200",
                    width: "100%",
                    background: "red",
                    text { "hi" }
                }
            })
        }
    ))
}


#[allow(non_snake_case)]
fn ScrollView<'a>(cx: Scope<'a, ScrollViewProps<'a>>) -> Element {
    let mut height = use_state(&cx, || 0);

    let onscroll = move |e: UiEvent<MouseData>| {
        let page = e.coordinates().page();
        if *height.get() >= 0 && page.y > 0.0 {
            return;
        }
        height += (page.y as i32) * 20;
    };

    cx.render(rsx!(
        container {
            width: "100%",
            height: "70%",
            padding: "100",
            scroll_y: "{height}",
            padding: "125",
            onscroll: onscroll,
            &cx.props.body
        }
    ))
}

#[derive(Props)]
struct ScrollViewProps<'a> {
    body: Element<'a>,
}