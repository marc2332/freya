use freya_engine::prelude::*;

use crate::parsing::{
    Parse,
    ParseError,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextHeightBehavior {
    All = 0,
    DisableFirstAscent = 1,
    DisableLastDescent = 2,
    DisableAll = 3,
}

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

impl From<TextHeightBehavior> for SkTextHeightBehavior {
    fn from(value: TextHeightBehavior) -> Self {
        match value {
            TextHeightBehavior::All => SkTextHeightBehavior::All,
            TextHeightBehavior::DisableAll => SkTextHeightBehavior::DisableAll,
            TextHeightBehavior::DisableFirstAscent => SkTextHeightBehavior::DisableFirstAscent,
            TextHeightBehavior::DisableLastDescent => SkTextHeightBehavior::DisableLastDescent,
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
