#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Viewport unit", (400.0, 350.0));
}

fn app() -> Element {
    rsx!(
        rect {
            background: "rgb(255, 35, 60)",
            width: "100%",
            height: "50v", // 50% of the Window
            rect {
                background: "rgb(71, 255, 60)",
                width: "100%",
                height: "25v" // 25% of the Window
            }
        }
    )
}
