use dioxus::prelude::*;
use elements_namespace as dioxus_elements;
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let heights = use_state(&cx, || (50, 50));

    cx.render(rsx! {
        view {
            height: "stretch",
            width: "stretch",
            view {
                background: "red",
                height: "{heights.0}%",
                width: "stretch",
                padding: "20",
                layer: "1",
                onclick: |_| heights.with_mut(|v| {
                    v.1 -= 5;
                    v.0 += 5;
                }),
                text {
                    layer: "1",
                    "Click to increase",
                }
            }
            view {
                background: "blue",
                height: "{heights.1}%",
                width: "stretch",
                padding: "20",
                layer: "1",
                onclick: |_| heights.with_mut(|v| {
                    v.0 -= 5;
                    v.1 += 5;
                }),
                text {
                    layer: "1",
                    "Click to increase",
                }
            }
        }
    })
}
