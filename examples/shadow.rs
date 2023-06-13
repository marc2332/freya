#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let shadow_size = use_state(cx, || 30f32);

    let onwheel = move |e: WheelEvent| {
        let y = e.get_delta_y() as f32;
        shadow_size.set((*shadow_size.get() + y).clamp(0.0, 100.0));
    };

    render!(
        container {
            height: "100%",
            width: "100%",
            padding: "60",
            onwheel: onwheel,
            rect {
                shadow: "inset 0 0 8 red",
                height: "80",
                width: "80",
                background: "black",
                padding: "25",
                label {
                    "Scroll!"
                }
            }
        }
    )
}
