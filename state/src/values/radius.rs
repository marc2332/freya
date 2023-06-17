use torin::radius::Radius;
use crate::Parse;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRadiusError;

impl Parse for Radius {
    type Err = ParseRadiusError;

    fn parse(value: &str, scale_factor: Option<f32>) -> Option<Radius> {
        let mut radius_config = Radius::default();
        let mut radius = value.split_ascii_whitespace();
    
        match radius.clone().count() {
            // Same in all corners
            1 => {
                radius_config.fill_all(radius.next()?.parse::<f32>().ok()? * scale_factor);
            }
            // By Top and Bottom
            2 => {
                // Top
                radius_config.fill_top(radius.next()?.parse::<f32>().ok()? * scale_factor);
    
                // Bottom
                radius_config.fill_bottom(radius.next()?.parse::<f32>().ok()? * scale_factor)
            }
            // Each corner
            4 => {
                radius_config = Radius::new(
                    radius.next()?.parse::<f32>().ok()? * scale_factor,
                    radius.next()?.parse::<f32>().ok()? * scale_factor,
                    radius.next()?.parse::<f32>().ok()? * scale_factor,
                    radius.next()?.parse::<f32>().ok()? * scale_factor,
                );
            }
            _ => {}
        }
    
        Some(radius_config)
    }    
}