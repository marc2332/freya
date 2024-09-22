#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_core::{
    plugins::{
        FreyaPlugin,
        PluginEvent,
    },
    prelude::PluginHandle,
};

struct DummyPlugin;

impl FreyaPlugin for DummyPlugin {
    fn on_event(&mut self, event: &PluginEvent, _handle: PluginHandle) {
        if let PluginEvent::AfterRender { .. } = event {
            println!("The app just got rendered to the canvas.");
        }
    }
}

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_plugin(DummyPlugin)
            .with_size(250.0, 250.),
    )
}

fn app() -> Element {
    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            width: "100%",
            height: "100%",
            Button {
                label { "Hover me!" }
            }
        }
    )
}
