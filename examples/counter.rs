use dioxus::prelude::*;
use elements_namespace as dioxus_elements;
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        rect {
            height: "20%",
            width: "100%",
            background: "black",
            padding: "25",
            label {
                "Number is: {count}"
            }
        }
        rect {
            height: "40%",
            width: "100%",
            background: "blue",
            padding: "25",
            onclick: move |_| count += 1,
            label {
                "Increase!"
             }
        }
        rect {
            height: "40%",
            width: "100%",
            background: "red",
            padding: "25",
            onclick: move |_| count -= 1,
            label {
                "Decrease!"
             }
        }
    ))
}
