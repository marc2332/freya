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
            background: "red",
            padding: "50",
            direction: "both",
            width: "auto",
            height: "auto",
            view {
                padding: "25",
                height: "100%",
                width: "100%",
                background: "blue",
                onclick: move |_| count += 10,
                text {
                    "Increase!"
                }
                text {
                    "{count}"
                }
            }
        }
    ))
}
