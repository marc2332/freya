use freya_engine::prelude::*;

use crate::{
    parsing::{
        Parse,
        ParseError,
    },
    values::Color,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SvgPaint {
    #[default]
    None,
    CurrentColor,
    Color(Color),
}

impl Parse for SvgPaint {
    fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "current_color" => Ok(SvgPaint::CurrentColor),
            "none" => Ok(SvgPaint::None),
            value => Ok(SvgPaint::Color(Color::parse(value)?)),
        }
    }
}

impl From<SvgPaint> for svg::Paint {
    fn from(value: SvgPaint) -> Self {
        match value {
            SvgPaint::None => svg::Paint::none(),
            SvgPaint::CurrentColor => svg::Paint::current_color(),
            SvgPaint::Color(color) => svg::Paint::from_color(color.into()),
        }
    }
}
