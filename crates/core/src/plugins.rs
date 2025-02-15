use freya_engine::prelude::{
    Canvas,
    FontCollection,
};
use freya_native_core::NodeId;
use torin::torin::Torin;
use winit::{
    event_loop::EventLoopProxy,
    window::Window,
};

use crate::{
    dom::FreyaDOM,
    event_loop_messages::EventLoopMessage,
    events::PlatformEvent,
};

#[derive(Clone)]
pub struct PluginHandle {
    pub proxy: EventLoopProxy<EventLoopMessage>,
}

impl PluginHandle {
    pub fn new(proxy: &EventLoopProxy<EventLoopMessage>) -> Self {
        Self {
            proxy: proxy.clone(),
        }
    }

    /// Emit a [PlatformEvent]. Useful to simulate certain events.
    pub fn send_platform_event(&self, event: PlatformEvent) {
        self.proxy
            .send_event(EventLoopMessage::PlatformEvent(event))
            .ok();
    }

    /// Emit a [EventLoopMessage].
    pub fn send_event_loop_event(&self, event: EventLoopMessage) {
        self.proxy.send_event(event).ok();
    }
}

/// Manages all loaded plugins.
#[derive(Default)]
pub struct PluginsManager {
    plugins: Vec<Box<dyn FreyaPlugin>>,
}

impl PluginsManager {
    pub fn add_plugin(&mut self, plugin: impl FreyaPlugin + 'static) {
        self.plugins.push(Box::new(plugin))
    }

    pub fn send(&mut self, event: PluginEvent, handle: PluginHandle) {
        for plugin in &mut self.plugins {
            plugin.on_event(&event, handle.clone())
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
    },

    /// After rendering the app to the Canvas.
    AfterRender {
        canvas: &'a Canvas,
        font_collection: &'a FontCollection,
        freya_dom: &'a FreyaDOM,
    },

    /// Before starting to measure the layout.
    StartedMeasuringLayout(&'a Torin<NodeId>),

    /// After measuring the layout.
    FinishedMeasuringLayout(&'a Torin<NodeId>),

    /// Before starting to process the queued events.
    StartedMeasuringEvents,

    /// After processing the queued events.
    FinishedMeasuringEvents,

    StartedUpdatingDOM,

    FinishedUpdatingDOM,
}

/// Skeleton for Freya plugins.
pub trait FreyaPlugin {
    /// React on events emitted by Freya.
    fn on_event(&mut self, event: &PluginEvent, handle: PluginHandle);
}
