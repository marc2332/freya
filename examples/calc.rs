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
    cx.render(rsx!(rect {
        background: "rgb(233, 196, 106)",
        padding: "50",
        direction: "both",
        width: "calc(100% - 50% + 100)",
        height: "100%",
    }))
}
