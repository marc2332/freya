/// Configuration for a window.
#[derive(Clone)]
pub struct WindowConfig<T: Clone> {
    pub width: u32,
    pub height: u32,
    pub decorations: bool,
    pub title: &'static str,
    pub transparent: bool,
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

/// Configuration builder for a window.
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
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn with_transparency(mut self, transparency: bool) -> Self {
        self.transparent = transparency;
        self
    }

    pub fn with_state(mut self, state: T) -> Self {
        self.state = Some(state);
        self
    }

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
