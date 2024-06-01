use accesskit::NodeId as AccessibilityId;

/// State consumed by components and updated by the platform.
pub struct NativePlatformState {
    pub focused_id: AccessibilityId,
    pub preferred_theme: PreferredTheme,
    pub navigation_mode: NavigationMode,
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
