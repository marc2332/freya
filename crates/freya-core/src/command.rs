use std::{
    any::Any,
    borrow::Cow,
    fmt::Debug,
};

use bytes::Bytes;

use crate::prelude::AccessibilityFocusStrategy;

#[derive(Debug)]
pub enum AppCommand {
    /// Open a URL with the system's default browser.
    OpenUrl(String),
    /// Load a font at runtime.
    LoadFont {
        font_name: Cow<'static, str>,
        font_data: Bytes,
    },
    Exit,
    Erased(ErasedCommand),
}

#[derive(Debug)]
pub enum WindowCommand {
    RequestRedraw,
    /// Focus with the given strategy.
    FocusAccessibilityNode(AccessibilityFocusStrategy),
    /// Set a custom scale factor.
    SetCustomScaleFactor(f64),
    Erased(ErasedCommand),
}

pub struct ErasedCommand(pub Box<dyn Any>);

impl Debug for ErasedCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ErasedCommand")
    }
}

/// # Safety
/// The values are never sent, received or accessed by other threads other than the main thread.
/// This is needed to send `Rc<T>` and other non-Send and non-Sync values from WindowConfig
/// to the winit EventLoop
unsafe impl Send for ErasedCommand {}
unsafe impl Sync for ErasedCommand {}
