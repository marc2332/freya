#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let (node_ref, size) = use_node(cx);
    render!(
        rect {
            height: "50%",
            width: "50%",
            min_height: "100",
            min_width: "200",
            max_width: "300",
            max_height: "250",
            background: "black",
            reference: node_ref,
            paragraph {
                width: "100%",
                label {
                    "Size: {size.width / 2.0}x{size.height}"
                }
            }
        }
    )
}
