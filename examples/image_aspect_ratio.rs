#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::fmt::Display;

use freya::prelude::*;
fn main() {
    launch(app);
}

static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

#[derive(Clone, Copy, PartialEq)]
enum AspectRatio {
    Max,
    Min,
    None,
    Fit,
}

impl Display for AspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Max => f.write_str("max"),
            Self::Min => f.write_str("min"),
            Self::None => f.write_str("none"),
            Self::Fit => f.write_str("fit"),
        }
    }
}

fn app() -> Element {
    let image_data = static_bytes(RUST_LOGO);
    let mut aspect_ratio = use_signal(|| AspectRatio::Min);

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            padding: "25",
            main_align: "center",
            cross_align: "center",
            Dropdown {
                selected_item: rsx!( label { "{aspect_ratio}" } ),
                for ar in [AspectRatio::Max, AspectRatio::Min, AspectRatio::None, AspectRatio::Fit] {
                    DropdownItem {
                        onpress: move |_| aspect_ratio.set(ar),
                        label { "{ar}" }
                    }
                }
            }
            image {
                max_height: "fill",
                image_data,
                aspect_ratio: "{aspect_ratio}"
            }
        }
    )
}
