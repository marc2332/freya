use freya_engine::prelude::*;

use crate::{
    Parse,
    ParseError,
};

impl Parse for TextHeightBehavior {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.try_as_str().and_then(|value| match value {
                "all" => Some(TextHeightBehavior::All),
                "disable-first-ascent" => Some(TextHeightBehavior::DisableFirstAscent),
                "disable-least-ascent" => Some(TextHeightBehavior::DisableLastDescent),
                "disable-all" => Some(TextHeightBehavior::DisableAll),
                _ => None,
            })
        })
    }
}

pub trait TextHeight {
    fn needs_custom_height(&self) -> bool;
}

impl TextHeight for TextHeightBehavior {
    fn needs_custom_height(&self) -> bool {
        matches!(
            self,
            Self::All | Self::DisableFirstAscent | Self::DisableLastDescent
        )
    }
}
