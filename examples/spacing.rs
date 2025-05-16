#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_title(app, "Spacing");
}

fn app() -> Element {
    rsx!(
        rect {
            spacing: "10",
            for i in 0..6 {
                rect {
                    key: "{i}",
                    background: "rgb(25, 35, 45)",
                    width: "100%",
                    height: "50"
                }
            }
        }
    )
}