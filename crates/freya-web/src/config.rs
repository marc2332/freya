use std::borrow::Cow;

use freya_core::{
    integration::AppComponent,
    prelude::Color,
};

/// Configuration for a Freya app in the browser.
pub struct WebConfig {
    pub(crate) app: AppComponent,
    pub(crate) background: Color,
    pub(crate) fonts: Vec<(String, Vec<u8>)>,
    pub(crate) default_fonts: Vec<Cow<'static, str>>,
}

impl WebConfig {
    pub fn new(app: impl Into<AppComponent>) -> Self {
        Self {
            app: app.into(),
            background: Color::WHITE,
            fonts: Vec::new(),
            default_fonts: Vec::new(),
        }
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    /// Register a font, the first registered font is used as the default one.
    pub fn with_font(mut self, name: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        self.fonts.push((name.into(), data.into()));
        self
    }

    /// Font families used when an element does not specify one.
    pub fn with_default_fonts(mut self, default_fonts: Vec<Cow<'static, str>>) -> Self {
        self.default_fonts = default_fonts;
        self
    }
}
