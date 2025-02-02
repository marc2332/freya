#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Animation", (400.0, 350.0));
}

import_svg!(Ferris, "./ferris.svg", {
    width: "70%",
    height: "50%"
});

fn app() -> Element {
    rsx!(
        Button {
            OverflowedContent {
                width: "100",
                rect {
                    direction: "horizontal",
                    cross_align: "center",
                    Ferris { }
                    label {
                        "Freya is a cross-platform GUI library for Rust"
                    }
                }
            }
        }
    )
}
