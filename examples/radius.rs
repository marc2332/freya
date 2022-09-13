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
    let mut radius = use_state(&cx, || 30f32);

    let onwheel = move |e: UiEvent<WheelData>| {
        let y = e.delta().strip_units().y;
        radius += (y as f32) * 20.0;
    };

    cx.render(rsx!(
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
    ))
}
