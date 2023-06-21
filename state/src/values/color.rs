use crate::Parse;
use skia_safe::{Color, HSV};

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
                if value.starts_with("hsl") {
                    parse_hsl(value)
                } else {
                    parse_rgb(value)
                }
            }
        }
    }
}

fn parse_rgb(color: &str) -> Result<Color, ParseColorError> {
    let color = color.replace("rgb(", "").replace(')', "");
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

    if let Some(a) = a {
        let a = a.trim().parse::<u8>().map_err(|_| ParseColorError)?;
        Ok(Color::from_argb(a, r, g, b))
    } else {
        Ok(Color::from_rgb(r, g, b))
    }
}

fn parse_hsl(color: &str) -> Result<Color, ParseColorError> {
    let color = color.replace("hsl(", "").replace(')', "");
    let mut colors = color.split(',');

    // Get each color component
    let h = colors.next()
        .ok_or(ParseColorError)?
        .trim().replace("deg", "")
        .parse::<f32>()
        .map_err(|_| ParseColorError)?;
    let s_str = colors.next().ok_or(ParseColorError)?.trim();
    let l_str = colors.next().ok_or(ParseColorError)?.trim();
    let a_str: Option<&str> = colors.next();

    // S, L and A can end in percentage, otherwise its 0.0 - 1.0
    let mut s = if s_str.ends_with("%") {
        s_str.replace("%", "").parse::<f32>().map_err(|_| ParseColorError)? / 100.0
    } else {
        s_str.parse::<f32>().map_err(|_| ParseColorError)?
    };

    let mut l = if l_str.ends_with("%") {
        l_str.replace("%", "").parse::<f32>().map_err(|_| ParseColorError)? / 100.0
    } else {
        l_str.parse::<f32>().map_err(|_| ParseColorError)?
    };

    // HSL to HSV Conversion
    l *= 2.0;
    s *= if l <= 1.0 {
        l
    } else {
        2.0 - l
    };
    let v = (l + s) / 2.0;
    s = (2.0 * s) / (l + s);
    let hsv = HSV { h, s, v };

    // Handle alpha formatting and convert to ARGB
    if let Some(a_str) = a_str {
        let a = if a_str.ends_with("%") {
            a_str.trim().replace("%", "").parse::<f32>().map_err(|_| ParseColorError)? / 100.0
        } else {
            a_str.trim().parse::<f32>().map_err(|_| ParseColorError)?
        };

        Ok(hsv.to_color((a * 255.0).round() as u8))
    } else {
        Ok(hsv.to_color(255))
    }
}
