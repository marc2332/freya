#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

static FERRIS: &[u8] = include_bytes!("./ferris.svg");

fn app(cx: Scope) -> Element {
    render!(
        svg {
            width: "100%",
            height: "50%",
            svg_data: FERRIS,
        }
        svg {
            width: "100%",
            height: "50%",
            svg_data: FERRIS,
        }
    )
}
