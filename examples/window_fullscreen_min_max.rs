#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_core::prelude::PlatformInformation;

fn main() {
    launch(app);
}

fn app() -> Element {
    let platform = use_platform();
    let PlatformInformation {
        is_fullscreen,
        is_minimized,
        is_maximized,
        ..
    } = *use_platform_information().read();

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                onpress: move |_| platform.toggle_fullscreen_window(),
                label {
                    "Fullscreen ({is_fullscreen})"
                }
            }
            Button {
                onpress: move |_| platform.toggle_minimize_window(),
                label {
                    "Minimize ({is_minimized})"
                }
            }
            Button {
                onpress: move |_| platform.toggle_maximize_window(),
                label {
                    "Maximize ({is_maximized})"
                }
            }
        }
    )
}
