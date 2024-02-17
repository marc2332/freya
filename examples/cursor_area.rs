#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use winit::window::CursorIcon;

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
                CursorArea {
                    icon: CursorIcon::Text,
                    label {
                        height: "100%",
                        width: "100%",
                        "text cursor"
                    }
                }
            }
            rect {
                width: "100%",
                height: "50%",
                CursorArea {
                    icon: CursorIcon::Progress,
                    label {
                        height: "100%",
                        width: "100%",
                        "Loading..."
                    }
                }
            }
        }
    )
}
