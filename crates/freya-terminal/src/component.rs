use std::borrow::Cow;

use freya_core::prelude::*;

use crate::{
    element::TerminalElement,
    handle::TerminalHandle,
};

/// User-facing Terminal component
#[derive(Clone, PartialEq)]
pub struct Terminal {
    handle: TerminalHandle,
    font_family: Cow<'static, str>,
    font_size: f32,
    foreground: Color,
    background: Color,
    layout: LayoutData,
    key: DiffKey,
}

impl Terminal {
    /// Create a terminal with a handle for interactive PTY
    pub fn new(handle: TerminalHandle) -> Self {
        Self {
            handle,
            font_family: Cow::Borrowed("Cascadia Code"),
            font_size: 14.,
            foreground: (220, 220, 220).into(),
            background: (10, 10, 10).into(),
            layout: LayoutData::default(),
            key: DiffKey::default(),
        }
    }

    /// Set the font family for the terminal
    pub fn font_family(mut self, font_family: impl Into<Cow<'static, str>>) -> Self {
        self.font_family = font_family.into();
        self
    }

    /// Set the font size for the terminal
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Set the foreground color
    pub fn foreground(mut self, foreground: impl Into<Color>) -> Self {
        self.foreground = foreground.into();
        self
    }

    /// Set the background color
    pub fn background(mut self, background: impl Into<Color>) -> Self {
        self.background = background.into();
        self
    }
}

impl Component for Terminal {
    fn render(&self) -> impl IntoElement {
        TerminalElement::new(self.handle.clone())
            .font_family(self.font_family.clone())
            .font_size(self.font_size)
            .foreground(self.foreground)
            .background(self.background)
    }
}

impl KeyExt for Terminal {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for Terminal {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}
impl ContainerExt for Terminal {}
