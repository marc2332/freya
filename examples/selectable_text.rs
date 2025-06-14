#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Selectable Text", (900.0, 650.0));
}

fn app() -> Element {
    rsx!(
        rect {
            padding: "25",
            spacing: "10",
            label {
                font_size: "35",
                "Select the text from below"
            }
            rect {
                font_size: "25",
                color: "rgb(20, 20, 20)",
                SelectableText {
                    value: "You can select this looooooooooong text"
                }
                SelectableText {
                    value: "Or this short text :)"
                }
            }
        }
    )
}
