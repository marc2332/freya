#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Resizable Container", (1000.0, 550.0));
}

fn app() -> Element {
    rsx!(
        ResizableContainer {
            ResizablePanel {
                initial_size: 50.,
                rect {
                    width: "fill",
                    height: "fill",
                    main_align: "center",
                    cross_align: "center",
                    label {
                        "Panel 1"
                    }
                }
            }
            ResizableHandle { }
            ResizablePanel { // Panel 1
                initial_size: 50.,
                ResizableContainer {
                    direction: "horizontal",
                    ResizablePanel {
                        initial_size: 35.,
                        rect {
                            width: "fill",
                            height: "fill",
                            main_align: "center",
                            cross_align: "center",
                            corner_radius: "6",
                            color: "white",
                            background:
                            "linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)",
                            label {
                                "Panel 1"
                            }
                        }
                    }
                    ResizableHandle { }
                    ResizablePanel {
                        initial_size: 20.,
                        min_size: 20.,
                        rect {
                            width: "fill",
                            height: "fill",
                            main_align: "center",
                            cross_align: "center",
                            corner_radius: "6",
                            color: "white",
                            background:
                            "linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)",
                            label {
                                "Panel 2"
                            }
                        }
                    }
                    ResizableHandle { }
                    ResizablePanel {
                        initial_size: 20.,
                        rect {
                            width: "fill",
                            height: "fill",
                            main_align: "center",
                            cross_align: "center",
                            corner_radius: "6",
                            color: "white",
                            background:
                            "linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)",
                            label {
                                "Panel 3"
                            }
                        }
                    }
                    ResizableHandle { }
                    ResizablePanel {
                        initial_size: 15.,
                        min_size: 10.,
                        rect {
                            width: "fill",
                            height: "fill",
                            main_align: "center",
                            cross_align: "center",
                            corner_radius: "6",
                            color: "white",
                            background:
                            "linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)",
                            label {
                                "Panel 4"
                            }
                        }
                    }
                    ResizableHandle { }
                    ResizablePanel {
                        initial_size: 10.,
                        rect {
                            width: "fill",
                            height: "fill",
                            main_align: "center",
                            cross_align: "center",
                            corner_radius: "6",
                            color: "white",
                            background:
                            "linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)",
                            label {
                                "Panel 5"
                            }
                        }
                    }
                }
            }
        }
    )
}
