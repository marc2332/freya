#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

fn app(cx: Scope) -> Element {
    let image_data = bytes_to_data(cx, RUST_LOGO);
    let mut size = use_state(cx, || 150);

    let onwheel = move |e: WheelEvent| {
        let y = e.get_delta_y();
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
                image_data: image_data,
                width: "{size}",
                height: "{size}",
            }
        }
    )
}
