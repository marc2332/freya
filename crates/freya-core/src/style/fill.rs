use std::{
    fmt::{
        self,
        Pointer,
    },
    hash::{
        Hash,
        Hasher,
    },
    mem::discriminant,
};

use freya_engine::prelude::Paint;
use torin::prelude::Area;

use crate::{
    prelude::Color,
    style::{
        gradient::{
            ConicGradient,
            LinearGradient,
            RadialGradient,
        },
        shader::ShaderFill,
    },
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    Color(Color),
    Shader(Box<ShaderFill>),
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

    /// Returns the inner [Color] when this is a [Fill::Color].
    pub fn as_color(&self) -> Option<Color> {
        match self {
            Fill::Color(color) => Some(*color),
            _ => None,
        }
    }

    pub fn apply_to_paint(&self, paint: &mut Paint, area: Area) {
        match &self {
            Fill::Color(color) => {
                paint.set_color(*color);
            }
            Fill::LinearGradient(gradient) => {
                paint.set_shader(gradient.prepare_shader(area));
            }
            Fill::RadialGradient(gradient) => {
                paint.set_shader(gradient.prepare_shader(area));
            }
            Fill::ConicGradient(gradient) => {
                paint.set_shader(gradient.prepare_shader(area));
            }
            Fill::Shader(shader) => {
                paint.set_shader(shader.prepare_shader(area));
            }
        }
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self::Color(Color::default())
    }
}

impl Hash for Fill {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
        match self {
            Fill::Color(color) => color.hash(state),
            Fill::Shader(shader) => shader.hash(state),
            Fill::LinearGradient(gradient) => gradient.hash(state),
            Fill::RadialGradient(gradient) => gradient.hash(state),
            Fill::ConicGradient(gradient) => gradient.hash(state),
        }
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
            Self::Shader(shader) => shader.as_ref().fmt(f),
            Self::LinearGradient(gradient) => gradient.as_ref().fmt(f),
            Self::RadialGradient(gradient) => gradient.as_ref().fmt(f),
            Self::ConicGradient(gradient) => gradient.as_ref().fmt(f),
        }
    }
}
