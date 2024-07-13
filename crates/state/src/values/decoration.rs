use freya_engine::prelude::*;

use crate::{
    Parse,
    ParseError,
    Parser,
};

impl Parse for TextDecoration {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut decoration = Self::default();

        while let Ok(value) = parser.consume_map(|token| {
            token.as_string().and_then(|value| match value {
                "none" => Some(Self::NO_DECORATION),
                "underline" => Some(Self::UNDERLINE),
                "overline" => Some(Self::OVERLINE),
                "line-through" => Some(Self::LINE_THROUGH),
                _ => None,
            })
        }) {
            decoration.set(value, true);
        }

        Ok(decoration)
    }
}

impl Parse for TextDecorationStyle {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|token| {
            token.as_string().and_then(|value| match value {
                "solid" => Some(Self::Solid),
                "double" => Some(Self::Double),
                "dotted" => Some(Self::Dotted),
                "dashed" => Some(Self::Dashed),
                "wavy" => Some(Self::Wavy),
                _ => None,
            })
        })
    }
}
