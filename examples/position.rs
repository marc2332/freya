#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Position", (400.0, 350.0));
}

fn app() -> Element {
    rsx!(
        rect {
            height: "100%",
            width: "100%",
            rect {
                height: "20%",
                width: "20%",
                background: "black",
                position: "absolute",
                position_top: "10",
                position_left: "10",
            }
            rect {
                height: "20%",
                width: "20%",
                background: "black",
                position: "absolute",
                position_top: "10",
                position_right: "10",
            }
            rect {
                height: "20%",
                width: "20%",
                background: "black",
                position: "absolute",
                position_bottom: "10",
                position_right: "10",
            }
            rect {
                height: "20%",
                width: "20%",
                background: "black",
                position: "absolute",
                position_bottom: "10",
                position_left: "10",
                padding: "20",

                rect {
                    height: "100%",
                    width: "100%",
                    corner_radius: "12",
                    background: "red",
                    position: "absolute",
                    position_top: "0",
                    position_left: "0",
                }

                rect {
                    height: "20%",
                    width: "20%",
                    background: "blue",
                }
            }
        }
    )
}
