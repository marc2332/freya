use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        view {
            height: "100%",
            width: "100%",
            background: "red",
            padding: "25",
            text {
                layer: "1",
                "{count}"
            }
            view {
                height: "30%",
                width: "100%",
                background: "blue",
                padding: "25",
                onclick: move |_| count += 10,
                layer: "1",
                text {
                    layer: "1",
                    "Increase!"
                }
            }
        }
    ))
}
