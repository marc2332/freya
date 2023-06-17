use std::{fmt, str};

use crate::Parse;
use skia_safe::Color;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Shadow {
    pub inset: bool,
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseShadowError;

impl Parse for Shadow {
    type Err = ParseShadowError;

    fn parse(value: &str, scale_factor: Option<f32>) -> Result<Self, Self::Err> {
        let mut shadow_values = value.split_ascii_whitespace();
        let mut shadow = Shadow::default();
        let scale_factor = scale_factor.unwrap_or(1.0);

        let first = shadow_values.next().ok_or(ParseShadowError)?;

        if first == "inset" {
            shadow.inset = true;
            shadow.x = shadow_values
                .next()
                .ok_or(ParseShadowError)?
                .parse::<f32>()
                .map_err(|_| ParseShadowError)?;
        } else {
            shadow.x = first.parse::<f32>().map_err(|_| ParseShadowError)?;
        }

        shadow.y = shadow_values
            .next()
            .ok_or(ParseShadowError)?
            .parse::<f32>()
            .map_err(|_| ParseShadowError)?;
        shadow.blur = shadow_values
            .next()
            .ok_or(ParseShadowError)?
            .parse::<f32>()
            .map_err(|_| ParseShadowError)?;

        let spread_or_color = shadow_values.next().ok_or(ParseShadowError)?;
        let mut color_string = String::new();
        if spread_or_color.parse::<f32>().is_ok() {
            shadow.spread = spread_or_color
                .parse::<f32>()
                .map_err(|_| ParseShadowError)?;
        } else {
            color_string.push_str(spread_or_color);
        }
        color_string.push_str(shadow_values.collect::<Vec<&str>>().join(" ").as_str());

        shadow.color = Color::parse(color_string.as_str(), None).map_err(|_| ParseShadowError)?;

        shadow.x *= scale_factor;
        shadow.y *= scale_factor;
        shadow.blur *= scale_factor;
        shadow.spread *= scale_factor;

        Ok(shadow)
    }
}

impl fmt::Display for Shadow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.inset {
			f.write_str("inset ")?;
		}

		write!(
			f,
			"{} {} {} {} rgb({}, {}, {}, {})",
			self.x,
			self.y,
			self.blur,
			self.spread,
			self.color.r(),
            self.color.g(),
            self.color.b(),
            self.color.a(),
		)
    }
}