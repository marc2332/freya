#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{hotreload::FreyaCtx, prelude::*};

fn main() {
    dioxus_hot_reload::hot_reload_init!(Config::<FreyaCtx>::default());

    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    render!(
        container {
            background: "rgb(15, 15, 15)",
            padding: "50",
            direction: "horizontal",
            width: "auto",
            height: "100%",
            onclick: move |_| {
                count += 1;
            },
            container {
                padding: "50",
                height: "100%",
                width: "50%",
                background: "red",
                label {
                    font_size: "50",
                    "{count}"
                }
            }
            Comp {}
        }
    )
}

#[allow(non_snake_case)]
fn Comp(cx: Scope) -> Element {
    render!(rect {
        width: "50%",
        height: "100%",
        background: "yellow"
    })
}
