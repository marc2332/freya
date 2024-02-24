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
    let image_data = static_bytes_to_data(RUST_LOGO);
    let mut size = use_signal(|| 250);

    let onwheel = move |e: WheelEvent| {
        let y = e.get_delta_y() as i32;
        let res = *size.read() + y;
        if res >= 600 || res <= 20 {
            return;
        }
        size.set(res);
    };

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
            main_align: "center",
            cross_align: "center",
            onwheel: onwheel,
            image {
                image_data: image_data,
                width: "{size}",
                height: "{size}",
            }
        }
    )
}
