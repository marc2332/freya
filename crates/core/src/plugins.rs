use dioxus_native_core::NodeId;
use freya_engine::prelude::{Canvas, FontCollection};
use torin::torin::Torin;
use winit::window::Window;

#[derive(Default)]
pub struct PluginsManager {
    plugins: Vec<Box<dyn FreyaPlugin>>,
}

impl PluginsManager {
    pub fn add_plugin(&mut self, plugin: impl FreyaPlugin + 'static) {
        self.plugins.push(Box::new(plugin))
    }

    pub fn send(&mut self, event: PluginEvent) {
        for plugin in &mut self.plugins {
            plugin.on_event(&event)
        }
    }
}

pub enum PluginEvent<'a> {
    WindowCreated(&'a Window),
    StartedRender(&'a Canvas, &'a FontCollection),
    FinishedRender(&'a Canvas, &'a FontCollection),
    StartedLayout(&'a Torin<NodeId>),
    FinishedLayout(&'a Torin<NodeId>),
}

pub trait FreyaPlugin {
    fn on_event(&mut self, event: &PluginEvent);
}
