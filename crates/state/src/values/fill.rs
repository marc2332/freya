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
    RadialGradient,
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
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(if value.starts_with("linear-gradient(") {
            Self::LinearGradient(LinearGradient::parse(value).map_err(|_| ParseError)?)
        } else if value.starts_with("radial-gradient(") {
            Self::RadialGradient(RadialGradient::parse(value).map_err(|_| ParseError)?)
        } else if value.starts_with("conic-gradient(") {
            Self::ConicGradient(ConicGradient::parse(value).map_err(|_| ParseError)?)
        } else {
            Self::Color(Color::parse(value).map_err(|_| ParseError)?)
        })
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
