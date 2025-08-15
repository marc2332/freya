use freya_engine::prelude::Weight as SkWeight;

use crate::parsing::{
    Parse,
    ParseError,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct FontWeight(i32);

impl FontWeight {
    pub const INVISIBLE: Self = Self(0);
    pub const THIN: Self = Self(100);
    pub const EXTRA_LIGHT: Self = Self(200);
    pub const LIGHT: Self = Self(300);
    pub const NORMAL: Self = Self(400);
    pub const MEDIUM: Self = Self(500);
    pub const SEMI_BOLD: Self = Self(600);
    pub const BOLD: Self = Self(700);
    pub const EXTRA_BOLD: Self = Self(800);
    pub const BLACK: Self = Self(900);
    pub const EXTRA_BLACK: Self = Self(1000);
}

impl From<i32> for FontWeight {
    fn from(weight: i32) -> Self {
        FontWeight(weight)
    }
}

impl From<FontWeight> for SkWeight {
    fn from(value: FontWeight) -> Self {
        value.0.into()
    }
}

impl Parse for FontWeight {
    // NOTES:
    // This is mostly taken from the OpenType specification (https://learn.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass)
    // CSS has one deviation from this spec, which uses the value "950" for extra_black.
    // skia_safe also has an "invisible" weight smaller than the thin weight, which could fall under CSS's interpretation of OpenType's
    // version. In this case it would be font_weight: "50".
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "invisible" => FontWeight::INVISIBLE,
            "thin" => FontWeight::THIN,
            "extra-light" => FontWeight::EXTRA_LIGHT,
            "light" => FontWeight::LIGHT,
            "normal" => FontWeight::NORMAL,
            "medium" => FontWeight::MEDIUM,
            "semi-bold" => FontWeight::SEMI_BOLD,
            "bold" => FontWeight::BOLD,
            "extra-bold" => FontWeight::EXTRA_BOLD,
            "black" => FontWeight::BLACK,
            "extra-black" => FontWeight::EXTRA_BLACK,
            "50" => FontWeight::INVISIBLE,
            "100" => FontWeight::THIN,
            "200" => FontWeight::EXTRA_LIGHT,
            "300" => FontWeight::LIGHT,
            "400" => FontWeight::NORMAL,
            "500" => FontWeight::MEDIUM,
            "600" => FontWeight::SEMI_BOLD,
            "700" => FontWeight::BOLD,
            "800" => FontWeight::EXTRA_BOLD,
            "900" => FontWeight::BLACK,
            "950" => FontWeight::EXTRA_BLACK,
            _ => FontWeight::NORMAL,
        })
    }
}
