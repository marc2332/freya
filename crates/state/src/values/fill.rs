use std::fmt;

use freya_engine::prelude::Color;

use crate::{
    DisplayColor,
    LinearGradient,
    Parse,
    ParseError,
    Parser,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    Color(Color),
    LinearGradient(LinearGradient),
    // RadialGradient(RadialGradient),
    // ConicGradient(ConicGradient),
}

impl Default for Fill {
    fn default() -> Self {
        Self::Color(Color::default())
    }
}

impl From<Color> for Fill {
    fn from(color: Color) -> Self {
        Fill::Color(color)
    }
}

impl Parse for Fill {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        LinearGradient::parse(parser)
            .map(Self::LinearGradient)
            .or(Color::parse(parser).map(Self::Color))
    }
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Color(color) => color.fmt_rgb(f),
            Self::LinearGradient(gradient) => gradient.fmt(f),
        }
    }
}
