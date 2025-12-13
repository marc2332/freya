use freya_core::prelude::PreferredTheme;

use crate::theming::{
    component_themes::Theme,
    themes::{
        DARK_THEME,
        LIGHT_THEME,
    },
};

pub trait FromPreference {
    fn to_theme(&self) -> Theme;
}

impl FromPreference for PreferredTheme {
    fn to_theme(&self) -> Theme {
        match self {
            PreferredTheme::Dark => DARK_THEME,
            PreferredTheme::Light => LIGHT_THEME,
        }
    }
}
