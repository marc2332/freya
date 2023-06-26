use crate::{Fill, Parse};
use std::fmt;
use torin::scaled::Scaled;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Border {
    pub fill: Fill,
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

    fn parse(value: &str) -> Result<Self, Self::Err> {
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

    fn parse(value: &str) -> Result<Self, Self::Err> {
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
            fill: Fill::parse(&border_values.collect::<Vec<&str>>().join(" "))
                .map_err(|_| ParseBorderError)?,
            alignment: BorderAlignment::default(),
        })
    }
}

impl Scaled for Border {
    fn scale(&mut self, scale_factor: f32) {
        self.width *= scale_factor;
    }
}
