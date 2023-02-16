#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(rect {
        background: "rgb(233, 196, 106)",
        padding: "25",
        direction: "both",
        width: "calc(100% - 50% + 100)",
        height: "100%",
    })
}
