#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Resizable Container", (1000.0, 550.0));
}

fn app() -> Element {
    let mut panels = use_signal(|| 5);
    rsx!(
        ResizableContainer {
                    ResizablePanel {
                        label {
                            "Panel 0"
                        }
                    }
                    ResizablePanel {
                        ResizableContainer {
                            direction: "horizontal",
                            ResizablePanel {
                                label {
                                    "Panel 2"
                                }
                            }
                            ResizablePanel {
                                label {
                                    "Panel 3"
                                }
                            }
                            ResizablePanel {
                                label {
                                    "Panel 4"
                                }
                            }
                        }
                    }
                }
    )
}
