use crate::Parse;
use skia_safe::{textlayout::TextShadow, Color};
use torin::scaled::Scaled;

#[derive(Default, Clone, Debug, PartialEq)]
pub enum ShadowPosition {
    #[default]
    Normal,
    Inset,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Shadow {
    pub position: ShadowPosition,
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

    fn parse(value: &str) -> Result<Self, Self::Err> {
        let mut shadow_values = value.split_ascii_whitespace();
        let mut shadow = Shadow::default();

        let first = shadow_values.next().ok_or(ParseShadowError)?;

        if first == "inset" {
            shadow.position = ShadowPosition::Inset;
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

        shadow.color = Color::parse(color_string.as_str()).map_err(|_| ParseShadowError)?;

        Ok(shadow)
    }
}

impl Scaled for Shadow {
    fn scale(&mut self, scale_factor: f32) {
        self.x *= scale_factor;
        self.y *= scale_factor;
        self.spread *= scale_factor;
        self.blur *= scale_factor;
    }
}

// Same as shadow, but no inset or spread.
impl Parse for TextShadow {
    type Err = ParseShadowError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        let mut shadow_values = value.split_ascii_whitespace();
        Ok(TextShadow {
            offset: (
                shadow_values
                    .next()
                    .ok_or(ParseShadowError)?
                    .parse::<f32>()
                    .map_err(|_| ParseShadowError)?,
                shadow_values
                    .next()
                    .ok_or(ParseShadowError)?
                    .parse::<f32>()
                    .map_err(|_| ParseShadowError)?,
            ).into(),
            blur_sigma: shadow_values
                .next()
                .ok_or(ParseShadowError)?
                .parse::<f64>()
                .map_err(|_| ParseShadowError)? / 2.0,
            color: Color::parse(shadow_values
                .collect::<Vec<&str>>()
                .join(" ")
                .as_str()).map_err(|_| ParseShadowError)?,
        })
    }
}

pub fn split_shadows(value: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut in_parenthesis = false;

    for character in value.chars() {
        if character == '(' {
            in_parenthesis = true;
        } else if character == ')' {
            in_parenthesis = false;
        }

        if character == ',' && !in_parenthesis {
            chunks.push(std::mem::take(&mut current));
        } else {
            current.push(character);
        }
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}
