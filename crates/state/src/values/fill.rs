use std::fmt;

use freya_engine::prelude::{
    Color,
    Paint,
};
use torin::prelude::Area;

use crate::{
    ConicGradient,
    DisplayColor,
    LinearGradient,
    Parse,
    ParseError,
    Parser,
    RadialGradient,
    Token,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    Color(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    ConicGradient(ConicGradient),
}

impl Fill {
    pub fn apply_to_paint(&self, paint: &mut Paint, area: Area) {
        match &self {
            Fill::Color(color) => {
                paint.set_color(*color);
            }
            Fill::LinearGradient(gradient) => {
                paint.set_shader(gradient.into_shader(area));
            }
            Fill::RadialGradient(gradient) => {
                paint.set_shader(gradient.into_shader(area));
            }
            Fill::ConicGradient(gradient) => {
                paint.set_shader(gradient.into_shader(area));
            }
        }
    }
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
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        if parser.check(&Token::ident("linear-gradient")) {
            LinearGradient::from_parser(parser).map(Self::LinearGradient)
        } else if parser.check(&Token::ident("radial-gradient")) {
            RadialGradient::from_parser(parser).map(Self::RadialGradient)
        } else if parser.check(&Token::ident("gradient-gradient")) {
            ConicGradient::from_parser(parser).map(Self::ConicGradient)
        } else {
            Color::from_parser(parser).map(Self::Color)
        }
    }
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Color(color) => color.fmt_rgb(f),
            Self::LinearGradient(gradient) => gradient.fmt(f),
            Self::RadialGradient(gradient) => gradient.fmt(f),
            Self::ConicGradient(gradient) => gradient.fmt(f),
        }
    }
}
