#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        LaunchConfig::new()
            .with_window(WindowConfig::new(app).with_title("Main window"))
            .with_window(WindowConfig::new(tiny_window).with_size(100., 100.)),
    );
}

fn app() -> Element {
    let platform = use_platform();

    let onpress = move |_| {
        platform.new_window(
            WindowConfig::new_with_props(another_window, another_windowProps { value: 123 })
                .with_title("Another window"),
        )
    };

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "white",
            color: "white",
            Button {
                onpress,
                label { "New window" }
            }
        }
    )
}

#[component]
fn another_window(value: i32) -> Element {
    let platform = use_platform();

    let onpress = move |_| platform.close_window();

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "white",
            font_size: "30",
            label {
                "Value: {value}"
            }
            Button {
                onpress,
                label { "Close" }
            }
        }
    )
}

fn tiny_window() -> Element {
    rsx!(
        label {
            "Just a tiny window!"
        }
    )
}
