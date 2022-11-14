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
            background: "rgb(150, 150, 150)",
            height: "100%",
            width: "100%",
            display: "center",
            direction: "both",
            rect {
                background: "rgb(255, 0, 255)",
                padding: "30",
                height: "75%",
                width: "75%",
                display: "center",
                direction: "vertical",
                rect {
                    background: "rgb(0, 255, 0)",
                    height: "33%",
                    width: "50%",
                }
                label {
                    color: "black",
                    "Hello World"
                }
                rect {
                    background: "rgb(255, 255, 0)",
                    height: "33%",
                    width: "50%",
                    display: "center",
                    direction: "horizontal",
                    rect {
                        background: "rgb(255, 255, 0)",
                        height: "100%",
                        width: "70%",
                        display: "center",
                        direction: "vertical",
                        rect {
                            background: "rgb(180, 180, 180)",
                            height: "50%",
                            width: "100%",
                        }
                        rect {
                            background: "rgb(100, 100, 100)",
                            height: "50%",
                            width: "100%",
                            rect {
                                background: "rgb(200, 200, 200)",
                                height: "100%",
                                width: "100%",
                                display: "center",
                                direction: "horizontal",
                                padding: "10",
                                rect {
                                    background: "red",
                                    height: "100%",
                                    width: "35%",
                                }
                                rect {
                                    height: "100%",
                                    width: "18",
                                    display: "center",
                                    direction: "vertical",
                                    label {
                                        color: "black",
                                        "HI",
                                    }
                                }
                                rect {
                                    background: "black",
                                    height: "100%",
                                    width: "35%",
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
