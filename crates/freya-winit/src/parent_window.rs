//! Support for making one window a **child** of another at runtime.
//!
//! winit can only express this at creation time (`WindowAttributes::with_parent_window`,
//! which is `unsafe` and takes a raw handle), so an app that opens a shared utility window
//! (a settings or inspector panel) and re-points it at whichever window asked for it has no
//! API to reach for. This module is that API's platform half; the entry points are
//! [`RendererContext::set_window_parent`](crate::renderer::RendererContext::set_window_parent)
//! and [`WinitPlatformExt::set_window_parent`](crate::extensions::WinitPlatformExt::set_window_parent).
//!
//! On macOS the relationship is AppKit's `addChildWindow:ordered:`: the child is ordered
//! above its parent and cannot be covered by it, it travels with the parent (minimize, move,
//! space switch), and it is closed when the parent closes. The parent stays fully
//! interactive, which is what separates this from a modal sheet.
//!
//! Implemented only for macOS for now, and a no-op elsewhere: the Windows and X11 owner-window
//! relationships have the same shape but are set through a different handle, and there is no
//! Wayland equivalent.

use winit::window::Window;

/// Make `child` a child window of `parent`, ordered above it. Re-parenting is a single call:
/// a child that already belongs to another window leaves that one first.
pub(crate) fn set_parent(child: &Window, parent: &Window) {
    #[cfg(target_os = "macos")]
    macos::set_parent(child, parent);
    #[cfg(not(target_os = "macos"))]
    let _ = (child, parent);
}

/// Detach `child` from whatever window it is a child of, leaving it a plain top-level window.
/// Does nothing when it has no parent.
pub(crate) fn clear_parent(child: &Window) {
    #[cfg(target_os = "macos")]
    macos::clear_parent(child);
    #[cfg(not(target_os = "macos"))]
    let _ = child;
}

#[cfg(target_os = "macos")]
mod macos {
    use objc2::rc::Retained;
    use objc2_app_kit::{
        NSView,
        NSWindow,
        NSWindowOrderingMode,
    };
    use raw_window_handle::{
        HasWindowHandle,
        RawWindowHandle,
    };
    use winit::window::Window;

    /// Resolve the `NSWindow` behind a winit [`Window`]. `None` when the handle is not AppKit
    /// or the view is not in a window yet.
    fn ns_window(window: &Window) -> Option<Retained<NSWindow>> {
        let RawWindowHandle::AppKit(handle) = window.window_handle().ok()?.as_raw() else {
            return None;
        };
        let ns_view = unsafe { handle.ns_view.cast::<NSView>().as_ref() };
        ns_view.window()
    }

    pub(crate) fn set_parent(child: &Window, parent: &Window) {
        let (Some(child), Some(parent)) = (ns_window(child), ns_window(parent)) else {
            return;
        };
        // AppKit removes the child from a previous parent itself, but only when the two
        // differ; adding a child to the window that already owns it re-orders it, which is
        // harmless.
        unsafe { parent.addChildWindow_ordered(&child, NSWindowOrderingMode::Above) };
    }

    pub(crate) fn clear_parent(child: &Window) {
        let Some(child) = ns_window(child) else {
            return;
        };
        // The child knows its own parent, so detaching needs no id from the caller.
        if let Some(parent) = child.parentWindow() {
            parent.removeChildWindow(&child);
        }
    }
}
