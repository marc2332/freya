#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

static FERRIS: &[u8] = include_bytes!("./ferris.svg");

fn main() {
    launch_with_props(app, "Opacity", (400.0, 350.0));
}

fn app() -> Element {
    let ferris = static_bytes_to_data(FERRIS);
    let mut opacity = use_signal(|| 70.0);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            rect {
                opacity: "{opacity / 100.0}",
                svg {
                    width: "100%",
                    height: "50%",
                    svg_data: ferris,
                }
                label {
                    text_align: "center",
                    width: "100%",
                    "Meet Ferris!"
                }
            }
            Slider {
                width: "100",
                value: *opacity.read(),
                onmoved: move |p| {
                    opacity.set(p);
                }
            }
            label {
                "Drag the slider"
            }
        }
    )
}
