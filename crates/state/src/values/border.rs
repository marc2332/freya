use std::fmt;

use freya_engine::prelude::Color;
use torin::scaled::Scaled;

use crate::{
    Fill,
    Parse,
    ParseError,
    Parser,
    Token,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Border {
    pub fill: Fill,
    pub width: BorderWidth,
    pub alignment: BorderAlignment,
}

impl Border {
    #[inline]
    pub fn is_visible(&self) -> bool {
        !(self.width.top == 0.0
            && self.width.left == 0.0
            && self.width.bottom == 0.0
            && self.width.right == 0.0)
            && self.fill != Fill::Color(Color::TRANSPARENT)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct BorderWidth {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Parse for BorderWidth {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        Ok(
            match (
                parser.consume_map(Token::try_as_f32)?,
                parser.consume_map(Token::try_as_f32).ok(),
                parser.consume_map(Token::try_as_f32).ok(),
                parser.consume_map(Token::try_as_f32).ok(),
            ) {
                (top, Some(right), Some(bottom), Some(left)) => Self {
                    top,
                    right,
                    bottom,
                    left,
                },
                (top, Some(horizontal), Some(bottom), None) => Self {
                    top,
                    right: horizontal,
                    bottom,
                    left: horizontal,
                },
                (vertical, Some(horizontal), None, None) => Self {
                    top: vertical,
                    right: horizontal,
                    bottom: vertical,
                    left: horizontal,
                },
                (all, None, None, None) => Self {
                    top: all,
                    right: all,
                    bottom: all,
                    left: all,
                },
                _ => return Err(ParseError),
            },
        )
    }
}

impl Scaled for BorderWidth {
    fn scale(&mut self, scale_factor: f32) {
        self.top *= scale_factor;
        self.left *= scale_factor;
        self.bottom *= scale_factor;
        self.right *= scale_factor;
    }
}

impl fmt::Display for BorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.top, self.right, self.bottom, self.left,
        )
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderAlignment {
    #[default]
    Inner,
    Outer,
    Center,
}

impl Parse for BorderAlignment {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.try_as_str().and_then(|value| match value {
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

impl Parse for Border {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        if parser.try_consume(&Token::ident("none")) {
            return Ok(Self::default());
        }

        Ok(Border {
            width: BorderWidth::from_parser(parser)?,
            alignment: BorderAlignment::from_parser(parser)?,
            fill: Fill::from_parser(parser)?,
        })
    }
}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.width, self.alignment, self.fill)
    }
}

impl Scaled for Border {
    fn scale(&mut self, scale_factor: f32) {
        self.width.scale(scale_factor);
    }
}
