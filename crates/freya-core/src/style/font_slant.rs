use freya_engine::prelude::Slant as SkSlant;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum FontSlant {
    #[default]
    Upright = 0,
    Italic = 1,
    Oblique = 2,
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
