use std::fmt;

use freya_engine::prelude::{
    SkColor,
    HSV,
    RGB,
};

use crate::parsing::{
    Parse,
    ParseError,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Color(u32);

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color(value)
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl From<Color> for SkColor {
    fn from(value: Color) -> Self {
        Self::new(value.0)
    }
}

impl From<Color> for freya_engine::prelude::Color4f {
    fn from(value: Color) -> Self {
        freya_engine::prelude::Color4f::new(
            value.r() as f32,
            value.g() as f32,
            value.b() as f32,
            value.a() as f32,
        )
    }
}

impl From<SkColor> for Color {
    fn from(value: SkColor) -> Self {
        let a = value.a();
        let r = value.r();
        let g = value.g();
        let b = value.b();
        Color::from_argb(a, r, g, b)
    }
}

impl Color {
    pub const TRANSPARENT: Self = Color::new(0);
    pub const BLACK: Self = Color::new(4278190080);
    pub const DARK_GRAY: Self = Color::new(4282664004);
    pub const GRAY: Self = Color::new(4287137928);
    pub const LIGHT_GRAY: Self = Color::new(4291611852);
    pub const DARK_GREY: Self = Color::new(4282664004);
    pub const GREY: Self = Color::new(4287137928);
    pub const LIGHT_GREY: Self = Color::new(4291611852);
    pub const WHITE: Self = Color::new(4294967295);
    pub const RED: Self = Color::new(4294901760);
    pub const GREEN: Self = Color::new(4278255360);
    pub const BLUE: Self = Color::new(4278190335);
    pub const YELLOW: Self = Color::new(4294967040);
    pub const CYAN: Self = Color::new(4278255615);
    pub const MAGENTA: Self = Color::new(4294902015);

    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_argb(255, r, g, b)
    }

    pub fn with_a(self, a: u8) -> Self {
        let color: SkColor = self.into();
        color.with_a(a).into()
    }

    pub fn a(self) -> u8 {
        (self.0 >> 24) as _
    }

    pub fn r(self) -> u8 {
        (self.0 >> 16) as _
    }

    pub fn g(self) -> u8 {
        (self.0 >> 8) as _
    }

    pub fn b(self) -> u8 {
        self.0 as _
    }

    pub fn to_rgb(self) -> RGB {
        let color: SkColor = self.into();
        color.to_rgb()
    }
}

pub trait DisplayColor {
    fn fmt_rgb(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn fmt_hsl(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl Parse for Color {
    fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "red" => Ok(Self::RED),
            "green" => Ok(Self::GREEN),
            "blue" => Ok(Self::BLUE),
            "yellow" => Ok(Self::YELLOW),
            "black" => Ok(Self::BLACK),
            "gray" | "grey" => Ok(Self::GRAY),
            "white" => Ok(Self::WHITE),
            "orange" => Ok(Self::from_rgb(255, 165, 0)),
            "transparent" | "none" => Ok(Self::TRANSPARENT),
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
        let color: SkColor = (*self).into();
        write!(
            f,
            "rgb({}, {}, {}, {})",
            color.r(),
            color.g(),
            color.b(),
            color.a()
        )
    }

    fn fmt_hsl(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color: SkColor = (*self).into();
        // HSV to HSL conversion
        let hsv = color.to_hsv();
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
            (color.a() as f32 / 128.0) * 100.0
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

    let base_color = SkColor::from_rgb(r, g, b);

    if let Some(a) = a {
        Ok(base_color.with_a(parse_alpha(a)?).into())
    } else {
        Ok(base_color.into())
    }
}

pub fn parse_alpha(value: &str) -> Result<u8, ParseError> {
    let value = value.trim();
    if let Ok(u8_alpha) = value.parse::<u8>() {
        Ok(u8_alpha)
    } else if let Ok(f32_alpha) = value.parse::<f32>() {
        let a = (255.0 * f32_alpha).clamp(0.0, 255.0).round() as u8;
        Ok(a)
    } else {
        Err(ParseError)
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

        Ok(hsv.to_color((a * 255.0).round() as u8).into())
    } else {
        Ok(hsv.to_color(255).into())
    }
}

fn parse_hex_color(color: &str) -> Result<Color, ParseError> {
    match color.len() {
        4 => {
            let r = u8::from_str_radix(&color[1..2].repeat(2), 16).map_err(|_| ParseError)?;
            let g = u8::from_str_radix(&color[2..3].repeat(2), 16).map_err(|_| ParseError)?;
            let b = u8::from_str_radix(&color[3..4].repeat(2), 16).map_err(|_| ParseError)?;
            Ok(Color::from_argb(1, r, g, b))
        }
        5 => {
            let r = u8::from_str_radix(&color[1..2].repeat(2), 16).map_err(|_| ParseError)?;
            let g = u8::from_str_radix(&color[2..3].repeat(2), 16).map_err(|_| ParseError)?;
            let b = u8::from_str_radix(&color[3..4].repeat(2), 16).map_err(|_| ParseError)?;
            let a = u8::from_str_radix(&color[4..5].repeat(2), 16).map_err(|_| ParseError)?;
            Ok(Color::from_argb(a, r, g, b))
        }
        7 => {
            let r = u8::from_str_radix(&color[1..3], 16).map_err(|_| ParseError)?;
            let g = u8::from_str_radix(&color[3..5], 16).map_err(|_| ParseError)?;
            let b = u8::from_str_radix(&color[5..7], 16).map_err(|_| ParseError)?;
            Ok(Color::from_argb(1, r, g, b))
        }
        9 => {
            let r = u8::from_str_radix(&color[1..3], 16).map_err(|_| ParseError)?;
            let g = u8::from_str_radix(&color[3..5], 16).map_err(|_| ParseError)?;
            let b = u8::from_str_radix(&color[5..7], 16).map_err(|_| ParseError)?;
            let a = u8::from_str_radix(&color[7..9], 16).map_err(|_| ParseError)?;
            Ok(Color::from_argb(a, r, g, b))
        }
        _ => Err(ParseError),
    }
}
