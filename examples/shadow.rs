use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use elements_namespace as dioxus_elements;

use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut shadow_size = use_state(&cx, || 30f32);

    let onscroll = move |ev: UiEvent<MouseData>| {
        let page = ev.coordinates().page();
        shadow_size += (page.y as f32) * 7.0;
    };

    cx.render(rsx!(
        container {
            height: "100%",
            width: "100%",
            padding: "125",
            onscroll: onscroll,
            view {
                shadow: "0 10 210 {shadow_size} red",
                height: "100%",
                width: "100%",
                background: "black",
                padding: "50",
                text {
                    "Scroll!"
                }
            }
        }
    ))
}
