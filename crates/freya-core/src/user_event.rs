use std::{
    any::Any,
    borrow::Cow,
    fmt::Debug,
};

use bytes::Bytes;

use crate::prelude::AccessibilityFocusStrategy;

#[derive(Debug)]
pub enum UserEvent {
    RequestRedraw,

    /// Focus with the given strategy
    FocusAccessibilityNode(AccessibilityFocusStrategy),

    /// Open an url with whatever the platform uses to browse the web.
    OpenUrl(String),

    /// Set a custom scale factor.
    SetCustomScaleFactor(f64),

    /// Load a font at runtime.
    LoadFont {
        font_name: Cow<'static, str>,
        font_data: Bytes,
    },

    Erased(SingleThreadErasedEvent),
}

pub struct SingleThreadErasedEvent(pub Box<dyn Any>);

impl Debug for SingleThreadErasedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SingleThreadErasedEvent")
    }
}

/// # Safety
/// The values are never sent, received or accessed by other threads other than the main thread.
/// This is needed to send `Rc<T>` and other non-Send and non-Sync values from WindowConfig
/// to the winit EventLoop
unsafe impl Send for SingleThreadErasedEvent {}
unsafe impl Sync for SingleThreadErasedEvent {}
