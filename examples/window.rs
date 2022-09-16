#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{dioxus_elements, *};

fn main() {
    launch_cfg(vec![(
        app,
        WindowConfig {
            width: 100,
            height: 100,
            decorations: false,
            transparent: true,
            title: "Custom window",
        },
    )]);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        rect {
            background: "white",
            padding: "50",
            direction: "both",
            width: "100%",
            height: "100%",
            radius: "50",
            label {
                color: "black",
                "Whoooah"
            }
        }
    ))
}
