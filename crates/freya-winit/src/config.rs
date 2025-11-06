use std::{borrow::Cow, io::Cursor};

use freya_core::integration::*;
use freya_engine::prelude::Color;
use image::ImageReader;
use winit::window::{Icon, WindowAttributes};

use crate::plugins::{FreyaPlugin, PluginsManager};

pub type WindowBuilderHook = Box<dyn FnOnce(WindowAttributes) -> WindowAttributes + Send + Sync>;

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
    /// Enable Window resizable behaviour.
    pub(crate) resizable: bool,
    /// Icon for the Window.
    pub(crate) icon: Option<Icon>,
    /// Hook function called with the Window Attributes.
    pub(crate) window_attributes_hook: Option<WindowBuilderHook>,
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
            resizable: true,
            icon: None,
            window_attributes_hook: None,
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

    /// Is Window resizable.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Specify Window icon.
    pub fn with_icon(mut self, data: &[u8]) -> Self {
        let reader = ImageReader::new(Cursor::new(data))
            .with_guessed_format()
            .expect("cursor io never fails");
        let image = reader
            .decode()
            .expect("image format is supported")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).expect("image format is correct");
        self.icon = Some(icon);
        self
    }

    /// Register a Window Attributes hook.
    pub fn with_window_attributes(
        mut self,
        window_attributes_hook: impl FnOnce(WindowAttributes) -> WindowAttributes
        + 'static
        + Send
        + Sync,
    ) -> Self {
        self.window_attributes_hook = Some(Box::new(window_attributes_hook));
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
