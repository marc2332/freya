use std::rc::Rc;

pub use platform_preferences::{
    AccentColor,
    ScrollBarStyle,
};
use torin::prelude::Size2D;

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
    /// The size of the root window.
    pub root_size: State<Size2D>,
    /// The current [`NavigationMode`].
    pub navigation_mode: State<NavigationMode>,
    /// The OS-level [`PreferredTheme`].
    pub preferred_theme: State<PreferredTheme>,
    /// The OS-level [`ScrollBarStyle`].
    pub scrollbar_style: State<ScrollBarStyle>,
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
}
