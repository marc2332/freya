use crate::Parse;
use freya_engine::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTextDecorationError;

impl Parse for TextDecoration {
    type Err = ParseTextDecorationError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTextDecorationStyleError;

impl Parse for TextDecorationStyle {
    type Err = ParseTextDecorationStyleError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
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
