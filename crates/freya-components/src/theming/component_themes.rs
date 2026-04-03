use std::{
    any::Any,
    fmt,
};

use freya_core::{
    integration::FxHashMap,
    prelude::*,
};

use crate::theming::themes::light_theme;

pub struct Theme {
    pub name: &'static str,
    pub colors: ColorsSheet,
    themes: FxHashMap<&'static str, Box<dyn Any>>,
}

impl Theme {
    pub fn new(name: &'static str, colors: ColorsSheet) -> Self {
        Self {
            name,
            colors,
            themes: FxHashMap::default(),
        }
    }

    /// Get a component theme by key.
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.themes.get(key).and_then(|v| v.downcast_ref())
    }

    /// Set a component theme by key.
    pub fn set<T: 'static>(&mut self, key: &'static str, val: T) {
        self.themes.insert(key, Box::new(val));
    }
}

impl fmt::Debug for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Theme")
            .field("name", &self.name)
            .field("colors", &self.colors)
            .field("themes", &format!("({} entries)", self.themes.len()))
            .finish()
    }
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.colors == other.colors
    }
}

impl Default for Theme {
    fn default() -> Self {
        light_theme()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorsSheet {
    // Brand & Accent
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,

    // Status / Semantic colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Surfaces / Backgrounds
    pub background: Color,
    pub surface_primary: Color,
    pub surface_secondary: Color,
    pub surface_tertiary: Color,
    pub surface_inverse: Color,
    pub surface_inverse_secondary: Color,
    pub surface_inverse_tertiary: Color,

    // Borders
    pub border: Color,
    pub border_focus: Color,
    pub border_disabled: Color,

    // Text / Content
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_placeholder: Color,
    pub text_inverse: Color,
    pub text_highlight: Color,

    // States / Interaction
    pub hover: Color,
    pub focus: Color,
    pub active: Color,
    pub disabled: Color,

    // Utility
    pub overlay: Color,
    pub shadow: Color,
}
