use freya_engine::prelude::Slant as SkSlant;

/// Slant (style) of a font.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum FontSlant {
    /// Regular, non-slanted text. This is the default.
    #[default]
    Upright = 0,
    /// Italic text, using the font's dedicated italic glyphs.
    Italic = 1,
    /// Oblique text, a slanted version of the upright glyphs.
    Oblique = 2,
}

impl FontSlant {
    pub fn pretty(&self) -> String {
        match self {
            Self::Upright => "Upright".to_string(),
            Self::Italic => "Italic".to_string(),
            Self::Oblique => "Oblique".to_string(),
        }
    }
}

impl From<FontSlant> for SkSlant {
    fn from(value: FontSlant) -> Self {
        match value {
            FontSlant::Italic => SkSlant::Italic,
            FontSlant::Oblique => SkSlant::Oblique,
            FontSlant::Upright => SkSlant::Upright,
        }
    }
}
