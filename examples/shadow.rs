use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        view {
            height: "100%",
            width: "100%",
            padding: "50",
            view {
                shadow: "0 5 200 50.0 red",
                height: "100%",
                width: "100%",
                background: "black",
                padding: "50",
                text {
                    layer: "1",
                    "Some shadows!"
                }
            }
        }
    ))
}
