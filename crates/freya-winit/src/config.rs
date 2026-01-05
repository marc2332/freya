use std::{
    borrow::Cow,
    fmt::Debug,
    future::Future,
    io::Cursor,
    pin::Pin,
};

use bytes::Bytes;
use freya_core::{
    integration::*,
    prelude::Color,
};
use image::ImageReader;
use winit::{
    event_loop::ActiveEventLoop,
    window::{
        Icon,
        Window,
        WindowAttributes,
    },
};

use crate::{
    plugins::{
        FreyaPlugin,
        PluginsManager,
    },
    renderer::LaunchProxy,
};

pub type WindowBuilderHook =
    Box<dyn FnOnce(WindowAttributes, &ActiveEventLoop) -> WindowAttributes + Send + Sync>;
pub type WindowHandleHook = Box<dyn FnOnce(&mut Window) + Send + Sync>;

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
    pub(crate) background: Color,
    /// Enable Window resizable behaviour.
    pub(crate) resizable: bool,
    /// Icon for the Window.
    pub(crate) icon: Option<Icon>,
    /// Hook function called with the Window Attributes.
    pub(crate) window_attributes_hook: Option<WindowBuilderHook>,
    /// Hook function called with the Window.
    pub(crate) window_handle_hook: Option<WindowHandleHook>,
}

impl Debug for WindowConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowConfig")
            .field("size", &self.size)
            .field("min_size", &self.min_size)
            .field("max_size", &self.max_size)
            .field("decorations", &self.decorations)
            .field("title", &self.title)
            .field("transparent", &self.transparent)
            .field("background", &self.background)
            .field("resizable", &self.resizable)
            .field("icon", &self.icon)
            .finish()
    }
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
            window_handle_hook: None,
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

    /// Specify the Window's background color.
    pub fn with_background(mut self, background: impl Into<Color>) -> Self {
        self.background = background.into();
        self
    }

    /// Is Window resizable.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Specify Window icon.
    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Register a Window Attributes hook.
    pub fn with_window_attributes(
        mut self,
        window_attributes_hook: impl FnOnce(WindowAttributes, &ActiveEventLoop) -> WindowAttributes
        + 'static
        + Send
        + Sync,
    ) -> Self {
        self.window_attributes_hook = Some(Box::new(window_attributes_hook));
        self
    }

    /// Register a Window handle hook.
    pub fn with_window_handle(
        mut self,
        window_handle_hook: impl FnOnce(&mut Window) + 'static + Send + Sync,
    ) -> Self {
        self.window_handle_hook = Some(Box::new(window_handle_hook));
        self
    }
}

pub type EmbeddedFonts = Vec<(Cow<'static, str>, Bytes)>;
#[cfg(feature = "tray")]
pub type TrayIconGetter = Box<dyn FnOnce() -> tray_icon::TrayIcon + Send>;
#[cfg(feature = "tray")]
pub type TrayHandler =
    Box<dyn FnMut(crate::tray_icon::TrayEvent, crate::renderer::RendererContext)>;

pub type TaskHandler =
    Box<dyn FnOnce(crate::renderer::LaunchProxy) -> Pin<Box<dyn Future<Output = ()>>> + 'static>;

/// Launch configuration.
pub struct LaunchConfig {
    pub(crate) windows_configs: Vec<WindowConfig>,
    #[cfg(feature = "tray")]
    pub(crate) tray: (Option<TrayIconGetter>, Option<TrayHandler>),
    pub(crate) plugins: PluginsManager,
    pub(crate) embedded_fonts: EmbeddedFonts,
    pub(crate) fallback_fonts: Vec<Cow<'static, str>>,
    pub(crate) tasks: Vec<TaskHandler>,
}

impl Default for LaunchConfig {
    fn default() -> Self {
        LaunchConfig {
            windows_configs: Vec::default(),
            #[cfg(feature = "tray")]
            tray: (None, None),
            plugins: PluginsManager::default(),
            embedded_fonts: Default::default(),
            fallback_fonts: default_fonts(),
            tasks: Vec::new(),
        }
    }
}

impl LaunchConfig {
    pub fn new() -> LaunchConfig {
        LaunchConfig::default()
    }

    pub fn window_icon(icon: &[u8]) -> Icon {
        let reader = ImageReader::new(Cursor::new(icon))
            .with_guessed_format()
            .expect("Cursor io never fails");
        let image = reader
            .decode()
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        Icon::from_rgba(rgba, width, height).expect("Failed to open icon")
    }

    #[cfg(feature = "tray")]
    pub fn tray_icon(icon: &[u8]) -> tray_icon::Icon {
        let reader = ImageReader::new(Cursor::new(icon))
            .with_guessed_format()
            .expect("Cursor io never fails");
        let image = reader
            .decode()
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        tray_icon::Icon::from_rgba(rgba, width, height).expect("Failed to open icon")
    }
}

impl LaunchConfig {
    /// Register a window configuration. You can call this multiple times.
    pub fn with_window(mut self, window_config: WindowConfig) -> Self {
        self.windows_configs.push(window_config);
        self
    }

    /// Register a tray icon and its handler.
    #[cfg(feature = "tray")]
    pub fn with_tray(
        mut self,
        tray_icon: impl FnOnce() -> tray_icon::TrayIcon + 'static + Send,
        tray_handler: impl FnMut(crate::tray_icon::TrayEvent, crate::renderer::RendererContext)
        + 'static,
    ) -> Self {
        self.tray = (Some(Box::new(tray_icon)), Some(Box::new(tray_handler)));
        self
    }

    /// Register a plugin.
    pub fn with_plugin(mut self, plugin: impl FreyaPlugin + 'static) -> Self {
        self.plugins.add_plugin(plugin);
        self
    }

    /// Embed a font.
    pub fn with_font(
        mut self,
        font_name: impl Into<Cow<'static, str>>,
        font: impl Into<Bytes>,
    ) -> Self {
        self.embedded_fonts.push((font_name.into(), font.into()));
        self
    }

    /// Register a fallback font. Will be used if the default fonts are not available.
    pub fn with_fallback_font(mut self, font_family: impl Into<Cow<'static, str>>) -> Self {
        self.fallback_fonts.push(font_family.into());
        self
    }

    /// Register a default font. Will be used if found.
    pub fn with_default_font(mut self, font_name: impl Into<Cow<'static, str>>) -> Self {
        self.fallback_fonts.insert(0, font_name.into());
        self
    }

    /// Register a single-thread launch task.
    /// The task receives a [LaunchProxy] that can be used to get access to [RendererContext](crate::renderer::RendererContext).
    /// The provided callback should return a `'static` future which will be scheduled on the renderer
    /// thread and polled until completion.
    pub fn with_future<F, Fut>(mut self, task: F) -> Self
    where
        F: FnOnce(LaunchProxy) -> Fut + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        self.tasks
            .push(Box::new(move |proxy| Box::pin(task(proxy))));
        self
    }
}
