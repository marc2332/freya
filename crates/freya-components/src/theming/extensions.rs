use freya_core::prelude::PreferredTheme;

use crate::theming::{
    component_themes::Theme,
    themes::{
        dark_theme,
        light_theme,
    },
};

pub trait FromPreference {
    fn to_theme(&self) -> Theme;
}

impl FromPreference for PreferredTheme {
    fn to_theme(&self) -> Theme {
        match self {
            PreferredTheme::Dark => dark_theme(),
            PreferredTheme::Light => light_theme(),
        }
    }
}
