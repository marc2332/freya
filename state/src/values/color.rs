use crate::Parse;
use skia_safe::{Color, HSV};
use std::fmt;

pub trait DisplayColor {
    fn fmt_rgb(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn fmt_hsl(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorError;

impl Parse for Color {
    type Err = ParseColorError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        match value {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            "yellow" => Ok(Color::YELLOW),
            "black" => Ok(Color::BLACK),
            "gray" => Ok(Color::GRAY),
            "white" => Ok(Color::WHITE),
            "orange" => Ok(Color::from_rgb(255, 165, 0)),
            "transparent" => Ok(Color::TRANSPARENT),
            _ => {
                if value.starts_with("hsl(") {
                    parse_hsl(value)
                } else if value.starts_with("rgb(") {
                    parse_rgb(value)
                } else {
                    Err(ParseColorError)
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

fn parse_rgb(color: &str) -> Result<Color, ParseColorError> {
    if !color.ends_with(')') {
        return Err(ParseColorError);
    }

    let color = color.replacen("rgb(", "", 1).replacen(')', "", 1);

    let mut colors = color.split(',');

    let r = colors
        .next()
        .ok_or(ParseColorError)?
        .trim()
        .parse::<u8>()
        .map_err(|_| ParseColorError)?;
    let g = colors
        .next()
        .ok_or(ParseColorError)?
        .trim()
        .parse::<u8>()
        .map_err(|_| ParseColorError)?;
    let b = colors
        .next()
        .ok_or(ParseColorError)?
        .trim()
        .parse::<u8>()
        .map_err(|_| ParseColorError)?;
    let a: Option<&str> = colors.next();

    // There should not be more than 4 components.
    if colors.next().is_some() {
        return Err(ParseColorError);
    }

    if let Some(a) = a {
        let a = a.trim().parse::<u8>().map_err(|_| ParseColorError)?;
        Ok(Color::from_argb(a, r, g, b))
    } else {
        Ok(Color::from_rgb(r, g, b))
    }
}

fn parse_hsl(color: &str) -> Result<Color, ParseColorError> {
    if !color.ends_with(')') {
        return Err(ParseColorError);
    }

    let color = color.replacen("hsl(", "", 1).replacen(')', "", 1);
    let mut colors = color.split(',');

    // Get each color component
    let h = colors
        .next()
        .ok_or(ParseColorError)?
        .trim()
        .replace("deg", "")
        .parse::<f32>()
        .map_err(|_| ParseColorError)?;
    let s_str = colors.next().ok_or(ParseColorError)?.trim();
    let l_str = colors.next().ok_or(ParseColorError)?.trim();
    let a_str: Option<&str> = colors.next();

    // There should not be more than 4 components.
    if colors.next().is_some() {
        return Err(ParseColorError);
    }

    // S, L and A can end in percentage, otherwise its 0.0 - 1.0
    let mut s = if s_str.ends_with('%') {
        s_str
            .replace('%', "")
            .parse::<f32>()
            .map_err(|_| ParseColorError)?
            / 100.0
    } else {
        s_str.parse::<f32>().map_err(|_| ParseColorError)?
    };

    let mut l = if l_str.ends_with('%') {
        l_str
            .replace('%', "")
            .parse::<f32>()
            .map_err(|_| ParseColorError)?
            / 100.0
    } else {
        l_str.parse::<f32>().map_err(|_| ParseColorError)?
    };

    // HSL to HSV Conversion
    l *= 2.0;
    s *= if l <= 1.0 { l } else { 2.0 - l };
    let v = (l + s) / 2.0;
    s = (2.0 * s) / (l + s);
    let hsv = HSV { h, s, v };

    // Handle alpha formatting and convert to ARGB
    if let Some(a_str) = a_str {
        let a = if a_str.ends_with('%') {
            a_str
                .trim()
                .replace('%', "")
                .parse::<f32>()
                .map_err(|_| ParseColorError)?
                / 100.0
        } else {
            a_str.trim().parse::<f32>().map_err(|_| ParseColorError)?
        };

        Ok(hsv.to_color((a * 255.0).round() as u8))
    } else {
        Ok(hsv.to_color(255))
    }
}
