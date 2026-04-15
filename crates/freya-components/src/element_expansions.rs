use freya_core::prelude::*;

use crate::theming::hooks::get_theme_or_default;

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
}

impl LabelThemeExt for Label {
    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.text_primary)
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

pub trait SvgThemeExt {
    fn theme_color(self) -> Self;
    fn theme_accent_color(self) -> Self;
    fn theme_fill(self) -> Self;
    fn theme_stroke(self) -> Self;
    fn theme_accent_fill(self) -> Self;
    fn theme_accent_stroke(self) -> Self;
}

impl SvgThemeExt for Svg {
    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.text_primary)
    }

    fn theme_accent_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.primary)
    }

    fn theme_fill(self) -> Self {
        let theme = get_theme_or_default();
        self.fill(theme.read().colors.text_primary)
    }

    fn theme_stroke(self) -> Self {
        let theme = get_theme_or_default();
        self.stroke(theme.read().colors.text_primary)
    }

    fn theme_accent_fill(self) -> Self {
        let theme = get_theme_or_default();
        self.fill(theme.read().colors.primary)
    }

    fn theme_accent_stroke(self) -> Self {
        let theme = get_theme_or_default();
        self.stroke(theme.read().colors.primary)
    }
}
