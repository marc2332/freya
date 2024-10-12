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
            height: "fill",
            width: "fill",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            spacing: "10",
            TooltipContainer {
                tooltip: rsx!(
                    Tooltip {
                        text: "Hello, World!"
                    }
                ),
                Button {
                    label { "Hello!!" }
                }
            }
            TooltipContainer {
                position: TooltipPosition::Besides,
                tooltip: rsx!(
                    Tooltip {
                        text: "Hello, World!"
                    }
                ),
                Button {
                    label { "Hello!!" }
                }
            }
        }
    )
}
