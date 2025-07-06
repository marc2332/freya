use std::fmt;

use freya_engine::prelude::{
    Color,
    Paint,
};
use torin::prelude::Area;

use super::{
    ConicGradient,
    DisplayColor,
    LinearGradient,
    RadialGradient,
};
use crate::parsing::{
    Parse,
    ParseError,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    Color(Color),
    LinearGradient(Box<LinearGradient>),
    RadialGradient(Box<RadialGradient>),
    ConicGradient(Box<ConicGradient>),
}

impl Fill {
    pub fn set_a(&mut self, a: u8) {
        if let Fill::Color(color) = self {
            // Only actually change the alpha if its non-transparent
            if *color != Color::TRANSPARENT {
                *color = color.with_a(a);
            }
        }
    }

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
            Self::LinearGradient(Box::new(
                LinearGradient::parse(value).map_err(|_| ParseError)?,
            ))
        } else if value.starts_with("radial-gradient(") {
            Self::RadialGradient(Box::new(
                RadialGradient::parse(value).map_err(|_| ParseError)?,
            ))
        } else if value.starts_with("conic-gradient(") {
            Self::ConicGradient(Box::new(
                ConicGradient::parse(value).map_err(|_| ParseError)?,
            ))
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
