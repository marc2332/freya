#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_title("Devtools Plugin")
            .with_size(700., 500.)
            .with_plugin(freya_devtools::DevtoolsPlugin::default()),
    )
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            label {
                font_size: "75",
                font_weight: "bold",
                "{count}"
            }
        }
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            spacing: "8",
            Button {
                onpress: move |_| count += 1,
                label { "Increase" }
            }
            Button {
                onpress: move |_| count -= 1,
                label { "Decrease" }
            }
        }
    )
}
