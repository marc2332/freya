#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_core::platform_state::PlatformInformation;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app() -> Element {
    let PlatformInformation { viewport_size, .. } = *use_platform_information().read();

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            label {
                font_size: "25",
                font_weight: "bold",
                "{viewport_size:?}"
            }
        }
    )
}
