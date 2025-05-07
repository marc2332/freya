#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "OverflowedContent", (400.0, 350.0));
}

import_svg!(Ferris, "./ferris.svg", {
    width: "60",
    height: "60",
});

fn app() -> Element {
    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            width: "fill",
            height: "fill",
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
        }
    )
}
