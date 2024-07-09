use std::{io::Cursor, sync::Arc};

use freya_core::{
    plugins::{FreyaPlugin, PluginsManager},
    style::default_fonts,
};
use freya_engine::prelude::Color;
use freya_node_state::Parse;
use image::io::Reader;
use winit::window::{Icon, Window, WindowAttributes};

pub type WindowBuilderHook = Box<dyn Fn(WindowAttributes) -> WindowAttributes>;
pub type EmbeddedFonts<'a> = Vec<(&'a str, &'a [u8])>;

/// Configuration for a Window.
pub struct WindowConfig {
    /// Size of the Window.
    pub size: (f64, f64),
    /// Minimum size of the Window.
    pub min_size: Option<(f64, f64)>,
    /// Maximum size of the Window.
    pub max_size: Option<(f64, f64)>,
    /// Enable Window decorations.
    pub decorations: bool,
    /// Title for the Window.
    pub title: &'static str,
    /// Make the Window transparent or not.
    pub transparent: bool,
    /// Background color of the Window.
    pub background: Color,
    /// The Icon of the Window.
    pub icon: Option<Icon>,
    /// Setup callback.
    pub on_setup: Option<WindowCallback>,
    /// Exit callback.
    pub on_exit: Option<WindowCallback>,
    /// Hook function called with the Window Attributes.
    pub window_attributes_hook: Option<WindowBuilderHook>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            size: (600.0, 600.0),
            min_size: None,
            max_size: None,
            decorations: true,
            title: "Freya app",
            transparent: false,
            background: Color::WHITE,
            icon: None,
            on_setup: None,
            on_exit: None,
            window_attributes_hook: None,
        }
    }
}

/// Launch configuration.
pub struct LaunchConfig<'a, T: Clone = ()> {
    pub state: Option<T>,
    pub window_config: WindowConfig,
    pub embedded_fonts: EmbeddedFonts<'a>,
    pub plugins: PluginsManager,
    pub default_fonts: Vec<String>,
}

impl<'a, T: Clone> Default for LaunchConfig<'a, T> {
    fn default() -> Self {
        Self {
            state: None,
            window_config: Default::default(),
            embedded_fonts: Default::default(),
            plugins: Default::default(),
            default_fonts: default_fonts(),
        }
    }
}

impl<'a, T: Clone> LaunchConfig<'a, T> {
    pub fn new() -> LaunchConfig<'a, T> {
        LaunchConfig::default()
    }
}

impl LaunchConfig<'_, ()> {
    pub fn load_icon(icon: &[u8]) -> Icon {
        let reader = Reader::new(Cursor::new(icon))
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
}

pub type WindowCallback = Arc<Box<fn(&mut Window)>>;

impl<'a, T: Clone> LaunchConfig<'a, T> {
    /// Specify a Window size.
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.window_config.size = (width, height);
        self
    }

    /// Specify a minimum Window size.
    pub fn with_min_size(mut self, min_width: f64, min_height: f64) -> Self {
        self.window_config.min_size = Some((min_width, min_height));
        self
    }

    /// Specify a maximum Window size.
    pub fn with_max_size(mut self, max_width: f64, max_height: f64) -> Self {
        self.window_config.max_size = Some((max_width, max_height));
        self
    }

    /// Whether the Window will have decorations or not.
    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.window_config.decorations = decorations;
        self
    }

    /// Specify the Window title.
    pub fn with_title(mut self, title: &'static str) -> Self {
        self.window_config.title = title;
        self
    }

    /// Make the Window transparent or not.
    pub fn with_transparency(mut self, transparency: bool) -> Self {
        self.window_config.transparent = transparency;
        self
    }

    /// Pass a custom value that your app will consume.
    pub fn with_state(mut self, state: T) -> Self {
        self.state = Some(state);
        self
    }

    /// Specify the Window background color.
    pub fn with_background(mut self, background: &str) -> Self {
        self.window_config.background = Color::parse(background).unwrap_or(Color::WHITE);
        self
    }

    /// Embed a font.
    pub fn with_font(mut self, font_name: &'a str, font: &'a [u8]) -> Self {
        self.embedded_fonts.push((font_name, font));
        self
    }

    /// Clear default fonts.
    pub fn without_default_fonts(mut self) -> Self {
        self.default_fonts.clear();
        self
    }

    /// Regiter a default font.
    pub fn with_default_font(mut self, font_name: &str) -> Self {
        self.default_fonts.push(font_name.to_string());
        self
    }

    /// Specify the Window icon.
    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.window_config.icon = Some(icon);
        self
    }

    /// Register a callback that will be executed when the window is created.
    pub fn on_setup(mut self, callback: fn(&mut Window)) -> Self {
        self.window_config.on_setup = Some(Arc::new(Box::new(callback)));
        self
    }

    /// Register a callback that will be executed when the window is closed.
    pub fn on_exit(mut self, callback: fn(&mut Window)) -> Self {
        self.window_config.on_exit = Some(Arc::new(Box::new(callback)));
        self
    }

    /// Add a new plugin.
    pub fn with_plugin(mut self, plugin: impl FreyaPlugin + 'static) -> Self {
        self.plugins.add_plugin(plugin);
        self
    }

    /// Register a Window Attributes hook.
    pub fn with_window_attributes(
        mut self,
        window_attributes_hook: impl Fn(WindowAttributes) -> WindowAttributes + 'static,
    ) -> Self {
        self.window_config.window_attributes_hook = Some(Box::new(window_attributes_hook));
        self
    }
}
