use crate::Parse;
use freya_engine::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTextShadowError;

// Same as shadow, but no inset or spread.
impl Parse for TextShadow {
    type Err = ParseTextShadowError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        let mut shadow_values = value.split_ascii_whitespace();
        Ok(TextShadow {
            offset: (
                shadow_values
                    .next()
                    .ok_or(ParseTextShadowError)?
                    .parse::<f32>()
                    .map_err(|_| ParseTextShadowError)?,
                shadow_values
                    .next()
                    .ok_or(ParseTextShadowError)?
                    .parse::<f32>()
                    .map_err(|_| ParseTextShadowError)?,
            )
                .into(),
            blur_sigma: shadow_values
                .next()
                .ok_or(ParseTextShadowError)?
                .parse::<f64>()
                .map_err(|_| ParseTextShadowError)?
                / 2.0,
            color: Color::parse(shadow_values.collect::<Vec<&str>>().join(" ").as_str())
                .map_err(|_| ParseTextShadowError)?,
        })
    }
}
