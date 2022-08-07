use dioxus::prelude::*;
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let heights = use_state(&cx, || (50, 50));

    cx.render(rsx! {
        div {
            height: "stretch",
            width: "stretch",
            div {
                background: "red",
                height: "{heights.0}%",
                width: "stretch",
                padding: "20",
                tabindex: "1",
                onclick: |_| heights.with_mut(|v| {
                    v.1 -= 5;
                    v.0 += 5;
                }),
                p {
                    tabindex: "1",
                    "Click to increase",
                }
            }
            div {
                background: "blue",
                height: "{heights.1}%",
                width: "stretch",
                padding: "20",
                tabindex: "1",
                onclick: |_| heights.with_mut(|v| {
                    v.0 -= 5;
                    v.1 += 5;
                }),
                p {
                    tabindex: "1",
                    "Click to increase",
                }
            }
        }
    })
}
