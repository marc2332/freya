use std::io::Cursor;

use freya_core::{
    event_loop_messages::EventLoopMessage,
    plugins::{
        FreyaPlugin,
        PluginsManager,
    },
    style::fallback_fonts,
    window_config::WindowConfig,
};
use image::ImageReader;
use winit::{
    event_loop::EventLoopBuilder,
    window::Icon,
};

pub type EventLoopBuilderHook = Box<dyn FnOnce(&mut EventLoopBuilder<EventLoopMessage>)>;
pub type EmbeddedFonts<'a> = Vec<(&'a str, &'a [u8])>;

/// Launch configuration.
pub struct LaunchConfig<'a, T: Clone = ()> {
    pub windows_configs: Vec<WindowConfig>,

    pub state: Option<T>,
    pub embedded_fonts: EmbeddedFonts<'a>,
    pub plugins: PluginsManager,
    pub fallback_fonts: Vec<String>,

    /// Hook function called with the Event Loop Builder.
    pub event_loop_builder_hook: Option<EventLoopBuilderHook>,
}

impl<T: Clone> Default for LaunchConfig<'_, T> {
    fn default() -> Self {
        Self {
            state: None,
            windows_configs: Default::default(),
            embedded_fonts: Default::default(),
            plugins: Default::default(),
            fallback_fonts: fallback_fonts(),
            event_loop_builder_hook: None,
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
    /// Register a window configuration. You can call this multiple times.
    pub fn with_window(mut self, window_config: WindowConfig) -> Self {
        self.windows_configs.push(window_config);
        self
    }

    /// Pass a custom value that your app will consume.
    pub fn with_state(mut self, state: T) -> Self {
        self.state = Some(state);
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

    /// Add a new plugin.
    pub fn with_plugin(mut self, plugin: impl FreyaPlugin + 'static) -> Self {
        self.plugins.add_plugin(plugin);
        self
    }

    /// Register an Event Loop Builder hook.
    pub fn with_event_loop_builder(
        mut self,
        event_loop_builder_hook: impl FnOnce(&mut EventLoopBuilder<EventLoopMessage>) + 'static,
    ) -> Self {
        self.event_loop_builder_hook = Some(Box::new(event_loop_builder_hook));
        self
    }
}
