#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_title(app, "Spacing");
}

fn app() -> Element {
    rsx!(
        rect {
            direction: "horizontal",
            main_align: "center",
            cross_align: "center",
            spacing: "5",
            rect {
                direction: "vertical",
                cross_align: "center",
                spacing: "20",
                rect {
                    background: "red",
                    width: "100",
                    height: "100"
                }
                rect {
                    background: "green",
                    width: "100",
                    height: "100"
                }
            }
            rect {
                direction: "vertical",
                main_align: "center",
                spacing: "50",
                rect {
                    background: "blue",
                    width: "100",
                    height: "100"
                }
                rect {
                    background: "black",
                    width: "100",
                    height: "100"
                }
            }
        }
    )
}
