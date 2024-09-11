#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Resizable Container", (700.0, 450.0));
}

fn app() -> Element {
    rsx!(
        ResizableContainer {
            ResizablePanel {
                initial_size: 50.,
                label {
                    "Panel 0"
                }
            }
            ResizableHandle { }
            ResizablePanel { // Panel 1
                initial_size: 50.,
                ResizableContainer {
                    direction: "horizontal",
                    ResizablePanel {
                        initial_size: 20.,
                        rect {
                            width: "fill",
                            height: "fill",
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
                        rect {
                            width: "fill",
                            height: "fill",
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
                            background:
                            "linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)",
                            label {
                                "Panel 3"
                            }
                        }
                    }
                    ResizableHandle { }
                    ResizablePanel {
                        initial_size: 20.,
                        rect {
                            width: "fill",
                            height: "fill",
                            background:
                            "linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)",
                            label {
                                "Panel 4"
                            }
                        }
                    }
                    ResizableHandle { }
                    ResizablePanel {
                        initial_size: 20.,
                        rect {
                            width: "fill",
                            height: "fill",
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
