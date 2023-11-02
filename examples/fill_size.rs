#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app(cx: Scope) -> Element {
    render!(
        rect {
            height: "50%",
            min_height: "150",
            max_height: "300",
            width: "100%",
            background: "rgb(0, 119, 182)",
        }
        rect {
            height: "fill",
            width: "100%",
            background: "rgb(20, 150, 220)",
        }
    )
}
