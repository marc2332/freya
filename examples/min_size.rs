use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        rect {
            height: "50%",
            width: "50%",
            min_height: "100",
            min_width: "200",
            background: "black",
            paragraph {
                width: "100%",
                "This element has a minimum width of 200 and minimum height of 100"
            }
        }
    ))
}
