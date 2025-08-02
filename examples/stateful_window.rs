#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(LaunchConfig::new().with_default_font("Impact").with_window(
        WindowConfig::new_with_props(app, appProps { number: 10 }).with_title("Window with state"),
    ));
}

#[component]
fn app(number: i32) -> Element {
    rsx!(rect {
        width: "100%",
        height: "100%",
        main_align: "center",
        cross_align: "center",
        label {
            font_size: "50",
            "{number}"
        }
    })
}
