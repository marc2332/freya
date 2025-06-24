use freya_engine::prelude::*;

use crate::parsing::{
    Parse,
    ParseError,
};

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
            )
                .into(),
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
