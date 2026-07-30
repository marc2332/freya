use std::rc::Rc;

pub use mundy::{
    AccentColor,
    Srgba,
};
use torin::prelude::{
    Point2D,
    Size2D,
};

use crate::{
    accessibility::id::AccessibilityId,
    prelude::{
        State,
        consume_root_context,
    },
    user_event::UserEvent,
};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
pub enum NavigationMode {
    #[default]
    NotKeyboard,
    Keyboard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PreferredTheme {
    #[default]
    Light,
    Dark,
}

/// Access point to different Freya-managed states such as the focused node,
/// root window size, navigation mode, and theme preference.
///
/// Retrieve it from any component with [`Platform::get`].
#[derive(Clone)]
pub struct Platform {
    /// The [`AccessibilityId`] of the currently focused node.
    pub focused_accessibility_id: State<AccessibilityId>,
    /// The accessibility node data of the currently focused node.
    pub focused_accessibility_node: State<accesskit::Node>,
    /// The size of the root window, in logical units — the same space as measured (sized-event)
    /// areas, so the two can be compared directly.
    pub root_size: State<Size2D>,
    /// The window's outer position (top-left) on the desktop, in logical units — kept in the
    /// same space as [`root_size`](Self::root_size), updated on `WindowEvent::Moved`. Lets
    /// userland persist/restore where a window sits without reaching for the raw winit handle.
    pub window_position: State<Point2D>,
    /// Whether the window is **filled** — winit's `Window::is_maximized`: the frame grown to
    /// all the space the current monitor offers (macOS *zoom*). This is **not** native
    /// fullscreen, which is [`is_fullscreen`](Self::is_fullscreen).
    ///
    /// Refreshed on every resize. Companion to [`root_size`](Self::root_size) /
    /// [`window_position`](Self::window_position): a filled window's geometry is the monitor's,
    /// not a size the user chose, so an app that persists geometry has to tell the two apart.
    pub is_maximized: State<bool>,
    /// Whether the window is in **native fullscreen** (winit's `Window::fullscreen`) — the
    /// other state whose geometry isn't the user's own. See
    /// [`is_maximized`](Self::is_maximized).
    pub is_fullscreen: State<bool>,
    /// Rendering scale factor, the OS scale factor multiplied by the custom scale factor.
    pub scale_factor: State<f64>,
    /// Custom scale factor, change it with [`Platform::set_custom_scale_factor`].
    pub custom_scale_factor: State<f64>,
    /// The current [`NavigationMode`].
    pub navigation_mode: State<NavigationMode>,
    /// The OS-level [`PreferredTheme`].
    pub preferred_theme: State<PreferredTheme>,
    /// Whether the app currently has the OS-level focus.
    pub is_app_focused: State<bool>,
    /// The OS-level [`AccentColor`].
    pub accent_color: State<AccentColor>,
    /// Sender used to dispatch [`UserEvent`]s to the active renderer.
    pub sender: Rc<dyn Fn(UserEvent)>,
}

impl Platform {
    /// Retrieve the [`Platform`] from the root context.
    pub fn get() -> Self {
        consume_root_context()
    }

    /// Dispatch a [`UserEvent`] to the active renderer.
    pub fn send(&self, event: UserEvent) {
        (self.sender)(event)
    }

    /// Request the renderer to use a custom scale factor, multiplied with the
    /// OS scale factor. The value might get clamped to a reasonable range.
    pub fn set_custom_scale_factor(&self, custom_scale_factor: f64) {
        self.send(UserEvent::SetCustomScaleFactor(custom_scale_factor));
    }
}
