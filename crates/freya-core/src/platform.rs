use std::{
    borrow::Cow,
    cell::RefCell,
    rc::Rc,
};

use bytes::Bytes;
pub use mundy::{
    AccentColor,
    Srgba,
};
use rustc_hash::FxHashMap;
use torin::prelude::Size2D;

use crate::{
    accessibility::id::AccessibilityId,
    current_context::CurrentContext,
    prelude::{
        GlobalContexts,
        State,
        try_consume_root_context,
    },
    user_event::{
        GlobalUserEvent,
        UserEvent,
    },
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
    /// Google Android.
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

    /// Detects the operating system of the compile target.
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

/// Application-wide platform api.
#[derive(Clone)]
pub struct Platform {
    sender: Rc<dyn Fn(GlobalUserEvent)>,
    windows: Rc<RefCell<FxHashMap<u64, PlatformWindow>>>,
}

impl Platform {
    pub fn new(sender: impl Fn(GlobalUserEvent) + 'static) -> Self {
        Self {
            sender: Rc::new(sender),
            windows: Rc::default(),
        }
    }

    #[track_caller]
    pub fn get() -> Self {
        GlobalContexts::get().get_context()
    }

    /// Returns the live window with this ID, panicking if it has closed or does not exist.
    #[track_caller]
    pub fn window(&self, window_id: impl Into<u64>) -> PlatformWindow {
        let window_id = window_id.into();
        self.try_window(window_id)
            .unwrap_or_else(|| panic!("Window {window_id} does not exist or has closed."))
    }

    /// Returns the live window with this ID, or `None` if it has closed or does not exist.
    pub fn try_window(&self, window_id: impl Into<u64>) -> Option<PlatformWindow> {
        self.windows.borrow().get(&window_id.into()).cloned()
    }

    /// Returns the window from whose component, event handler, or scoped task this is called.
    #[track_caller]
    pub fn current_window(&self) -> PlatformWindow {
        let window_id: CurrentWindowId = CurrentContext::try_with(|_| try_consume_root_context())
            .flatten()
            .expect(concat!(
                "Platform::current_window() requires a Freya window context. ",
                "In `spawn_global`, use `Platform::get().window(window_id)` or capture ",
                "the current window before spawning."
            ));
        self.window(window_id.0)
    }

    pub fn register_window(&self, window: PlatformWindow) {
        self.windows.borrow_mut().insert(window.id, window);
    }

    pub fn unregister_window(&self, window_id: impl Into<u64>) {
        let _removed_window = self.windows.borrow_mut().remove(&window_id.into());
    }

    pub fn send(&self, event: GlobalUserEvent) {
        (self.sender)(event)
    }

    /// Open a URL with the system's default browser.
    pub fn open_url(&self, url: impl Into<String>) {
        self.send(GlobalUserEvent::OpenUrl(url.into()));
    }

    /// Request application exit.
    pub fn exit(&self) {
        self.send(GlobalUserEvent::Exit);
    }

    /// Load a font at runtime in every window.
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
        self.send(GlobalUserEvent::LoadFont {
            font_name: font_name.into(),
            font_data: font_data.into(),
        });
    }
}

/// The ID of the window whose component context is currently executing.
#[derive(Clone, Copy)]
pub struct CurrentWindowId(pub u64);

/// State and APIs for a given window.
///
/// # Example
///
/// ```rust,no_run
/// use freya_core::prelude::*;
///
/// fn app() -> impl IntoElement {
///     let window = Platform::get().current_window();
///     let root_size = *window.root_size.read();
///     let preferred_theme = *window.preferred_theme.read();
///
///     format!(
///         "Window: {:.0} × {:.0}, preferred theme: {preferred_theme:?}",
///         root_size.width, root_size.height,
///     )
/// }
/// ```
#[derive(Clone)]
pub struct PlatformWindow {
    pub id: u64,
    /// The [`AccessibilityId`] of the currently focused node in this window.
    pub focused_accessibility_id: State<AccessibilityId>,
    /// Accessibility data for this window's currently focused node.
    pub focused_accessibility_node: State<accesskit::Node>,
    /// This window's logical size.
    pub root_size: State<Size2D>,
    /// This window's effective rendering scale factor.
    pub scale_factor: State<f64>,
    /// The custom scale factor, changed through [`Self::set_custom_scale_factor`].
    pub custom_scale_factor: State<f64>,
    /// The current [`NavigationMode`] in this window.
    pub navigation_mode: State<NavigationMode>,
    /// The color theme preferred by the operating system for this window.
    pub preferred_theme: State<PreferredTheme>,
    /// Whether this window has operating-system focus.
    pub is_app_focused: State<bool>,
    /// The operating system's accent color, when available.
    pub accent_color: State<AccentColor>,
    /// Dispatches [`UserEvent`]s to this window's renderer.
    pub sender: Rc<dyn Fn(UserEvent)>,
}

impl PlatformWindow {
    pub fn send(&self, event: UserEvent) {
        (self.sender)(event)
    }

    /// Request a render of this window.
    pub fn request_redraw(&self) {
        self.send(UserEvent::RequestRedraw);
    }

    /// Request a custom rendering scale factor for this window.
    pub fn set_custom_scale_factor(&self, custom_scale_factor: f64) {
        self.send(UserEvent::SetCustomScaleFactor(custom_scale_factor));
    }
}
