#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{dioxus_elements, *};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        rect {
            background: "red",
            padding: "50",
            direction: "both",
            width: "auto",
            height: "auto",
            rect {
                padding: "25",
                height: "100%",
                width: "100%",
                background: "blue",
                onclick: move |_| count += 10,
                label {
                    "Increase!"
                }
                label {
                    "{count}"
                }
            }
        }
    ))
}
