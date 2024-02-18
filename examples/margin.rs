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
            background: "rgb(255, 100, 50)",
            rect {
                margin: "50",
                width: "100",
                height: "100",
                background: "rgb(50, 255, 100)",
            }
            rect {
                margin: "10 20 30 40",
                width: "100",
                height: "100",
                background: "rgb(100, 50, 255)",
            }
            rect {
                width: "100",
                height: "100",
                background: "rgb(150, 150, 150)",
            }
        }
    )
}
