#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(400.0)
            .with_height(200.0)
            .with_decorations(false)
            .with_transparency(true)
            .with_title("Floating window")
            .build(),
    );
}

fn app() -> Element {
    rsx!(
        rect {
            background: "white",
            padding: "10",
            main_align: "center",
            cross_align: "center",
            width: "100%",
            height: "100%",
            corner_radius: "15",
            label {
                color: "black",
                "A frameless window"
            }
        }
    )
}
