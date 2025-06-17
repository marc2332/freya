#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use tokio::time::sleep;

fn main() {
    launch_with_props(app, "Chip", (450.0, 450.0));
}

fn app() -> Element {
    use_init_default_theme();
    rsx!(
        Body {
            rect {
                width: "fill",
                height: "fill",
                direction: "horizontal",
                content: "flex",
                padding: "16",
                Chip {
                    label {
                        "Bananas"
                    }
                }
            }
        }
    )
}
