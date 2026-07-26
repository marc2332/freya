use std::{
    any::Any,
    fmt,
};

use freya_core::{
    integration::FxHashMap,
    prelude::*,
};

use crate::theming::themes::light_theme;

/// The color source a [`Theme`] resolves
/// [`Preference::Reference`](crate::theming::macros::Preference::Reference) against.
///
/// The core [`ColorsSheet`] is required, so a reference to one of its slots always resolves
/// no matter which palette an app installs, and built-in components can never be broken by a
/// custom one. [`Palette::color`] is an open, app-defined namespace on top: it is consulted
/// only for names the sheet does not carry, and it may compute a value (a tint of a slot, a
/// mix of two) rather than store one.
pub trait Palette: 'static {
    /// The core slots.
    fn sheet(&self) -> &ColorsSheet;

    /// Resolve an app-defined slot name. Called only when `name` is not a core slot; returning
    /// `None` (the default) leaves the reference unresolved.
    fn color(&self, name: &str) -> Option<Color> {
        let _ = name;
        None
    }
}

/// The plain sheet is itself a palette, with no extended slots.
impl Palette for ColorsSheet {
    fn sheet(&self) -> &ColorsSheet {
        self
    }
}

pub struct Theme {
    pub name: &'static str,
    pub palette: Box<dyn Palette>,
    themes: FxHashMap<&'static str, Box<dyn Any>>,
}

impl Theme {
    pub fn new(name: &'static str, palette: impl Palette) -> Self {
        Self {
            name,
            palette: Box::new(palette),
            themes: FxHashMap::default(),
        }
    }

    /// The core color slots of this theme's palette.
    pub fn colors(&self) -> &ColorsSheet {
        self.palette.sheet()
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
            .field("colors", self.colors())
            .field("themes", &format!("({} entries)", self.themes.len()))
            .finish()
    }
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.colors() == other.colors()
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
    pub focus: Color,
    pub active: Color,
    pub disabled: Color,

    // Utility
    pub overlay: Color,
    pub shadow: Color,
}
