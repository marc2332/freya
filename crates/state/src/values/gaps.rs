use torin::gaps::Gaps;

use crate::{
    Parse,
    ParseError,
};

impl Parse for Gaps {
    fn parse(value: &str) -> Result<Self, ParseError> {
        let mut paddings = Gaps::default();

        let mut values = value.split_ascii_whitespace();

        match values.clone().count() {
            // Same in each directions
            1 => {
                paddings.fill_all(
                    values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );
            }
            // By vertical and horizontal
            2 => {
                // Vertical
                paddings.fill_vertical(
                    values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );

                // Horizontal
                paddings.fill_horizontal(
                    values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                )
            }
            // Individual vertical but same horizontal
            3 => {
                let top = values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;
                let left_and_right = values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;
                let bottom = values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;
                paddings = Gaps::new(top, left_and_right, bottom, left_and_right);
            }
            // Each directions
            4 => {
                paddings = Gaps::new(
                    values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                    values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                    values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                    values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );
            }
            _ => {}
        }

        Ok(paddings)
    }
}
