use std::{io::Cursor, sync::Arc};

use freya_core::plugins::{FreyaPlugin, PluginsManager};
use freya_engine::prelude::Color;
use freya_node_state::Parse;
use image::io::Reader;
use winit::window::{Icon, Window, WindowBuilder};

pub type WindowBuilderHook = Box<dyn Fn(WindowBuilder) -> WindowBuilder>;
pub type EmbeddedFonts<'a> = Vec<(&'a str, &'a [u8])>;

/// Configuration for a Window.
pub struct WindowConfig<T: Clone> {
    /// Width of the Window.
    pub width: f64,
    /// Height of the window.
    pub height: f64,
    /// Minimum width of the Window.
    pub min_width: Option<f64>,
    /// Minimum height of the window.
    pub min_height: Option<f64>,
    /// Maximum width of the Window.
    pub max_width: Option<f64>,
    /// Maximum height of the window.
    pub max_height: Option<f64>,
    /// Enable Window decorations.
    pub decorations: bool,
    /// Title for the Window.
    pub title: &'static str,
    /// Make the Window transparent or not.
    pub transparent: bool,
    /// A custom value to consume from your app.
    pub state: Option<T>,
    /// Background color of the Window.
    pub background: Color,
    /// The Icon of the Window.
    pub icon: Option<Icon>,
    /// Setup callback.
    pub on_setup: Option<WindowCallback>,
    /// Exit callback.
    pub on_exit: Option<WindowCallback>,
    /// Hook function called with the Window Builder.
    pub window_builder_hook: Option<WindowBuilderHook>,
}

impl<T: Clone> Default for WindowConfig<T> {
    fn default() -> Self {
        LaunchConfigBuilder::default().build().window
    }
}

/// Launch configuration.
pub struct LaunchConfig<'a, T: Clone> {
    pub window: WindowConfig<T>,
    pub embedded_fonts: EmbeddedFonts<'a>,
    pub plugins: PluginsManager,
    pub default_fonts: Vec<String>,
}

impl<'a, T: Clone> Default for LaunchConfig<'a, T> {
    fn default() -> Self {
        Self {
            window: Default::default(),
            embedded_fonts: Default::default(),
            plugins: Default::default(),
            default_fonts: vec!["Fira Sans".to_string()],
        }
    }
}

impl<'a, T: Clone> LaunchConfig<'a, T> {
    pub fn builder() -> LaunchConfigBuilder<'a, T> {
        LaunchConfigBuilder::default()
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

/// Configuration Builder.
pub struct LaunchConfigBuilder<'a, T> {
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) min_width: Option<f64>,
    pub(crate) min_height: Option<f64>,
    pub(crate) max_width: Option<f64>,
    pub(crate) max_height: Option<f64>,
    pub(crate) decorations: bool,
    pub(crate) title: &'static str,
    pub(crate) transparent: bool,
    pub(crate) state: Option<T>,
    pub(crate) background: Color,
    pub(crate) fonts: Vec<(&'a str, &'a [u8])>,
    pub(crate) icon: Option<Icon>,
    pub(crate) on_setup: Option<WindowCallback>,
    pub(crate) on_exit: Option<WindowCallback>,
    pub(crate) plugins: PluginsManager,
    pub(crate) window_builder_hook: Option<WindowBuilderHook>,
    pub(crate) default_fonts: Vec<String>,
}

impl<T> Default for LaunchConfigBuilder<'_, T> {
    fn default() -> Self {
        Self {
            width: 600.0,
            height: 600.0,
            min_width: None,
            min_height: None,
            max_height: None,
            max_width: None,
            decorations: true,
            title: "Freya app",
            transparent: false,
            state: None,
            background: Color::WHITE,
            fonts: Vec::default(),
            icon: None,
            on_setup: None,
            on_exit: None,
            plugins: PluginsManager::default(),
            window_builder_hook: None,
            default_fonts: vec!["Fira Sans".to_string()],
        }
    }
}

impl<'a, T: Clone> LaunchConfigBuilder<'a, T> {
    /// Specify a Window width.
    pub fn with_width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Specify a Window height.
    pub fn with_height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    /// Specify a minimum Window width.
    pub fn with_min_width(mut self, min_width: f64) -> Self {
        self.min_width = Some(min_width);
        self
    }

    /// Specify a minimum Window height.
    pub fn with_min_height(mut self, min_height: f64) -> Self {
        self.min_height = Some(min_height);
        self
    }

    /// Specify a maximum Window width.
    pub fn with_max_width(mut self, max_width: f64) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Specify a maximum Window height.
    pub fn with_max_height(mut self, max_height: f64) -> Self {
        self.max_height = Some(max_height);
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

    /// Pass a custom value that your app will consume.
    pub fn with_state(mut self, state: T) -> Self {
        self.state = Some(state);
        self
    }

    /// Specify the Window background color.
    pub fn with_background(mut self, background: &str) -> Self {
        self.background = Color::parse(background).unwrap_or(Color::WHITE);
        self
    }

    /// Embed a font.
    pub fn with_font(mut self, font_name: &'a str, font: &'a [u8]) -> Self {
        self.fonts.push((font_name, font));
        self
    }

    /// Clear default fonts.
    pub fn without_default_fonts(mut self) -> Self {
        self.default_fonts.clear();
        self
    }

    /// Resgiter a default font.
    pub fn with_default_font(mut self, font_name: &str) -> Self {
        self.default_fonts.push(font_name.to_string());
        self
    }

    /// Specify the Window icon.
    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Register a callback that will be executed when the window is created.
    pub fn on_setup(mut self, callback: fn(&mut Window)) -> Self {
        self.on_setup = Some(Arc::new(Box::new(callback)));
        self
    }

    /// Register a callback that will be executed when the window is closed.
    pub fn on_exit(mut self, callback: fn(&mut Window)) -> Self {
        self.on_exit = Some(Arc::new(Box::new(callback)));
        self
    }

    /// Add a new plugin.
    pub fn with_plugin(mut self, plugin: impl FreyaPlugin + 'static) -> Self {
        self.plugins.add_plugin(plugin);
        self
    }

    /// Register a Window Builder hook.
    pub fn with_window_builder(
        mut self,
        window_builder_hook: impl Fn(WindowBuilder) -> WindowBuilder + 'static,
    ) -> Self {
        self.window_builder_hook = Some(Box::new(window_builder_hook));
        self
    }

    /// Build the configuration.
    pub fn build(self) -> LaunchConfig<'a, T> {
        LaunchConfig {
            window: WindowConfig {
                width: self.width,
                height: self.height,
                min_width: self.min_width,
                min_height: self.min_height,
                max_width: self.max_width,
                max_height: self.max_height,
                title: self.title,
                decorations: self.decorations,
                transparent: self.transparent,
                state: self.state,
                background: self.background,
                icon: self.icon,
                on_setup: self.on_setup,
                on_exit: self.on_exit,
                window_builder_hook: self.window_builder_hook,
            },
            embedded_fonts: self.fonts,
            plugins: self.plugins,
            default_fonts: self.default_fonts,
        }
    }
}
