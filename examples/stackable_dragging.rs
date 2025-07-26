#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Stackable Dragging", (400.0, 350.0));
}

fn app() -> Element {
    rsx!(

    )
}
