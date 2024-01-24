#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::signals::use_signal;
use freya::prelude::*;
use std::rc::Rc;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_title("Performance Overlay Plugin")
            .with_width(700.)
            .with_height(500.)
            .with_plugin(PerformanceOverlayPlugin::default())
            .build(),
    )
}

fn app() -> Element {
    let mut values = use_signal(|| ["Hello, World!"].repeat(100));

    rsx!(
        Button {
            onclick: move |_| values.push("Bye, World!"),
            label {
                "Bye!"
            }
        }
        VirtualScrollView {
            theme: theme_with!(ScrollViewTheme {
                height: "fill".into(),
            }),
            length: values.read().len(),
            item_size: 25.0,
            builder_values: values.clone(),
            direction: "vertical",
            builder: Rc::new(move |(key, index, values)| {
                let values = values.unwrap();
                let value = values.read()[index];
                let background = if index % 2 == 0 {
                    "rgb(200, 200, 200)"
                } else {
                    "white"
                };
                rsx! {
                    rect {
                        key: "{key}",
                        background: "{background}",
                        label {
                            height: "25",
                            width: "100%",
                            "{index} {value}"
                        }
                    }
                }
            })
        }
    )
}
