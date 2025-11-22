use std::{
    any::Any,
    fmt::Debug,
};

use cursor_icon::CursorIcon;

use crate::prelude::AccessibilityFocusStrategy;

#[derive(Debug)]
pub enum UserEvent {
    RequestRedraw,

    /// Focus with the given strategy
    FocusAccessibilityNode(AccessibilityFocusStrategy),

    /// Set a new cursor icon.
    SetCursorIcon(CursorIcon),

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
