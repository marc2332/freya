#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
        rect {
            height: "100%",
            width: "100%",
            background: "black",
            color: "white",
            padding: "12",
            onclick: move |_| count += 1,

            text {
                "Hello ",
                text {
                    font_weight: "bold",
                    color: "red",
                    "World!",
                    text {
                        color: "green",
                        " Wow."
                    }
                }
            }
        }
    )
}