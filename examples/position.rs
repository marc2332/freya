#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Position", (400.0, 350.0));
}

fn app(cx: Scope) -> Element {
    render!(
        rect {
            height: "100%",
            width: "100%",
            rect {
                height: "20%",
                width: "100%",
                background: "green",
            }
            rect {
                height: "100",
                width: "100",
                background: "red",
                position: "absolute",
                layer: "-1"
            }
            rect {
                height: "20%",
                width: "100%",
                background: "orange",
            }
            rect {
                height: "20%",
                width: "100%",
                background: "yellow",
            }
        }
    )
}
