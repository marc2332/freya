#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        GestureArea {
            ongesture: |e| println!("{e:?}"),
            rect {
                width: "100%",
                height: "100%",
                direction: "both",
                display: "center",
                label {
                    align: "center",
                    width: "100%",
                    "Touch!"
                }
            }
        }
    )
}
