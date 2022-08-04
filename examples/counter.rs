use dioxus::prelude::*;
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        div {
            height: "20%",
            width: "100%",
            background: "black",
            padding: "25",
            p { "Number is: {count}" }
        }
        div {
            height: "40%",
            width: "100%",
            background: "blue",
            padding: "25",
            onclick: move |_| count += 1,
            p { "Increase!" }
        }
        div {
            height: "40%",
            width: "100%",
            background: "red",
            padding: "25",
            onclick: move |_| count -= 1,
            p { "Decrease!" }
        }
    ))
}
