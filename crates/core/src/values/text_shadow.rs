use freya_engine::prelude::*;

use crate::{
    parsing::{
        Parse,
        ParseError,
    },
    values::Color,
};

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

impl From<TextShadow> for freya_engine::prelude::SkTextShadow {
    fn from(value: TextShadow) -> Self {
        let color: SkColor = value.color.into();
        SkTextShadow::new(
            color,
            SkPoint::new(value.offset.0, value.offset.1),
            value.blur_sigma,
        )
    }
}

// Same as shadow, but no inset or spread.
impl Parse for TextShadow {
    fn parse(value: &str) -> Result<Self, ParseError> {
        let mut shadow_values = value.split_ascii_whitespace();
        Ok(TextShadow {
            offset: (
                shadow_values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?,
                shadow_values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?,
            ),
            blur_sigma: shadow_values
                .next()
                .ok_or(ParseError)?
                .parse::<f64>()
                .map_err(|_| ParseError)?
                / 2.0,
            color: Color::parse(shadow_values.collect::<Vec<&str>>().join(" ").as_str())
                .map_err(|_| ParseError)?,
        })
    }
}
