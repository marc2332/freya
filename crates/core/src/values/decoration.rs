use bitflags::bitflags;
use freya_engine::prelude::*;

use crate::parsing::{
    Parse,
    ParseError,
};

bitflags! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextDecoration: u32 {
        const NO_DECORATION = 0;
        const UNDERLINE = 1;
        const OVERLINE = 2;
        const LINE_THROUGH = 3;
    }
}

impl Default for TextDecoration {
    fn default() -> Self {
        TextDecoration::NO_DECORATION
    }
}

impl From<TextDecoration> for SkTextDecoration {
    fn from(value: TextDecoration) -> Self {
        let mut text_decoration = SkTextDecoration::default();
        if value.contains(TextDecoration::UNDERLINE) {
            text_decoration.insert(SkTextDecoration::UNDERLINE);
        }
        if value.contains(TextDecoration::OVERLINE) {
            text_decoration.insert(SkTextDecoration::OVERLINE);
        }
        if value.contains(TextDecoration::LINE_THROUGH) {
            text_decoration.insert(SkTextDecoration::LINE_THROUGH);
        }
        text_decoration
    }
}

impl Parse for TextDecoration {
    fn parse(value: &str) -> Result<Self, ParseError> {
        let mut decoration = TextDecoration::default();
        let values = value.split_ascii_whitespace();

        for val in values {
            decoration.set(
                match val {
                    "underline" => TextDecoration::UNDERLINE,
                    "overline" => TextDecoration::OVERLINE,
                    "line-through" => TextDecoration::LINE_THROUGH,
                    _ => TextDecoration::NO_DECORATION,
                },
                true,
            );
        }

        Ok(decoration)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextDecorationStyle {
    Solid = 0,
    Double = 1,
    Dotted = 2,
    Dashed = 3,
    Wavy = 4,
}

impl Default for TextDecorationStyle {
    fn default() -> Self {
        Self::Solid
    }
}

impl From<TextDecorationStyle> for SkTextDecorationStyle {
    fn from(value: TextDecorationStyle) -> Self {
        match value {
            TextDecorationStyle::Solid => SkTextDecorationStyle::Solid,
            TextDecorationStyle::Double => SkTextDecorationStyle::Double,
            TextDecorationStyle::Dotted => SkTextDecorationStyle::Dotted,
            TextDecorationStyle::Dashed => SkTextDecorationStyle::Dashed,
            TextDecorationStyle::Wavy => SkTextDecorationStyle::Wavy,
        }
    }
}

impl Parse for TextDecorationStyle {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "solid" => TextDecorationStyle::Solid,
            "double" => TextDecorationStyle::Double,
            "dotted" => TextDecorationStyle::Dotted,
            "dashed" => TextDecorationStyle::Dashed,
            "wavy" => TextDecorationStyle::Wavy,
            _ => TextDecorationStyle::Solid,
        })
    }
}
