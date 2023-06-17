use crate::Parse;
use torin::padding::Paddings;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePaddingsError;

impl Parse for Paddings {
    type Err = ParsePaddingsError;

    fn parse(value: &str, scale_factor: Option<f32>) -> Result<Self, Self::Err> {
        let mut paddings = Paddings::default();

        let mut values = value.split_ascii_whitespace();
        let scale_factor = scale_factor.unwrap_or(1.0);

        match values.clone().count() {
            // Same in each directions
            1 => {
                paddings.fill_all(
                    values
                        .next()
                        .ok_or(ParsePaddingsError)?
                        .parse::<f32>()
                        .map_err(|_| ParsePaddingsError)?
                        * scale_factor,
                );
            }
            // By vertical and horizontal
            2 => {
                // Vertical
                paddings.fill_vertical(
                    values
                        .next()
                        .ok_or(ParsePaddingsError)?
                        .parse::<f32>()
                        .map_err(|_| ParsePaddingsError)?
                        * scale_factor,
                );

                // Horizontal
                paddings.fill_horizontal(
                    values
                        .next()
                        .ok_or(ParsePaddingsError)?
                        .parse::<f32>()
                        .map_err(|_| ParsePaddingsError)?
                        * scale_factor,
                )
            }
            // Each directions
            4 => {
                paddings = Paddings::new(
                    values
                        .next()
                        .ok_or(ParsePaddingsError)?
                        .parse::<f32>()
                        .map_err(|_| ParsePaddingsError)?
                        * scale_factor,
                    values
                        .next()
                        .ok_or(ParsePaddingsError)?
                        .parse::<f32>()
                        .map_err(|_| ParsePaddingsError)?
                        * scale_factor,
                    values
                        .next()
                        .ok_or(ParsePaddingsError)?
                        .parse::<f32>()
                        .map_err(|_| ParsePaddingsError)?
                        * scale_factor,
                    values
                        .next()
                        .ok_or(ParsePaddingsError)?
                        .parse::<f32>()
                        .map_err(|_| ParsePaddingsError)?
                        * scale_factor,
                );
            }
            _ => {}
        }

        Ok(paddings)
    }
}
