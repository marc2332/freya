use std::io::Cursor;

use freya_core::{
    event_loop_messages::EventLoopMessage,
    parsing::Parse,
    plugins::{
        FreyaPlugin,
        PluginsManager,
    },
    style::fallback_fonts,
};
use freya_engine::prelude::Color;
use image::ImageReader;
use winit::{
    event_loop::EventLoopBuilder,
    window::{
        Icon,
        Window,
        WindowAttributes,
    },
};

pub type WindowCallback = Box<dyn FnOnce(&mut Window)>;
pub type EventLoopBuilderHook = Box<dyn FnOnce(&mut EventLoopBuilder<EventLoopMessage>)>;
pub type WindowBuilderHook = Box<dyn FnOnce(WindowAttributes) -> WindowAttributes>;
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
    /// Window visibility. Default to `true`.
    pub visible: bool,
    /// The Icon of the Window.
    pub icon: Option<Icon>,
    /// Setup callback.
    pub on_setup: Option<WindowCallback>,
    /// Exit callback.
    pub on_exit: Option<WindowCallback>,
    /// Hook function called with the Window Attributes.
    pub window_attributes_hook: Option<WindowBuilderHook>,
    /// Hook function called with the Event Loop Builder.
    pub event_loop_builder_hook: Option<EventLoopBuilderHook>,
    /// Max resource in bytes to be used by the GPU. Defaults to automatic.
    pub max_gpu_resources_bytes: Option<usize>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            size: (700.0, 500.0),
            min_size: None,
            max_size: None,
            decorations: true,
            title: "Freya App",
            transparent: false,
            background: Color::WHITE,
            visible: true,
            icon: None,
            on_setup: None,
            on_exit: None,
            window_attributes_hook: None,
            event_loop_builder_hook: None,
            max_gpu_resources_bytes: None,
        }
    }
}

/// Launch configuration.
pub struct LaunchConfig<'a, T: Clone = ()> {
    pub state: Option<T>,
    pub window_config: WindowConfig,
    pub embedded_fonts: EmbeddedFonts<'a>,
    pub plugins: PluginsManager,
    pub fallback_fonts: Vec<String>,
}

impl<T: Clone> Default for LaunchConfig<'_, T> {
    fn default() -> Self {
        Self {
            state: None,
            window_config: Default::default(),
            embedded_fonts: Default::default(),
            plugins: Default::default(),
            fallback_fonts: fallback_fonts(),
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
}

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

    /// Specify the Window visibility at launch.
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.window_config.visible = visible;
        self
    }

    /// Embed a font.
    pub fn with_font(mut self, font_name: &'a str, font: &'a [u8]) -> Self {
        self.embedded_fonts.push((font_name, font));
        self
    }

    /// Register a fallback font. Will be used if the default fonts are not available.
    pub fn with_fallback_font(mut self, font_name: &str) -> Self {
        self.fallback_fonts.push(font_name.to_string());
        self
    }

    /// Register a default font. Will be used if found.
    pub fn with_default_font(mut self, font_name: &str) -> Self {
        self.fallback_fonts.insert(0, font_name.to_string());
        self
    }

    /// Specify the Window icon.
    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.window_config.icon = Some(icon);
        self
    }

    /// Register a callback that will be executed when the window is created.
    pub fn on_setup(mut self, callback: impl FnOnce(&mut Window) + 'static) -> Self {
        self.window_config.on_setup = Some(Box::new(callback));
        self
    }

    /// Register a callback that will be executed when the window is closed.
    pub fn on_exit(mut self, callback: impl FnOnce(&mut Window) + 'static) -> Self {
        self.window_config.on_exit = Some(Box::new(callback));
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
        window_attributes_hook: impl FnOnce(WindowAttributes) -> WindowAttributes + 'static,
    ) -> Self {
        self.window_config.window_attributes_hook = Some(Box::new(window_attributes_hook));
        self
    }

    /// Register an Event Loop Builder hook.
    pub fn with_event_loop_builder(
        mut self,
        event_loop_builder_hook: impl FnOnce(&mut EventLoopBuilder<EventLoopMessage>) + 'static,
    ) -> Self {
        self.window_config.event_loop_builder_hook = Some(Box::new(event_loop_builder_hook));
        self
    }

    /// Specify the max resources to be cached for the GPU, in bytes.
    pub fn with_max_gpu_resources_bytes(mut self, max_gpu_resources_bytes: usize) -> Self {
        self.window_config.max_gpu_resources_bytes = Some(max_gpu_resources_bytes);
        self
    }
}
