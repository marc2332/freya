use std::fmt::{
    self,
    Pointer,
};

use freya_engine::prelude::Paint;
use torin::prelude::Area;

use crate::{
    prelude::Color,
    style::gradient::{
        ConicGradient,
        LinearGradient,
        RadialGradient,
    },
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Color(color) => color.fmt(f),
            Self::LinearGradient(gradient) => gradient.as_ref().fmt(f),
            Self::RadialGradient(gradient) => gradient.as_ref().fmt(f),
            Self::ConicGradient(gradient) => gradient.as_ref().fmt(f),
        }
    }
}
