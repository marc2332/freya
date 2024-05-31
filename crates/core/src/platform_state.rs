use accesskit::NodeId as AccessibilityId;
use torin::prelude::Size2D;
use winit::dpi::PhysicalSize;

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
}

impl PlatformInformation {
    pub fn from_winit(physical_size: PhysicalSize<u32>) -> Self {
        Self {
            viewport_size: Size2D::new(physical_size.width as f32, physical_size.height as f32),
        }
    }

    pub fn new(viewport_size: Size2D) -> Self {
        Self { viewport_size }
    }
}
