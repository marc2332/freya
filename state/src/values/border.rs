use crate::{fmt_color_rgba, parse_color};
use skia_safe::Color;
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

impl str::FromStr for BorderAlignment {
    type Err = ParseBorderAlignmentError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut border_align_value = value.split_ascii_whitespace();

        match border_align_value.next() {
            Some("inner") => Ok(BorderAlignment::Inner),
            Some("outer") => Ok(BorderAlignment::Outer),
            Some("center") => Ok(BorderAlignment::Center),
            _ => Ok(BorderAlignment::default()),
        }
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

pub struct ParseBorderError;

impl str::FromStr for Border {
    type Err = ParseBorderError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut border_values = value.split_ascii_whitespace();

        Ok(Border {
            width: border_values
                .next()
                .ok_or(ParseBorderError)?
                .parse::<f32>()
                .map_err(|_| ParseBorderError)?,
            style: match border_values.next().ok_or(ParseBorderError)? {
                "solid" => BorderStyle::Solid,
                _ => BorderStyle::None,
            },
            color: parse_color(&border_values.collect::<Vec<&str>>().join(" "))
                .ok_or(ParseBorderError)?,
            alignment: BorderAlignment::default(),
        })
    }
}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}, {}, {}", self.width, self.style, fmt_color_rgba(&self.color))
    }
}