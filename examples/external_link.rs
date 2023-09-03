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
        ExternalLink {
            url: "https://link1.com",
            label {
                font_size: "25",
                "link1"
            }
        }
        ExternalLink {
            url: "https://link2.com",
            label {
                font_size: "25",
                "link2"
            }
        }
        ExternalLink {
            url: "https://link3.com",
            label {
                font_size: "25",
                "link3 (no tooltip)"
            },
            show_tooltip: false
        }
    )
}
