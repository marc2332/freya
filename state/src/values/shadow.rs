use crate::{Fill, Parse};
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
    pub fill: Fill,
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

        if let Ok(spread) = spread_or_color.parse::<f32>() {
            shadow.spread = spread;
        } else {
            color_string.push_str(spread_or_color);
            color_string.push_str(" ");
        }
        color_string.push_str(shadow_values.collect::<Vec<&str>>().join(" ").as_str());

        shadow.fill = Fill::parse(color_string.as_str()).map_err(|_| ParseShadowError)?;

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
