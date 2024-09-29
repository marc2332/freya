use std::fmt;

use freya_engine::prelude::*;

use crate::{
    parse_angle,
    parse_func,
    Parse,
    ParseError,
    Parser,
    Token,
};

pub trait DisplayColor {
    fn fmt_rgb(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn fmt_hsl(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl Parse for Color {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        if parser.check(&Token::Pound) {
            parse_hex_color(parser)
        } else if parser.check(&Token::ident("rgb")) {
            parse_rgb(parser)
        } else if parser.check(&Token::ident("hsl")) {
            parse_hsl(parser)
        } else {
            let value = parser.consume_if(Token::is_ident).map(Token::into_string)?;

            match value.as_str() {
                "rgb" => parse_rgb(parser),
                "hsl" => parse_hsl(parser),
                "red" => Ok(Color::RED),
                "green" => Ok(Color::GREEN),
                "blue" => Ok(Color::BLUE),
                "yellow" => Ok(Color::YELLOW),
                "black" => Ok(Color::BLACK),
                "gray" => Ok(Color::GRAY),
                "white" => Ok(Color::WHITE),
                "orange" => Ok(Color::from_rgb(255, 165, 0)),
                "transparent" | "none" => Ok(Color::TRANSPARENT),
                _ => Err(ParseError),
            }
        }
    }
}

impl DisplayColor for Color {
    fn fmt_rgb(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rgb({}, {}, {}, {})",
            self.r(),
            self.g(),
            self.b(),
            self.a()
        )
    }

    fn fmt_hsl(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // HSV to HSL conversion
        let hsv = self.to_hsv();
        let l = hsv.v - (hsv.v * hsv.s / 2.0);
        let s = if l == 1.0 || l == 0.0 {
            0.0
        } else {
            (hsv.v - l) / f32::min(l, 1.0 - l)
        };

        write!(
            f,
            "hsl({}deg, {}%, {}%, {}%)",
            hsv.h,
            s * 100.0,
            l * 100.0,
            (self.a() as f32 / 128.0) * 100.0
        )
    }
}

fn parse_rgb(parser: &mut Parser) -> Result<Color, ParseError> {
    parse_func(parser, "rgb", |parser| {
        let red = parser.consume_map(Token::try_as_u8)?;

        parser.consume(&Token::Comma)?;

        let green = parser.consume_map(Token::try_as_u8)?;

        parser.consume(&Token::Comma)?;

        let blue = parser.consume_map(Token::try_as_u8)?;

        Ok(if parser.try_consume(&Token::Comma) {
            let alpha = parser.consume_map(Token::try_as_u8)?;

            Color::from_argb(alpha, red, green, blue)
        } else {
            Color::from_rgb(red, green, blue)
        })
    })
}

fn parse_hsl(parser: &mut Parser) -> Result<Color, ParseError> {
    parse_func(parser, "hsl", |parser| {
        let h = parse_angle(parser)?;

        if !(0.0..=360.0).contains(&h) {
            return Err(ParseError);
        }

        parser.consume(&Token::Comma)?;

        let mut s = parser
            .consume_if(Token::is_i64)
            .map(Token::into_f32)
            .and_then(|value| {
                if (0.0..=100.0).contains(&value) {
                    Ok(value / 100.0)
                } else {
                    Err(ParseError)
                }
            })?;

        parser.consume(&Token::Percent)?;
        parser.consume(&Token::Comma)?;

        let mut l = parser
            .consume_if(Token::is_i64)
            .map(Token::into_f32)
            .and_then(|value| {
                if (0.0..=100.0).contains(&value) {
                    Ok(value / 100.0)
                } else {
                    Err(ParseError)
                }
            })?;

        parser.consume(&Token::Percent)?;

        let a = if parser.consume(&Token::Comma).is_ok() {
            let value = parser
                .consume_if(Token::is_i64)
                .map(Token::into_f32)
                .and_then(|value| {
                    if (0.0..=100.0).contains(&value) {
                        Ok(value / 100.0)
                    } else {
                        Err(ParseError)
                    }
                })?;

            parser.consume(&Token::Percent)?;

            Some(value)
        } else {
            None
        };

        parser.consume(&Token::ParenClose)?;

        // HSL to HSV Conversion
        l *= 2.0;
        s *= if l <= 1.0 { l } else { 2.0 - l };

        let v = (l + s) / 2.0;

        s = (2.0 * s) / (l + s);

        let hsv = HSV::from((h, s, v));

        // Handle alpha formatting and convert to ARGB
        Ok(a.map(|a| hsv.to_color((a * 255.0).round() as u8))
            .unwrap_or_else(|| hsv.to_color(255)))
    })
}

fn parse_hex_color(parser: &mut Parser) -> Result<Color, ParseError> {
    parser.consume(&Token::Pound)?;

    let hex = parser.consume_if(Token::is_ident).map(Token::into_string)?;

    if ![6, 8].contains(&hex.len()) {
        return Err(ParseError);
    }

    let value = i64::from_str_radix(&hex, 16).map_err(|_| ParseError)?;

    let a = if hex.len() == 8 {
        Some(u8::try_from((value >> 24) & 0xFF).map_err(|_| ParseError)?)
    } else {
        None
    };

    let r = u8::try_from((value >> 16) & 0xFF).map_err(|_| ParseError)?;
    let g = u8::try_from((value >> 8) & 0xFF).map_err(|_| ParseError)?;
    let b = u8::try_from(value & 0xFF).map_err(|_| ParseError)?;

    Ok(a.map(|a| Color::from_argb(a, r, g, b))
        .unwrap_or_else(|| Color::from_rgb(r, g, b)))
}
