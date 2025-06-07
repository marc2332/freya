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
                        main_align: "space-evenly",
                        cross_align: "center",
                        background: "orange",
                        padding: "15",
                        height: "70%",
                        rect {
                            width: "20",
                            height: "20",
                            background: "black"
                        }
                        rect {
                            width: "20",
                            height: "20",
                            background: "white"
                        }
                    }
                }
                rect {
                    cross_align: "end",
                    width: "100%",
                    height: "50%",
                    rect {
                        main_align: "space-between",
                        cross_align: "center",
                        width: "50%",
                        height: "100%",
                        label {
                            "Some crabs"
                        }
                        label {
                            "🦀🦀"
                        }
                        rect {
                            main_align: "space-around",
                            direction: "horizontal",
                            cross_align: "center",
                            background: "black",
                            width: "fill",
                            padding: "5",
                            rect {
                                width: "20",
                                height: "20",
                                background: "blue"
                            }
                            rect {
                                width: "70",
                                height: "40",
                                background: "red"
                            }
                            rect {
                                width: "20",
                                height: "20",
                                background: "white"
                            }
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
