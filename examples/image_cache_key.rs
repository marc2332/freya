#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

fn app() -> Element {
    rsx!(
        ScrollView {
            image {
                image_data: static_bytes(RUST_LOGO),
                width: "fill",
                height: "500",
                aspect_ratio: "max",
                cache_key: "rust-logo",
            }
            image {
                image_data: static_bytes(RUST_LOGO),
                width: "fill",
                height: "500",
                aspect_ratio: "max",
                cache_key: "rust-logo"
            }
            image {
                image_data: static_bytes(RUST_LOGO),
                width: "fill",
                height: "500",
                aspect_ratio: "max",
                cache_key: "rust-logo"
            }
        }
    )
}
