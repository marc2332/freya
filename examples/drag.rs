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
        DraggableCanvas {
            rect {
                background: "rgb(35, 35, 35)",
                width: "100%",
                height: "100%",
                Draggable {
                    rect {
                        background: "rgb(0, 119, 182)",
                        width: "120",
                        height: "120",
                        corner_radius: "999",
                        shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
                        main_align: "center",
                        cross_align: "center",
                        label {
                            color: "white",
                            "Drag me"
                        }
                    }
                }
            }
        }
    )
}
