use dioxus::prelude::*;
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        div {
            height: "40%",
            width: "100%",
            background: "red",
            padding: "25",
            p {
                tabindex: "1",
                "{count}"
            }
            div {
                height: "40%",
                width: "100%",
                background: "blue",
                padding: "25",
                onclick: move |_| count += 10,
                tabindex: "1",
                p {
                    tabindex: "1",
                    "Decrease!"
                 }

            }
        }
    ))
}
