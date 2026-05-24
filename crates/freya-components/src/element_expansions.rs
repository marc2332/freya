use freya_core::prelude::*;

use crate::{
    get_theme,
    theming::hooks::get_theme_or_default,
    typography::{
        TypographyThemePartial,
        TypographyThemePreference,
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

fn typography() -> crate::typography::TypographyTheme {
    get_theme!(
        &None::<TypographyThemePartial>,
        TypographyThemePreference,
        "typography"
    )
}

impl LabelThemeExt for Label {
    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.text_primary)
    }

    fn title(self) -> Self {
        self.font_size(typography().title)
    }

    fn subtitle(self) -> Self {
        self.font_size(typography().subtitle)
    }

    fn body(self) -> Self {
        self.font_size(typography().body)
    }

    fn caption(self) -> Self {
        self.font_size(typography().caption)
    }

    fn overline(self) -> Self {
        self.font_size(typography().overline)
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
