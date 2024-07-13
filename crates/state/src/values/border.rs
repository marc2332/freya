use std::fmt;

use torin::scaled::Scaled;

use crate::{
    Fill,
    Parse,
    ParseError,
    Parser,
    Token,
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
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.as_string().and_then(|value| match value {
                "inner" => Some(BorderAlignment::Inner),
                "outer" => Some(BorderAlignment::Outer),
                "center" => Some(BorderAlignment::Center),
                _ => None,
            })
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
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        if parser.try_consume(&Token::ident("none")) {
            return Ok(Self::default());
        }

        Ok(Border {
            width: parser.consume_map(Token::as_float)?,
            style: parser.consume_map(|value| {
                value.as_string().and_then(|value| match value {
                    "none" => Some(BorderStyle::None),
                    "solid" => Some(BorderStyle::Solid),
                    _ => None,
                })
            })?,
            fill: Fill::parse(parser)?,
            alignment: BorderAlignment::default(),
        })
    }
}

impl Scaled for Border {
    fn scale(&mut self, scale_factor: f32) {
        self.width *= scale_factor;
    }
}
