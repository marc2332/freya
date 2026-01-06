use std::ops::Mul;

use freya_engine::prelude::{
    SkColor,
    SkColor4f,
    SkRGB,
};

/// Represents one color.
/// You may create [Color]s using
/// - [Color::from_rgb],
/// - [Color::from_argb],
/// - [Color::from_af32rgb],
/// - [Color::new],
/// - Or using tuples like `(255, 0, 150)`, `(133, 133, 133, 0.5)` (alpha as `f32` in 0.0..=1.0), or `(200, 100, 50, 180)` (alpha as `u8` in 0..=255)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct Color(u32);

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        (
            (self.r() as f32 * rhs) as u8,
            (self.g() as f32 * rhs) as u8,
            (self.b() as f32 * rhs) as u8,
            self.a(),
        )
            .into()
    }
}

impl Color {
    pub fn mul_if(self, check: bool, var: f32) -> Self {
        if check { self * var } else { self }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::from_rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, f32)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, f32)) -> Self {
        Color::from_af32rgb(a, r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Color::from_argb(a, r, g, b)
    }
}

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

impl From<Color> for SkColor4f {
    fn from(value: Color) -> Self {
        SkColor4f::new(
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

    pub const fn from_af32rgb(a: f32, r: u8, g: u8, b: u8) -> Self {
        Self::from_argb((255. * a) as u8, r, g, b)
    }

    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_argb(255, r, g, b)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let s = if let Some(stripped) = hex.strip_prefix('#') {
            stripped
        } else if let Some(stripped) = hex.strip_prefix("0x") {
            stripped
        } else {
            hex
        };

        match s.len() {
            6 => {
                // RRGGBB
                u32::from_str_radix(s, 16).ok().map(|rgb| {
                    let r = ((rgb >> 16) & 0xFF) as u8;
                    let g = ((rgb >> 8) & 0xFF) as u8;
                    let b = (rgb & 0xFF) as u8;
                    Color::from_rgb(r, g, b)
                })
            }
            8 => {
                // RRGGBBAA
                u32::from_str_radix(s, 16).ok().map(|rgba| {
                    let r = ((rgba >> 24) & 0xFF) as u8;
                    let g = ((rgba >> 16) & 0xFF) as u8;
                    let b = ((rgba >> 8) & 0xFF) as u8;
                    let a = (rgba & 0xFF) as u8;
                    Color::from_argb(a, r, g, b)
                })
            }
            _ => None,
        }
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

    pub fn to_rgb(self) -> SkRGB {
        let color: SkColor = self.into();
        color.to_rgb()
    }

    pub fn pretty(&self) -> String {
        format!(
            "({:?}, {:?}, {:?}, {:?})",
            self.r(),
            self.g(),
            self.b(),
            self.a() as f32 / 100.
        )
    }
}
