use torin::prelude::AspectRatio;

use crate::parsing::{
    Parse,
    ParseError,
};

impl Parse for AspectRatio {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "none" => Self::None,
            "fit" => Self::Fit,
            "fill" => Self::Fill,
            "max" => Self::Max,
            _ => return Err(ParseError),
        })
    }
}
