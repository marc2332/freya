use freya_engine::prelude::Canvas;
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
    CanvasRendered(&'a Canvas),
}

pub trait FreyaPlugin {
    fn on_event(&mut self, event: &PluginEvent);
}
