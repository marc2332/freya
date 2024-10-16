use freya_engine::prelude::*;

use crate::{
    Parse,
    ParseError,
};

impl Parse for TextHeightBehavior {
    fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "all" => Ok(TextHeightBehavior::All),
            "disable-first-ascent" => Ok(TextHeightBehavior::DisableFirstAscent),
            "disable-least-ascent" => Ok(TextHeightBehavior::DisableLastDescent),
            "disable-all" => Ok(TextHeightBehavior::DisableAll),
            _ => Err(ParseError),
        }
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
