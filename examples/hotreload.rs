#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    dioxus_hot_reload::hot_reload_init!();

    launch(app);
}

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    render!(
        container {
            background: "rgb(15, 15, 15)",
            padding: "50",
            direction: "both",
            width: "auto",
            height: "auto",
            onclick: move |_| {
                count.with_mut(|c| *c += 1);
            },
            container {
                padding: "50",
                height: "100%",
                width: "100%",
                background: "rgb(45, 45, 45)",
                label {
                    "{count}"
                }
            }
        }
    )
}
