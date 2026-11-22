use freya_core::prelude::*;

use crate::{
    theming::hooks::get_theme_or_default,
    typography::{
        TextRole,
        TextRoleExt,
    },
};

pub trait RectThemeExt {
    fn theme_background(self) -> Self;
    fn theme_color(self) -> Self;
}

impl RectThemeExt for Rect {
    fn theme_background(self) -> Self {
        let theme = get_theme_or_default();
        self.background(theme.read().colors.background)
    }

    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.text_primary)
    }
}

pub trait LabelThemeExt {
    fn theme_color(self) -> Self;

    /// Apply the theme's `title` font size (largest heading level).
    fn title(self) -> Self;

    /// Apply the theme's `subtitle` font size (secondary heading level).
    fn subtitle(self) -> Self;

    /// Apply the theme's `body` font size (default body text).
    fn body(self) -> Self;

    /// Apply the theme's `caption` font size (smaller annotation text).
    fn caption(self) -> Self;

    /// Apply the theme's `overline` font size (smallest accent text).
    fn overline(self) -> Self;
}

impl LabelThemeExt for Label {
    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.text_primary)
    }

    fn title(self) -> Self {
        self.theme_font_size(TextRole::Title)
    }

    fn subtitle(self) -> Self {
        self.theme_font_size(TextRole::Subtitle)
    }

    fn body(self) -> Self {
        self.theme_font_size(TextRole::Content)
    }

    fn caption(self) -> Self {
        self.theme_font_size(TextRole::Caption)
    }

    fn overline(self) -> Self {
        self.theme_font_size(TextRole::Overline)
    }
}

pub trait ParagraphThemeExt {
    fn theme_color(self) -> Self;
}

impl ParagraphThemeExt for Paragraph {
    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.text_primary)
    }
}
