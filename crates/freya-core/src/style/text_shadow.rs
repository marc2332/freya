use freya_engine::prelude::*;

use crate::style::color::Color;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct TextShadow {
    pub color: Color,
    pub offset: (f32, f32),
    pub blur_sigma: f64,
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
