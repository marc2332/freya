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
        TabsBar {
            Tab {
                label {
                    "Tab 1"
                }
            }
            Tab {
                label {
                    "Tab 2"
                }
            }
            Tab {
                label {
                    "Tab 3"
                }
            }
            Tab {
                label {
                    "Tab 4"
                }
            }
        }
    )
}
