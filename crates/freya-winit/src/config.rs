use std::borrow::Cow;

use freya_core::integration::*;
use freya_engine::prelude::Color;

use crate::plugins::{
    FreyaPlugin,
    PluginsManager,
};

/// Configuration for a Window.
pub struct WindowConfig {
    /// Root component for the window app.
    pub(crate) app: FpRender,
    /// Size of the Window.
    pub(crate) size: (f64, f64),
    /// Minimum size of the Window.
    pub(crate) min_size: Option<(f64, f64)>,
    /// Maximum size of the Window.
    pub(crate) max_size: Option<(f64, f64)>,
    /// Enable Window decorations.
    pub(crate) decorations: bool,
    /// Title for the Window.
    pub(crate) title: &'static str,
    /// Make the Window transparent or not.
    pub(crate) transparent: bool,
    /// Background color of the Window.
    /// TODO: Actually use it
    #[allow(unused)]
    pub(crate) background: Color,
}

impl WindowConfig {
    /// Create a window with the given app.
    pub fn new(app: impl Into<FpRender>) -> Self {
        Self::new_with_defaults(app.into())
    }

    fn new_with_defaults(app: impl Into<FpRender>) -> Self {
        Self {
            app: app.into(),
            size: (700.0, 500.0),
            min_size: None,
            max_size: None,
            decorations: true,
            title: "Freya",
            transparent: false,
            background: Color::WHITE,
        }
    }

    /// Specify a Window size.
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = (width, height);
        self
    }

    /// Specify a minimum Window size.
    pub fn with_min_size(mut self, min_width: f64, min_height: f64) -> Self {
        self.min_size = Some((min_width, min_height));
        self
    }

    /// Specify a maximum Window size.
    pub fn with_max_size(mut self, max_width: f64, max_height: f64) -> Self {
        self.max_size = Some((max_width, max_height));
        self
    }

    /// Whether the Window will have decorations or not.
    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    /// Specify the Window title.
    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    /// Make the Window transparent or not.
    pub fn with_transparency(mut self, transparency: bool) -> Self {
        self.transparent = transparency;
        self
    }
}

/// Launch configuration.
pub struct LaunchConfig {
    pub(crate) windows_configs: Vec<WindowConfig>,
    pub(crate) plugins: PluginsManager,
    pub(crate) fallback_fonts: Vec<Cow<'static, str>>,
}

impl Default for LaunchConfig {
    fn default() -> Self {
        LaunchConfig {
            windows_configs: Vec::default(),
            plugins: PluginsManager::default(),
            fallback_fonts: default_fonts(),
        }
    }
}

impl LaunchConfig {
    pub fn new() -> LaunchConfig {
        LaunchConfig::default()
    }
}

impl LaunchConfig {
    /// Register a window configuration. You can call this multiple times.
    pub fn with_window(mut self, window_config: WindowConfig) -> Self {
        self.windows_configs.push(window_config);
        self
    }

    /// Register a plugin.
    pub fn with_plugin(mut self, plugin: impl FreyaPlugin + 'static) -> Self {
        self.plugins.add_plugin(plugin);
        self
    }

    /// Register a fallback font. Will be used if the default fonts are not available.
    pub fn with_fallback_font(mut self, font_family: impl Into<Cow<'static, str>>) -> Self {
        self.fallback_fonts.push(font_family.into());
        self
    }
}
