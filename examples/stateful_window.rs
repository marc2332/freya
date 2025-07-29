#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        LaunchConfig::new()
            .with_state(10)
            .with_default_font("Impact")
            .with_window(
                WindowConfig::default()
                    .with_app(app)
                    .with_title("Window with state"),
            ),
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
