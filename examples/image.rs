#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::core::UiEvent;
use dioxus::events::WheelData;
use dioxus::prelude::*;
use freya::{
    dioxus_elements::{self},
    *,
};

fn main() {
    launch(app);
}

static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

fn app(cx: Scope) -> Element {
    let mut size = use_state(&cx, || 150);

    let onwheel = move |e: UiEvent<WheelData>| {
        let y = e.delta().strip_units().y;
        if *size.get() >= 15 && y > 15.0 {
            return;
        }
        size += (y as i32) * 20;
    };

    render!(
        rect {
            width: "100%",
            height: "100%",
            padding: "100",
            onwheel: onwheel,
            image {
                image_data: RUST_LOGO,
                width: "{size}",
                height: "{size}",
            }
        }
    )
}
