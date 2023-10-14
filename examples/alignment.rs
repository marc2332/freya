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
        rect {
            height: "100%",
            width: "100%",
            cross_alignment: "end",
            main_alignment: "end",
            rect {
                width: "300",
                height: "300",
                background: "yellow",
                main_alignment: "start",
                cross_alignment: "start",
                rect {
                    main_alignment: "end",
                    cross_alignment: "center",
                    background: "red",
                    direction: "horizontal",
                    width: "150",
                    height: "150",
                    rect {
                        width: "50",
                        height: "50",
                        background: "green"
                    }
                    rect {
                        width: "50",
                        height: "50",
                        background: "orange"
                    }
                }
                label {
                    "🦀🦀🦀🦀🦀🦀"
                }
            }
            label {
                "Hello, World!"
            }
        }
    )
}
