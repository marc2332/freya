#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut font_size = use_state(cx, || 30f32);

    let onwheel = move |e: WheelEvent| {
        let y = e.get_delta_y();
        font_size += (y as f32) * 5.0;
    };

    render!(
        label {
            onwheel: onwheel,
            font_size: "{font_size}",
            "hello world"
        }
    )
}
