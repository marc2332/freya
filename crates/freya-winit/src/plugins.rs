use std::{
    cell::RefCell,
    rc::Rc,
};

use freya_core::integration::*;
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

use crate::renderer::{
    NativeEvent,
    NativeWindowEvent,
    NativeWindowEventAction,
};

#[derive(Clone)]
pub struct PluginHandle {
    pub proxy: EventLoopProxy<NativeEvent>,
}

impl PluginHandle {
    pub fn new(proxy: &EventLoopProxy<NativeEvent>) -> Self {
        Self {
            proxy: proxy.clone(),
        }
    }

    /// Emit a [PlatformEvent]. Useful to simulate certain events.
    pub fn send_platform_event(&self, event: PlatformEvent, window_id: WindowId) {
        self.proxy
            .send_event(NativeEvent::Window(NativeWindowEvent {
                window_id,
                action: NativeWindowEventAction::PlatformEvent(event),
            }))
            .ok();
    }

    /// Emit a [NativeEvent].
    pub fn send_event_loop_event(&self, event: NativeEvent) {
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
    /// A Window just got created.
    WindowCreated {
        window: &'a Window,
        font_collection: &'a FontCollection,
        tree: &'a Tree,
        animation_clock: &'a AnimationClock,
    },

    /// A Window just got closed.
    WindowClosed {
        window: &'a Window,
        tree: &'a Tree,
    },

    /// After having rendered, presented and everything else.
    AfterRedraw {
        window: &'a Window,
        font_collection: &'a FontCollection,
        tree: &'a Tree,
    },

    /// Before presenting the canvas to the window.
    BeforePresenting {
        window: &'a Window,
        font_collection: &'a FontCollection,
        tree: &'a Tree,
    },

    /// After presenting the canvas to the window.
    AfterPresenting {
        window: &'a Window,
        font_collection: &'a FontCollection,
        tree: &'a Tree,
    },

    /// Before starting to render the app to the Canvas.
    BeforeRender {
        window: &'a Window,
        canvas: &'a Canvas,
        font_collection: &'a FontCollection,
        tree: &'a Tree,
    },

    /// After rendering the app to the Canvas.
    AfterRender {
        window: &'a Window,
        canvas: &'a Canvas,
        font_collection: &'a FontCollection,
        tree: &'a Tree,
        animation_clock: &'a AnimationClock,
    },

    /// Before starting to measure the layout.
    StartedMeasuringLayout {
        window: &'a Window,
        tree: &'a Tree,
    },

    /// After measuring the layout.
    FinishedMeasuringLayout {
        window: &'a Window,
        tree: &'a Tree,
    },

    /// Before starting to process the queued events.
    StartedMeasuringEvents {
        window: &'a Window,
        tree: &'a Tree,
    },

    /// After processing the queued events.
    FinishedMeasuringEvents {
        window: &'a Window,
        tree: &'a Tree,
    },

    StartedUpdatingTree {
        window: &'a Window,
        tree: &'a Tree,
    },

    FinishedUpdatingTree {
        window: &'a Window,
        tree: &'a Tree,
    },

    BeforeAccessibility {
        window: &'a Window,
        font_collection: &'a FontCollection,
        tree: &'a Tree,
    },

    AfterAccessibility {
        window: &'a Window,
        font_collection: &'a FontCollection,
        tree: &'a Tree,
    },
}

/// Skeleton for Freya plugins.
pub trait FreyaPlugin {
    /// React on events emitted by Freya.
    fn on_event(&mut self, event: &PluginEvent, handle: PluginHandle);
}
