#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            rect {
                width: "100%",
                height: "50%",
                background: "rgb(28, 28, 28)",
                color: "white",
                WindowDragArea {
                    label {
                        width: "100%",
                        text_align: "center",
                        margin: "100 0",
                        "Drag Me!"
                    }
                }
            }
            rect {
                width: "100%",
                height: "50%",
                label {
                    text_align: "center",
                    margin: "100 0",
                    "Use the top half to drag the window"
                }
            }
        }
    )
}
