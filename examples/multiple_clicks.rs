#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app() -> Element {
    let mut clicks = use_multiple_clicks(Duration::from_secs(1), 3);

    rsx!(
        rect {
            onclick: move |_| {
                clicks.clicked();
            },
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            label {
                font_size: "75",
                "{clicks.len()}"
            }
        }
    )
}
