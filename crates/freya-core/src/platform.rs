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
    current_context::CurrentContext,
    prelude::{
        State,
        consume_root_context,
        try_consume_root_context,
    },
    user_event::UserEvent,
};

/// How the user is navigating the application.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
pub enum NavigationMode {
    /// Navigation is driven by a pointing device or touch input.
    #[default]
    NotKeyboard,
    /// Navigation is driven by keyboard input.
    Keyboard,
}

/// The color theme preferred by the operating system.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PreferredTheme {
    /// The operating system prefers a light color theme.
    #[default]
    Light,
    /// The operating system prefers a dark color theme.
    Dark,
}

/// Operating system targeted by an application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetPlatform {
    /// Microsoft Windows.
    Windows,
    /// Apple macOS.
    MacOs,
    /// Linux.
    Linux,
    /// Android.
    Android,
    /// Apple iOS.
    Ios,
    /// An operating system Freya does not recognize.
    Unknown,
}

impl TargetPlatform {
    /// Returns the target platform from the current Freya context, or detects the compile target.
    pub fn get() -> Self {
        CurrentContext::try_with(|_| try_consume_root_context())
            .flatten()
            .unwrap_or_else(Self::detect)
    }

    /// Detects the operating system of the current compile target.
    pub fn detect() -> Self {
        if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "macos") {
            Self::MacOs
        } else if cfg!(target_os = "linux") {
            Self::Linux
        } else if cfg!(target_os = "android") {
            Self::Android
        } else if cfg!(target_os = "ios") {
            Self::Ios
        } else {
            Self::Unknown
        }
    }

    /// Returns whether this is a desktop operating system.
    pub fn is_desktop(&self) -> bool {
        matches!(self, Self::Windows | Self::MacOs | Self::Linux)
    }

    /// Returns whether this is a mobile operating system.
    pub fn is_mobile(&self) -> bool {
        matches!(self, Self::Android | Self::Ios)
    }
}

/// Reactive state and APIs provided by Freya for the current window.
///
/// Retrieve it from a component with [`Platform::get`]. Each state field is a [`State`]. Calling
/// [`State::read`] subscribes the component to changes, so use it when the UI should react to a
/// platform update.
///
/// # Example
///
/// ```rust,no_run
/// use freya_core::prelude::*;
///
/// fn app() -> impl IntoElement {
///     let platform = Platform::get();
///     let root_size = *platform.root_size.read();
///     let preferred_theme = *platform.preferred_theme.read();
///
///     format!(
///         "Window: {:.0} × {:.0}, preferred theme: {preferred_theme:?}",
///         root_size.width, root_size.height,
///     )
/// }
/// ```

#[derive(Clone)]
pub struct Platform {
    /// The [`AccessibilityId`] of the node currently focused for accessibility.
    pub focused_accessibility_id: State<AccessibilityId>,
    /// Accessibility data for the node currently focused for accessibility.
    pub focused_accessibility_node: State<accesskit::Node>,
    /// The root window's logical size.
    pub root_size: State<Size2D>,
    /// The effective rendering scale factor, including the operating system and custom factors.
    pub scale_factor: State<f64>,
    /// The custom scale factor. Use [`Platform::set_custom_scale_factor`] to change it.
    pub custom_scale_factor: State<f64>,
    /// The current input [`NavigationMode`].
    pub navigation_mode: State<NavigationMode>,
    /// The color theme preferred by the operating system.
    pub preferred_theme: State<PreferredTheme>,
    /// Whether the application currently has operating-system focus.
    pub is_app_focused: State<bool>,
    /// The accent color supplied by the operating system, when available.
    pub accent_color: State<AccentColor>,
    /// Dispatches [`UserEvent`] values to the active renderer.
    pub sender: Rc<dyn Fn(UserEvent)>,
}

impl Platform {
    /// Returns the [`Platform`] for the current window.
    ///
    /// This must be called while a Freya component is rendering or handling an event.
    #[track_caller]
    pub fn get() -> Self {
        consume_root_context()
    }

    /// Dispatches a [`UserEvent`] to the active renderer.
    pub fn send(&self, event: UserEvent) {
        (self.sender)(event)
    }

    /// Requests a custom rendering scale factor.
    ///
    /// The effective scale factor is this value multiplied by the operating system scale factor.
    /// Freya clamps the requested value to its supported range.
    pub fn set_custom_scale_factor(&self, custom_scale_factor: f64) {
        self.send(UserEvent::SetCustomScaleFactor(custom_scale_factor));
    }

    /// Loads a font at runtime under the given family name in all windows.
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
