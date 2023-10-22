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
            cross_align: "end",
            main_align: "end",
            rect {
                width: "65%",
                height: "65%",
                background: "yellow",
                main_align: "start",
                cross_align: "start",
                overflow: "clip",
                rect {
                    main_align: "end",
                    cross_align: "center",
                    background: "red",
                    direction: "horizontal",
                    width: "50%",
                    height: "50%",
                    overflow: "clip",
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
                rect {
                    cross_align: "end",
                    width: "100%%",
                    height: "50%",
                    rect {
                        main_align: "start",
                        cross_align: "center",
                        width: "50%",
                        height: "100%",
                        label {
                            "Some crabs"
                        }
                        label {
                            "🦀🦀"
                        }
                        label {
                            "Even more crabs"
                        }
                        label {
                            "🦀🦀🦀🦀🦀"
                        }
                    }
                }
            }
            label {
                "Hello, World!"
            }
        }
    )
}
