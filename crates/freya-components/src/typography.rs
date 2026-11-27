use freya_core::prelude::*;

use crate::theming::{
    component_themes::AnyMap,
    hooks::get_theme_or_default,
};

/// A font size in logical pixels or a multiplier of the base size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypographySize {
    Fixed(f32),
    Relative(f32),
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

impl TextRole {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Control => "font_size.control",
            Self::Content => "font_size.content",
            Self::Title => "font_size.title",
            Self::Subtitle => "font_size.subtitle",
            Self::Caption => "font_size.caption",
            Self::Overline => "font_size.overline",
        }
    }
}

/// Builds font-size variables from a shared base size.
#[derive(Clone, Debug, PartialEq)]
pub struct TypographyScale {
    pub base_size: f32,
    pub control: TypographySize,
    pub body: TypographySize,
    pub title: TypographySize,
    pub subtitle: TypographySize,
    pub caption: TypographySize,
    pub overline: TypographySize,
}

impl Default for TypographyScale {
    fn default() -> Self {
        Self {
            base_size: 16.,
            control: TypographySize::Relative(0.875),
            body: TypographySize::Relative(1.),
            title: TypographySize::Relative(1.5),
            subtitle: TypographySize::Relative(1.125),
            caption: TypographySize::Relative(0.75),
            overline: TypographySize::Relative(0.6875),
        }
    }
}

impl TypographyScale {
    /// Write resolved font sizes.
    pub fn apply_to(&self, variables: &mut AnyMap) {
        for (role, size) in [
            (TextRole::Control, self.control),
            (TextRole::Content, self.body),
            (TextRole::Title, self.title),
            (TextRole::Subtitle, self.subtitle),
            (TextRole::Caption, self.caption),
            (TextRole::Overline, self.overline),
        ] {
            let size = match size {
                TypographySize::Fixed(size) => size,
                TypographySize::Relative(multiplier) => self.base_size * multiplier,
            };
            variables.set(role.key(), size);
        }
    }
}

/// Apply semantic font sizes to text or a container whose children inherit them.
pub trait TextRoleExt: TextStyleExt + Sized {
    fn theme_font_size(self, role: TextRole) -> Self {
        self.font_size(*get_theme_or_default().read().get_var::<f32>(role.key()))
    }
}

impl<T: TextStyleExt> TextRoleExt for T {}
