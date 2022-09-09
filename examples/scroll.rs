use components::ScrollView;
use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use freya::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        rect {
            height: "100%",
            width: "100%",
            padding: "100",
            background: "black",
            ScrollView {
                rect {
                    height: "200",
                    width: "100%",
                    background: "red",
                    padding: "20",
                    rect {
                        height: "100%",
                        width: "100%",
                        background: "blue",
                        label { "hi" }
                    }
                }
                rect {
                    height: "200",
                    width: "100%",
                    background: "red",
                    label { "hi" }
                }
                rect {
                    height: "200",
                    width: "100%",
                    background: "red",
                    label { "hi" }
                }
            }
        }
    ))
}
