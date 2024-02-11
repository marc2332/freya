use dioxus_native_core::NodeId;
use freya_dom::dom::FreyaDOM;
use freya_engine::prelude::{Canvas, FontCollection};
use torin::torin::Torin;
use winit::window::Window;

use crate::layout::Viewports;

/// Manages all loaded plugins.
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

/// Event emitted to Plugins.
pub enum PluginEvent<'a> {
    /// The Window just got created.
    WindowCreated(&'a Window),

    /// Before starting to render the app to the Canvas.
    BeforeRender {
        canvas: &'a Canvas,
        font_collection: &'a FontCollection,
        freya_dom: &'a FreyaDOM,
        viewports: &'a Viewports,
    },

    /// After rendering the app to the Canvas.
    AfterRender {
        canvas: &'a Canvas,
        font_collection: &'a FontCollection,
        freya_dom: &'a FreyaDOM,
        viewports: &'a Viewports,
    },

    /// Before starting to measure the layout.
    StartedLayout(&'a Torin<NodeId>),

    /// After measuring the layout.
    FinishedLayout(&'a Torin<NodeId>),
}

/// Skeleton for Freya plugins.
pub trait FreyaPlugin {
    /// React on events emitted by Freya.
    fn on_event(&mut self, event: &PluginEvent);
}
