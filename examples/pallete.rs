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
            direction: "horizontal",
            view {
                height: "100%",
                width: "10%",
                background: "rgb(66, 75, 84)",
            }
            view {
                height: "100%",
                width: "10%",
                background: "rgb(179, 141, 151)",
            }
            view {
                height: "100%",
                width: "10%",
                background: "rgb(213, 172, 169)",
            }
            view {
                height: "100%",
                width: "10%",
                background: "rgb(235, 207, 178)",
            }
            view {
                height: "100%",
                width: "10%",
                background: "rgb(197, 186, 175)",
            }
            view {
                height: "100%",
                width: "10%",
                background: "rgb(237, 238, 201)",
            }
            view {
                height: "100%",
                width: "10%",
                background: "rgb(221, 231, 199)",
            }
            view {
                height: "100%",
                width: "10%",
                background: "rgb(191, 216, 189)",
            }
            view {
                height: "100%",
                width: "10%",
                background: "rgb(152, 201, 163)",
            }
            view {
                height: "100%",
                width: "10%",
                background: "rgb(119, 191, 163)",
            }
        }
    ))
}
