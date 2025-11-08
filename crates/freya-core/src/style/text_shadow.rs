use std::hash::Hash;

use freya_engine::prelude::*;

use crate::style::color::Color;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct TextShadow {
    pub color: Color,
    pub offset: (f32, f32),
    pub blur_sigma: f64,
}

impl Hash for TextShadow {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.offset.0.to_bits().hash(state);
        self.offset.1.to_bits().hash(state);
        self.blur_sigma.to_bits().hash(state);
    }
}

impl TextShadow {
    pub fn new(color: Color, offset: (f32, f32), blur_sigma: f64) -> Self {
        Self {
            color,
            offset,
            blur_sigma,
        }
    }
}

impl From<TextShadow> for SkTextShadow {
    fn from(value: TextShadow) -> Self {
        let color: SkColor = value.color.into();
        SkTextShadow::new(
            color,
            SkPoint::new(value.offset.0, value.offset.1),
            value.blur_sigma,
        )
    }
}
