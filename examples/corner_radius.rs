#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let radius = use_state(cx, || 30f32);

    let onwheel = move |e: WheelEvent| {
        let y = e.get_delta_y() as f32;
        radius.set((*radius.get() + y).clamp(0.0, 300.0));
    };

    render!(
        rect {
            overflow: "clip",
            height: "100%",
            width: "100%",
            padding: "60",
            onwheel: onwheel,
            rect {
                shadow: "0 0 25 0 rgb(0, 0, 0, 170)",
                corner_radius: "{radius} {radius * 0.7} {radius * 0.4} {radius * 0.2}",
                corner_smoothing: "1",
                height: "100%",
                width: "100%",
                background: "red",
                border: "7 solid white",
                border_align: "outer"
            }
        }
    )
}
