use freya_core::prelude::*;

use crate::theming::hooks::get_theme_or_default;

pub trait RectThemeExt {
    fn theme_background(self) -> Self;
    fn theme_color(self) -> Self;
}

impl RectThemeExt for Rect {
    fn theme_background(self) -> Self {
        let theme = get_theme_or_default();
        self.background(theme.colors.background)
    }

    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.colors.text_primary)
    }
}

pub trait LabelThemeExt {
    fn theme_color(self) -> Self;
}

impl LabelThemeExt for Label {
    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.colors.text_primary)
    }
}

pub trait ParagraphThemeExt {
    fn theme_color(self) -> Self;
}

impl ParagraphThemeExt for Paragraph {
    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.colors.text_primary)
    }
}
