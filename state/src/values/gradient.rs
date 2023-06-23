use skia_safe::Color;
use crate::{Parse, ExtSplit};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ColorStop {
	pub color: Color,
	pub offset: Option<f32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorStopError;

impl Parse for ColorStop {
    type Err = ParseColorStopError;
    
    fn parse(value: &str) -> Result<Self, Self::Err> {
		let mut split = value.split_ascii_whitespace_excluding_group('(', ')');
		let color_str = split.next().ok_or(ParseColorStopError)?;
		let offset_str = split.next();

		Ok(ColorStop {
			color: Color::parse(color_str).map_err(|_| ParseColorStopError)?,
			offset: if let Some(offset_str) = offset_str {
				let mut offset = offset_str.replace("%", "").parse::<f32>().map_err(|_| ParseColorStopError)?;
				if offset_str.ends_with("%") {
					offset /= 100.0
				}

				Some(offset)
			} else {
				None
			}
		})
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinearGradient {
	pub stops: Vec<ColorStop>,
	pub angle: f32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLinearGradientError;

impl Parse for LinearGradient {
    type Err = ParseLinearGradientError;
    
    fn parse(value: &str) -> Result<Self, Self::Err> {
		if !value.starts_with("linear-gradient") || !value.ends_with(")") {
			return Err(ParseLinearGradientError)
		}
		
		let mut gradient = LinearGradient::default();
		let mut value = value.replacen("linear-gradient(", "", 1);
		value.remove(value.rfind(")").ok_or(ParseLinearGradientError)?);

		let mut split = value.split_excluding_group(',', '(', ')');

		let angle_or_first_stop = split
			.next()
			.ok_or(ParseLinearGradientError)?
			.trim()
			.replace("deg", "");
		
		if let Ok(angle) = angle_or_first_stop.replace("deg", "").parse::<f32>() {
			gradient.angle = angle;
		} else {
			gradient.stops.push(ColorStop::parse(angle_or_first_stop.as_str()).map_err(|_| ParseLinearGradientError)?);
		}

		for stop in split {
			gradient.stops.push(ColorStop::parse(stop).map_err(|_| ParseLinearGradientError)?);
		}

		Ok(gradient)
    }
}

#[test]
fn test() {
	println!("{:?}", LinearGradient::parse("linear-gradient(rgb(255, 255, 255) 50%, blue, green)"));
}