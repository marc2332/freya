#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app() -> Element {
    rsx!(
        rect {
            padding: "8",
            spacing: "8",
            rect {
                direction: "horizontal",
                overflow: "clip",
                corner_radius: "32",
                border: "2 outer black",
                ButtonSegment {
                    label {
                        "Option A"
                    }
                }
                ButtonSegment {
                    label {
                        "Option B"
                    }
                }
                ButtonSegment {
                    label {
                        "Option C"
                    }
                }
            }

            rect {
                direction: "horizontal",
                border: "2 outer black",
                ButtonSegment {
                    label {
                        "Option A"
                    }
                }
            }
            rect {
                direction: "horizontal",
                overflow: "clip",
                corner_radius: "32",
                border: "2 outer black",
                ButtonSegment {
                    label {
                        "Option A"
                    }
                }
                ButtonSegment {
                    label {
                        "Option B"
                    }
                }
                ButtonSegment {
                    label {
                        "Option C"
                    }
                }
                ButtonSegment {
                    label {
                        "Option A"
                    }
                }
            }
        }
    )
}
