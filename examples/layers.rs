#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    render!(
        container {
            color: "white",
            background: "rgb(15, 15, 15)",
            padding: "25",
            direction: "both",
            width: "auto",
            height: "auto",
            onclick: move |_| {
                count.with_mut(|c| *c = 0);
            },
            container {
                padding: "25",
                height: "100%",
                width: "100%",
                background: "rgb(45, 45, 45)",
                onclick: move |_| {
                    count.with_mut(|c| *c = 1)
                },
                container {
                    padding: "25",
                    height: "100%",
                    width: "100%",
                    background: "rgb(90, 90, 90)",
                    onclick: move |_| {
                        count.with_mut(|c| *c = 2)
                    },
                    label {
                        "Click on every layer."
                    }
                    label {
                        "{count}"
                    }
                }
            }
        }
    )
}
