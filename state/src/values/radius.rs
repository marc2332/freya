use crate::Parse;
use torin::radius::Radius;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRadiusError;

impl Parse for Radius {
    type Err = ParseRadiusError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        let mut radius = Radius::default();

        let mut values = value.split_ascii_whitespace();

        match values.clone().count() {
            // Same in all corners
            1 => {
                radius.fill_all(
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                );
            }
            // By Top and Bottom
            2 => {
                // Top
                radius.fill_top(
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                );

                // Bottom
                radius.fill_bottom(
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                )
            }
            // Each corner
            4 => {
                radius = Radius::new(
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?,
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?,
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?,
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?,
                );
            }
            _ => {}
        }

        Ok(radius)
    }
}