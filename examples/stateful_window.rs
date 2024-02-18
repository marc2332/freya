#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::builder()
            .with_title("Window with state")
            .with_state(10)
            .build(),
    );
}

fn app() -> Element {
    let num = consume_context::<i32>();

    rsx!(rect {
        width: "100%",
        height: "100%",
        main_align: "center",
        cross_align: "center",
        label {
            font_size: "50",
            "{num}"
        }
    })
}
