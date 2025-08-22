#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_params(app, "Resizable Container", (1000.0, 550.0));
}

fn app() -> Element {
    let mut panels = use_signal(|| 5);

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
                    Button {
                        onpress: move|_| panels += 1,
                        label {
                            "Push"
                        }
                    }
                    Button {
                        onpress: move|_| panels -= 1,
                        label {
                            "Remove"
                        }
                    }
                }
            }
            ResizablePanel {
                initial_size: 50.,
                ResizableContainer {
                    direction: "horizontal",
                    for panel in 1..panels() {
                        ResizablePanel {
                            key: "{panel}",
                            initial_size: panel as f32 * 15.,
                            min_size: panel as f32 * 5.,
                            order: panel,
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
                                    "Panel {panel}"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
