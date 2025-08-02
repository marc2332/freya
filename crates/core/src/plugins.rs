use std::{
    cell::RefCell,
    rc::Rc,
};

use freya_engine::prelude::{
    Canvas,
    FontCollection,
};
use winit::{
    event_loop::EventLoopProxy,
    window::{
        Window,
        WindowId,
    },
};

use crate::{
    dom::FreyaDOM,
    event_loop_messages::{
        EventLoopMessage,
        EventLoopMessageAction,
    },
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
    pub fn send_platform_event(&self, event: PlatformEvent, window_id: WindowId) {
        self.proxy
            .send_event(EventLoopMessage {
                window_id: Some(window_id),
                action: EventLoopMessageAction::PlatformEvent(event),
            })
            .ok();
    }

    /// Emit a [EventLoopMessage].
    pub fn send_event_loop_event(&self, event: EventLoopMessage) {
        self.proxy.send_event(event).ok();
    }
}

/// Manages all loaded plugins.
#[derive(Default, Clone)]
pub struct PluginsManager {
    plugins: Rc<RefCell<Vec<Box<dyn FreyaPlugin>>>>,
}

impl PluginsManager {
    pub fn add_plugin(&mut self, plugin: impl FreyaPlugin + 'static) {
        self.plugins.borrow_mut().push(Box::new(plugin))
    }

    pub fn send(&mut self, event: PluginEvent, handle: PluginHandle) {
        for plugin in self.plugins.borrow_mut().iter_mut() {
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
        window: &'a Window,
        canvas: &'a Canvas,
        font_collection: &'a FontCollection,
        fdom: &'a FreyaDOM,
    },

    /// After rendering the app to the Canvas.
    AfterRender {
        window: &'a Window,
        canvas: &'a Canvas,
        font_collection: &'a FontCollection,
        fdom: &'a FreyaDOM,
    },

    /// Before starting to measure the layout.
    StartedMeasuringLayout {
        window: &'a Window,
        fdom: &'a FreyaDOM,
    },

    /// After measuring the layout.
    FinishedMeasuringLayout {
        window: &'a Window,
        fdom: &'a FreyaDOM,
    },

    /// Before starting to process the queued events.
    StartedMeasuringEvents {
        window: &'a Window,
        fdom: &'a FreyaDOM,
    },

    /// After processing the queued events.
    FinishedMeasuringEvents {
        window: &'a Window,
        fdom: &'a FreyaDOM,
    },

    StartedUpdatingDOM {
        window: &'a Window,
        fdom: &'a FreyaDOM,
    },

    FinishedUpdatingDOM {
        window: &'a Window,
        fdom: &'a FreyaDOM,
    },
}

/// Skeleton for Freya plugins.
pub trait FreyaPlugin {
    /// React on events emitted by Freya.
    fn on_event(&mut self, event: &PluginEvent, handle: PluginHandle);
}
