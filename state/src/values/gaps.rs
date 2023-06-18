use crate::Parse;
use torin::gaps::Gaps;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGapError;

impl Parse for Gaps {
    type Err = ParseGapError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        let mut paddings = Gaps::default();

        let mut values = value.split_ascii_whitespace();

        match values.clone().count() {
            // Same in each directions
            1 => {
                paddings.fill_all(
                    values
                        .next()
                        .ok_or(ParseGapError)?
                        .parse::<f32>()
                        .map_err(|_| ParseGapError)?,
                );
            }
            // By vertical and horizontal
            2 => {
                // Vertical
                paddings.fill_vertical(
                    values
                        .next()
                        .ok_or(ParseGapError)?
                        .parse::<f32>()
                        .map_err(|_| ParseGapError)?,
                );

                // Horizontal
                paddings.fill_horizontal(
                    values
                        .next()
                        .ok_or(ParseGapError)?
                        .parse::<f32>()
                        .map_err(|_| ParseGapError)?,
                )
            }
            // Each directions
            4 => {
                paddings = Gaps::new(
                    values
                        .next()
                        .ok_or(ParseGapError)?
                        .parse::<f32>()
                        .map_err(|_| ParseGapError)?,
                    values
                        .next()
                        .ok_or(ParseGapError)?
                        .parse::<f32>()
                        .map_err(|_| ParseGapError)?,
                    values
                        .next()
                        .ok_or(ParseGapError)?
                        .parse::<f32>()
                        .map_err(|_| ParseGapError)?,
                    values
                        .next()
                        .ok_or(ParseGapError)?
                        .parse::<f32>()
                        .map_err(|_| ParseGapError)?,
                );
            }
            _ => {}
        }

        Ok(paddings)
    }
}
