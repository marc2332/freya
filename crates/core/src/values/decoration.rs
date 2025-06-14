use freya_engine::prelude::*;

use crate::parsing::{
    Parse,
    ParseError,
};

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
