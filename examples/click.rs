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
    let count = use_state(&cx, || 0);
    cx.render(rsx!(
        container {
            background: "rgb(15, 15, 15)",
            padding: "50",
            direction: "both",
            width: "auto",
            height: "auto",
            onclick: move |_| {
                count.with_mut(|c| *c = 0);
            },
            container {
                padding: "50",
                height: "100%",
                width: "100%",
                background: "rgb(45, 45, 45)",
                onclick: move |_| {
                    count.with_mut(|c| *c = 1)
                },
                container {
                    padding: "50",
                    height: "100%",
                    width: "100%",
                    background: "rgb(90, 90, 90)",
                    onclick: move |_| {
                        count.with_mut(|c| *c = 2)
                    },
                    label {
                        "Clikck on every layer."
                    }
                    label {
                        "{count}"
                    }
                }
            }
        }
    ))
}
