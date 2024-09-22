use std::fmt;

use freya_engine::prelude::*;

use crate::{
    Parse,
    ParseError,
};

pub trait DisplayColor {
    fn fmt_rgb(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn fmt_hsl(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl Parse for Color {
    fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            "yellow" => Ok(Color::YELLOW),
            "black" => Ok(Color::BLACK),
            "gray" => Ok(Color::GRAY),
            "white" => Ok(Color::WHITE),
            "orange" => Ok(Color::from_rgb(255, 165, 0)),
            "transparent" | "none" => Ok(Color::TRANSPARENT),
            _ => {
                if value.starts_with("hsl(") {
                    parse_hsl(value)
                } else if value.starts_with("rgb(") {
                    parse_rgb(value)
                } else if value.starts_with('#') {
                    parse_hex_color(value)
                } else {
                    Err(ParseError)
                }
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

fn parse_rgb(color: &str) -> Result<Color, ParseError> {
    if !color.ends_with(')') {
        return Err(ParseError);
    }

    let color = color.replacen("rgb(", "", 1).replacen(')', "", 1);

    let mut colors = color.split(',');

    let r = colors
        .next()
        .ok_or(ParseError)?
        .trim()
        .parse::<u8>()
        .map_err(|_| ParseError)?;
    let g = colors
        .next()
        .ok_or(ParseError)?
        .trim()
        .parse::<u8>()
        .map_err(|_| ParseError)?;
    let b = colors
        .next()
        .ok_or(ParseError)?
        .trim()
        .parse::<u8>()
        .map_err(|_| ParseError)?;
    let a: Option<&str> = colors.next();

    // There should not be more than 4 components.
    if colors.next().is_some() {
        return Err(ParseError);
    }

    if let Some(a) = a {
        let alpha_trimmed = a.trim();
        if let Ok(u8_alpha) = alpha_trimmed.parse::<u8>() {
            Ok(Color::from_argb(u8_alpha, r, g, b))
        } else if let Ok(f32_alpha) = alpha_trimmed.parse::<f32>() {
            let a = (255.0 * f32_alpha).clamp(0.0, 255.0).round() as u8;
            Ok(Color::from_argb(a, r, g, b))
        } else {
            Err(ParseError)
        }
    } else {
        Ok(Color::from_rgb(r, g, b))
    }
}

fn parse_hsl(color: &str) -> Result<Color, ParseError> {
    if !color.ends_with(')') {
        return Err(ParseError);
    }

    let color = color.replacen("hsl(", "", 1).replacen(')', "", 1);
    let mut colors = color.split(',');

    // Get each color component as a string
    let h_str = colors.next().ok_or(ParseError)?.trim();
    let s_str = colors.next().ok_or(ParseError)?.trim();
    let l_str = colors.next().ok_or(ParseError)?.trim();
    let a_str: Option<&str> = colors.next();

    // Ensure correct units and lengths.
    if colors.next().is_some()
        || !h_str.ends_with("deg")
        || !s_str.ends_with('%')
        || !l_str.ends_with('%')
    {
        return Err(ParseError);
    }

    // S, L and A can end in percentage, otherwise its 0.0 - 1.0
    let h = h_str
        .replacen("deg", "", 1)
        .parse::<f32>()
        .map_err(|_| ParseError)?;
    let mut s = s_str
        .replacen('%', "", 1)
        .parse::<f32>()
        .map_err(|_| ParseError)?
        / 100.0;
    let mut l = l_str
        .replacen('%', "", 1)
        .parse::<f32>()
        .map_err(|_| ParseError)?
        / 100.0;

    // HSL to HSV Conversion
    l *= 2.0;
    s *= if l <= 1.0 { l } else { 2.0 - l };
    let v = (l + s) / 2.0;
    s = (2.0 * s) / (l + s);
    let hsv = HSV::from((h, s, v));

    // Handle alpha formatting and convert to ARGB
    if let Some(a_str) = a_str {
        if !s_str.ends_with('%') {
            return Err(ParseError);
        }

        let a = a_str
            .trim()
            .replace('%', "")
            .parse::<f32>()
            .map_err(|_| ParseError)?
            / 100.0;

        Ok(hsv.to_color((a * 255.0).round() as u8))
    } else {
        Ok(hsv.to_color(255))
    }
}

fn parse_hex_color(color: &str) -> Result<Color, ParseError> {
    if color.len() == 7 {
        let r = u8::from_str_radix(&color[1..3], 16).map_err(|_| ParseError)?;
        let g = u8::from_str_radix(&color[3..5], 16).map_err(|_| ParseError)?;
        let b = u8::from_str_radix(&color[5..7], 16).map_err(|_| ParseError)?;
        Ok(Color::from_rgb(r, g, b))
    } else {
        Err(ParseError)
    }
}
