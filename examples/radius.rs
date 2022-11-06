#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{
    dioxus_elements::{self, WheelEvent},
    *,
};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut radius = use_state(&cx, || 30f32);

    let onwheel = move |e: WheelEvent| {
        let y = e.get_delta_y();
        radius += (y as f32) * 20.0;
    };

    render!(
        container {
            height: "100%",
            width: "100%",
            padding: "125",
            onwheel: onwheel,
            rect {
                shadow: "0 0 150 30.0 black",
                radius: "{radius}",
                height: "100%",
                width: "100%",
                background: "black",
            }
        }
    )
}
