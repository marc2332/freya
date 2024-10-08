#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "fill",
            direction: "horizontal",
            main_align: "space-around",
            content: "flex",

            rect {
                width: "flex(0.5)",
                height: "fill",
                background: "red",
            }

            rect {
                width: "120",
                height: "fill",
                background: "orange",
            }
        }
    )
}
