use freya_core::prelude::*;

use crate::{
    define_theme,
    theming::{
        component_themes::ColorsSheet,
        hooks::get_theme_or_default,
        macros::{
            Preference,
            ResolvablePreference,
        },
    },
};

/// A font size in logical pixels or a multiplier of the typography base size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypographySize {
    Fixed(f32),
    Relative(f32),
}

impl From<f32> for TypographySize {
    fn from(size: f32) -> Self {
        Self::Fixed(size)
    }
}

impl ResolvablePreference<TypographySize> for Preference<TypographySize> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> TypographySize {
        match self {
            Self::Specific(size) => *size,
            _ => {
                panic!("Typography sizes must be fixed values or relative multipliers.")
            }
        }
    }
}

/// The semantic purpose of text, independent of its concrete font size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextRole {
    Control,
    #[default]
    Content,
    Title,
    Subtitle,
    Caption,
    Overline,
}

define_theme! {
    %[no_ext]
    pub Typography {
        %[fields]
        base_size: f32,
        control: TypographySize,
        title: TypographySize,
        subtitle: TypographySize,
        body: TypographySize,
        caption: TypographySize,
        overline: TypographySize,
    }
}

impl Default for TypographyThemePreference {
    fn default() -> Self {
        Self {
            base_size: 16.0.into(),
            control: TypographySize::Relative(0.875).into(),
            title: TypographySize::Relative(1.5).into(),
            subtitle: TypographySize::Relative(1.125).into(),
            body: TypographySize::Relative(1.0).into(),
            caption: TypographySize::Relative(0.75).into(),
            overline: TypographySize::Relative(0.6875).into(),
        }
    }
}

impl TypographyTheme {
    pub fn font_size(&self, role: TextRole) -> f32 {
        let size = match role {
            TextRole::Control => self.control,
            TextRole::Content => self.body,
            TextRole::Title => self.title,
            TextRole::Subtitle => self.subtitle,
            TextRole::Caption => self.caption,
            TextRole::Overline => self.overline,
        };

        match size {
            TypographySize::Fixed(size) => size,
            TypographySize::Relative(multiplier) => self.base_size * multiplier,
        }
    }
}

/// Apply semantic font sizes to text or a container whose children inherit them.
pub trait TextRoleExt: TextStyleExt + Sized {
    /// Resolve the role against the current theme's typography scale.
    fn theme_font_size(self, role: TextRole) -> Self {
        self.font_size(get_theme_or_default().read().font_size(role))
    }
}

impl<T: TextStyleExt> TextRoleExt for T {}
