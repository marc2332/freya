use std::fmt;

use torin::scaled::Scaled;

use crate::{
    Fill,
    Parse,
    ParseError,
};

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

impl Parse for BorderAlignment {
    fn parse(value: &str) -> Result<Self, ParseError> {
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

impl Parse for Border {
    fn parse(value: &str) -> Result<Self, ParseError> {
        if value == "none" {
            return Ok(Self::default());
        }

        let mut border_values = value.split_ascii_whitespace();

        Ok(Border {
            width: border_values
                .next()
                .ok_or(ParseError)?
                .parse::<f32>()
                .map_err(|_| ParseError)?,
            style: match border_values.next().ok_or(ParseError)? {
                "solid" => BorderStyle::Solid,
                _ => BorderStyle::None,
            },
            fill: Fill::parse(&border_values.collect::<Vec<&str>>().join(" "))
                .map_err(|_| ParseError)?,
            alignment: BorderAlignment::default(),
        })
    }
}

impl Scaled for Border {
    fn scale(&mut self, scale_factor: f32) {
        self.width *= scale_factor;
    }
}
