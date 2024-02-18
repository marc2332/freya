use crate::{DisplayColor, LinearGradient, Parse};
use freya_engine::prelude::Color;
use std::fmt;

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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFillError;

impl Parse for Fill {
    type Err = ParseFillError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(if value.starts_with("linear-gradient(") {
            Self::LinearGradient(LinearGradient::parse(value).map_err(|_| ParseFillError)?)
        } else {
            Self::Color(Color::parse(value).map_err(|_| ParseFillError)?)
        })
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
