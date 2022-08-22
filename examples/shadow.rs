use dioxus::{prelude::*, core::UiEvent, events::MouseData};
use elements_namespace as dioxus_elements;

use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {

    let shadow_size = use_state(&cx, || (30f32,));

    let onscroll = move |ev: UiEvent<MouseData>| {
        shadow_size.with_mut(|shadow_size| {
            let page = ev.coordinates().page();
            shadow_size.0 += (page.y as f32) * 7.0;
        })
    };

    cx.render(rsx!(
        view {
            height: "100%",
            width: "100%",
            padding: "125",
            onscroll: onscroll,
            view {
                shadow: "0 10 210 {shadow_size.0} red",
                height: "100%",
                width: "100%",
                background: "black",
                padding: "50",
                text {
                    layer: "1",
                    "Scroll!"
                }
            }
        }
    ))
}
