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
            width: "800",
            height: "fill",
            direction: "horizontal",
            main_align: "space-around",
            wrap_content: "wrap",
            border: "1px outer black",
            spacing: "15",

            rect {
                width: "300",
                height: "200",
                background: "red",
            }

            rect {
                width: "300",
                height: "200",
                background: "orange",
            }

            rect {
                width: "300",
                height: "200",
                background: "green",
            }
        }
    )
}
