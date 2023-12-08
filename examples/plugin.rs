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
            PluginEvent::AfterRender(_canvas, _font_collection) => {
                println!("The app just got rendered to the canvas.");
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
            .with_width(250.0)
            .with_height(200.0)
            .build(),
    )
}

fn app(cx: Scope) -> Element {
    render!(
        rect { main_align: "center", cross_align: "center", width: "100%", height: "100%",
            Button { label { "Hover me!" } }
        }
    )
}
