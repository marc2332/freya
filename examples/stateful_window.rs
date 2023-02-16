#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        WindowConfig::builder()
            .with_title("Window with state")
            .with_state(10)
            .build(),
    );
}

fn app(cx: Scope) -> Element {
    let num = cx.consume_context::<i32>().unwrap();

    render!(rect {
        background: "white",
        padding: "10",
        width: "100%",
        height: "100%",
        radius: "50",
        label {
            color: "black",
            "{num}"
        }
    })
}
