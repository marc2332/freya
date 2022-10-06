#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::{core::UiEvent, events::WheelData, prelude::*};
use freya::{dioxus_elements, *};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut shadow_size = use_state(&cx, || 30f32);

    let onwheel = move |e: UiEvent<WheelData>| {
        let y = e.delta().strip_units().y;
        shadow_size += (y as f32) * 7.0;
    };

    render!(
        container {
            height: "100%",
            width: "100%",
            padding: "125",
            onwheel: onwheel,
            rect {
                shadow: "0 10 210 {shadow_size} red",
                height: "100%",
                width: "100%",
                background: "black",
                padding: "50",
                label {
                    "Scroll!"
                }
            }
        }
    )
}
