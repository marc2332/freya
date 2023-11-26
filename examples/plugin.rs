#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_core::plugins::{FreyaPlugin, PluginEvent};

struct DummyPlugin;

impl FreyaPlugin for DummyPlugin {
    fn on_event(&mut self, event: &PluginEvent) {
        match event {
            PluginEvent::CanvasRendered(_canvas, _font_collection) => {
                println!("rendered");
            }
            _ => {}
        }
    }
}

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_plugin(DummyPlugin)
            .build(),
    )
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
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
            Button {
                onclick: move |_| count += 1,
                label { "Increase" }
            }
            Button {
                onclick: move |_| count -= 1,
                label { "Decrease" }
            }
        }
    )
}
