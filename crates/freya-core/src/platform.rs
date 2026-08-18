use std::{
    borrow::Cow,
    rc::Rc,
};

use bytes::Bytes;
pub use mundy::{
    AccentColor,
    Srgba,
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

    /// Load a font at runtime, making it available under the given name in all windows.
    ///
    /// Names are matched exactly. Loading more fonts under one name adds them as styles of
    /// that family, so a name cannot be reassigned.
    ///
    /// Runtime fonts are never used as fallbacks.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya_core::prelude::*;
    ///
    /// fn load_rubik() {
    ///     let font = std::fs::read("./Rubik.ttf").expect("Failed to read the font file.");
    ///     Platform::get().load_font("Rubik", font);
    /// }
    /// ```
    pub fn load_font(&self, font_name: impl Into<Cow<'static, str>>, font_data: impl Into<Bytes>) {
        self.send(UserEvent::LoadFont {
            font_name: font_name.into(),
            font_data: font_data.into(),
        });
    }
}
