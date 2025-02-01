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
    let image_data = static_bytes(RUST_LOGO);

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
            main_align: "center",

            for sampling in [
                "nearest",
                "bilinear",
                "trilinear",
                "mitchell",
                "catmull-rom"
            ] {
                rect {
                    direction: "horizontal",
                    spacing: "12",
                    main_align: "center",
                    cross_align: "center",

                    image {
                        image_data: image_data.clone(),
                        width: "96",
                        height: "96",
                        cache_key: "{sampling}",
                        sampling,
                    }

                    label {
                        "Sampling mode: {sampling}"
                    }
                }
            }
        }
    )
}
