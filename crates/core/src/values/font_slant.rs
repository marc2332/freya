use freya_engine::prelude::Slant as SkSlant;

use crate::parsing::{
    Parse,
    ParseError,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FontSlant {
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

impl Parse for FontSlant {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "upright" => FontSlant::Upright,
            "italic" => FontSlant::Italic,
            "oblique" => FontSlant::Oblique,
            _ => FontSlant::Upright,
        })
    }
}
