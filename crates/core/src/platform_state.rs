use accesskit::NodeId as AccessibilityId;
use torin::prelude::Size2D;
use winit::window::Window;

/// State consumed by components and updated by the platform.
#[derive(Clone)]
pub struct NativePlatformState {
    pub focused_id: AccessibilityId,
    pub preferred_theme: PreferredTheme,
    pub navigation_mode: NavigationMode,
    pub information: PlatformInformation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PreferredTheme {
    #[default]
    /// Use the light variant.
    Light,

    /// Use the dark variant.
    Dark,
}

impl From<winit::window::Theme> for PreferredTheme {
    fn from(value: winit::window::Theme) -> Self {
        match value {
            winit::window::Theme::Light => Self::Light,
            winit::window::Theme::Dark => Self::Dark,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationMode {
    #[default]
    NotKeyboard,

    Keyboard,
}

/// Information about the platform.
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct PlatformInformation {
    pub viewport_size: Size2D,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
}

impl PlatformInformation {
    pub fn from_winit(
        winit: &Window
    ) -> Self {
        let window_size = winit.inner_size();
        Self {
            viewport_size: Size2D::new(window_size.width as f32, window_size.height as f32),
            is_minimized: winit.is_minimized().unwrap_or_default(),
            is_maximized: winit.is_maximized(),
            is_fullscreen: winit.fullscreen().is_some()
        }
    }

    pub fn new(
        viewport_size: Size2D,
        is_minimized: bool,
        is_maximized: bool,
        is_fullscreen: bool,
    ) -> Self {
        Self {
            viewport_size,
            is_minimized,
            is_maximized,
            is_fullscreen,
        }
    }
}
