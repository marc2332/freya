/// Configuration for a Window.
#[derive(Clone)]
pub struct WindowConfig<T: Clone> {
    /// Width of the Window.
    pub width: u32,
    /// Height of the window.
    pub height: u32,
    /// Enable Window decorations.
    pub decorations: bool,
    /// Title for the Window.
    pub title: &'static str,
    /// Make the Window transparent or not.
    pub transparent: bool,
    /// A custom value to consume from your app.
    pub state: Option<T>,
}

impl<T: Clone> Default for WindowConfig<T> {
    fn default() -> Self {
        Self {
            width: 350,
            height: 350,
            decorations: true,
            title: "Freya app",
            transparent: false,
            state: None,
        }
    }
}

impl<T: Clone> WindowConfig<T> {
    pub fn builder() -> WindowConfigBuilder<T> {
        WindowConfigBuilder::default()
    }
}

/// Configuration Builder for a window.
#[derive(Clone)]
pub struct WindowConfigBuilder<T> {
    pub width: u32,
    pub height: u32,
    pub decorations: bool,
    pub title: &'static str,
    pub transparent: bool,
    pub state: Option<T>,
}

impl<T> Default for WindowConfigBuilder<T> {
    fn default() -> Self {
        Self {
            width: 350,
            height: 350,
            decorations: true,
            title: "Freya app",
            transparent: false,
            state: None,
        }
    }
}

impl<T: Clone> WindowConfigBuilder<T> {
    /// Specify a Window width.
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Specify a Window height.
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
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

    /// Build the Window.
    pub fn build(self) -> WindowConfig<T> {
        WindowConfig {
            width: self.width,
            height: self.height,
            title: self.title,
            decorations: self.decorations,
            transparent: self.transparent,
            state: self.state,
        }
    }
}
