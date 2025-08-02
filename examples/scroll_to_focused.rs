#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_params(app, "Scroll to Focused", (450.0, 450.0));
}

fn app() -> Element {
    rsx!(
        ScrollView {
            padding: "50",
            ScrollView {
                direction: "horizontal",
                height: "500",
                spacing: "500",
                Button {
                    label { "1" }
                }
                label {
                    a11y_focusable: "true",
                    max_lines: "1",
                    "This can also be focused!"
                }
                Button {
                    label { "2" }
                }
            }
            ScrollView {
                direction: "horizontal",
                height: "500",
                spacing: "500",
                Button {
                    label { "3" }
                }
                label {
                    a11y_focusable: "true",
                    max_lines: "1",
                    "And this too!"
                }
                Button {
                    label { "4" }
                }
            }
        }
    )
}
