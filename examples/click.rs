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
            padding: "15",
            text {
                "{count}"
            }
            view {
                height: "300",
                width: "300",
                background: "blue",
                onclick: move |_| count += 10,
                text {
                    "Increase!"
                }
            }
        }
    ))
}
