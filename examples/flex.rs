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
            height: "100%",
            width: "100%",
            direction: "horizontal",
            main_align: "flex",
            spacing: "4",
            padding: "4",
            rect {
                height: "100%",
                width: "10%",
                background: "red",
            }
            rect {
                width: "flex",
                height: "100%",
                background: "orange",
            }
            rect {
                height: "100%",
                width: "25",
                background: "black",
            }
            rect {
                width: "flex(3)",
                height: "100%",
                background: "yellow",
            }
            rect {
                width: "flex",
                height: "100%",
                background: "green",
            }
            rect {
                height: "100%",
                width: "30%",
                background: "blue",
            }
        }
    )
}
