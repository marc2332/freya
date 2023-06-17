use crate::Parse;
use skia_safe::{Color};
use std::{fmt, str};

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Border {
    pub color: Color,
    pub style: BorderStyle,
    pub width: f32,
    pub alignment: BorderAlignment,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderAlignment {
    #[default]
    Inner,
    Outer,
    Center,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBorderAlignmentError;

impl Parse for BorderAlignment {
    type Err = ParseBorderAlignmentError;

    fn parse(value: &str, _scale_factor: Option<f32>) -> Result<Self, Self::Err> {
        Ok(match value {
            "inner" => BorderAlignment::Inner,
            "outer" => BorderAlignment::Outer,
            "center" => BorderAlignment::Center,
            _ => BorderAlignment::default(),
        })
    }
}

impl fmt::Display for BorderAlignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(match self {
			BorderAlignment::Inner => "inner",
			BorderAlignment::Outer => "outer",
			BorderAlignment::Center => "center",
		})
    }
}

impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(match self {
			BorderStyle::Solid => "solid",
			BorderStyle::None => "none",
		})
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBorderError;

impl Parse for Border {
    type Err = ParseBorderError;

    fn parse(value: &str, scale_factor: Option<f32>) -> Result<Self, Self::Err> {
        let mut border_values = value.split_ascii_whitespace();

        Ok(Border {
            width: border_values
                .next()
                .ok_or(ParseBorderError)?
                .parse::<f32>()
                .map_err(|_| ParseBorderError)? * scale_factor.unwrap_or(1.0),
            style: match border_values.next().ok_or(ParseBorderError)? {
                "solid" => BorderStyle::Solid,
                _ => BorderStyle::None,
            },
            color: Color::parse(&border_values.collect::<Vec<&str>>().join(" "), None).map_err(|_| ParseBorderError)?,
            alignment: BorderAlignment::default(),
        })
    }
}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} rgb({}, {}, {}, {})", self.width, self.style, self.color.r(), self.color.g(), self.color.b(), self.color.a())
    }
}