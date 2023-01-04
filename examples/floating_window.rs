#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg((
        app,
        WindowConfig::<()>::builder()
            .with_width(100)
            .with_height(100)
            .with_decorations(false)
            .with_transparency(true)
            .with_title("Floating window")
            .build(),
    ));
}

fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "white",
            padding: "20",
            display: "center",
            direction: "both",
            width: "100",
            height: "100",
            radius: "50",
            label {
                color: "black",
                "Whoooah"
            }
        }
    )
}
