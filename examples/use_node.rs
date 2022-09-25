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
    let (node_ref, size) = use_node(&cx);

    let w = size.width / 2.0;

    cx.render(rsx!(rect {
        width: "100%",
        height: "100%",
        reference: node_ref,
        container {
            width: "{w}",
            height: "100%",
            background: "blue",
            padding: "20",
            label {
                "Size: {size.width / 2.0}x{size.height}"
            }
        }
    }))
}
