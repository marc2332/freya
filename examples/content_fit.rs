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
            content: "fit",
            height: "100%",
            rect {
                width: "fill-min",
                height: "25%",
                background: "red",
            }
            rect {
                width: "150",
                height: "25%",
                background: "green",
            }
            rect {
                width: "fill-min",
                height: "25%",
                background: "blue",
            }
            rect {
                width: "300",
                height: "25%",
                background: "black",
            }
        }
    )
}
