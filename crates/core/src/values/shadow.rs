use torin::scaled::Scaled;

use super::Fill;
use crate::parsing::{
    ExtSplit,
    Parse,
    ParseError,
};

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

impl Parse for Shadow {
    fn parse(value: &str) -> Result<Self, ParseError> {
        let mut shadow_values = value.split_ascii_whitespace_excluding_group('(', ')');
        let mut shadow = Shadow::default();

        let first = shadow_values.next().ok_or(ParseError)?;

        if first == "inset" {
            shadow.position = ShadowPosition::Inset;
            shadow.x = shadow_values
                .next()
                .ok_or(ParseError)?
                .parse::<f32>()
                .map_err(|_| ParseError)?;
        } else {
            shadow.x = first.parse::<f32>().map_err(|_| ParseError)?;
        }

        shadow.y = shadow_values
            .next()
            .ok_or(ParseError)?
            .parse::<f32>()
            .map_err(|_| ParseError)?;
        shadow.blur = shadow_values
            .next()
            .ok_or(ParseError)?
            .parse::<f32>()
            .map_err(|_| ParseError)?;

        let spread_or_fill = shadow_values.next().ok_or(ParseError)?;

        let mut already_filled = false;
        if let Ok(spread) = spread_or_fill.parse::<f32>() {
            shadow.spread = spread;
        } else {
            already_filled = true;
            shadow.fill = Fill::parse(spread_or_fill).map_err(|_| ParseError)?;
        }

        if let Some(fill) = shadow_values.next() {
            if !already_filled {
                shadow.fill = Fill::parse(fill).map_err(|_| ParseError)?
            } else {
                return Err(ParseError);
            }
        }

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
