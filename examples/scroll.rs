use components::ScrollView;
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
            padding: "100",
            background: "black",
            ScrollView {
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
            }
        }
    ))
}
