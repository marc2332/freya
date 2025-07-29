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
            margin: "20",
            width: "100%",
            height: "fill",
            direction: "horizontal",
            main_align: "start",
            cross_align: "center",
            wrap_content: "wrap",
            border: "1 outer black",
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
