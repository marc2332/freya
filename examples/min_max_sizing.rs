#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let (node_ref, size) = use_node();
    rsx!(
        rect {
            width: "50%",
            height: "50%",
            min_width: "200",
            min_height: "100",
            max_width: "300",
            max_height: "250",
            background: "black",
            reference: node_ref,
            label {
                color: "white",
                "Size: {size.area.width()}x{size.area.height()}"
            }
        }
    )
}
