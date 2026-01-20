use freya_engine::prelude::Weight as SkWeight;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct FontWeight(i32);

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::NORMAL
    }
}

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

impl From<FontWeight> for i32 {
    fn from(weight: FontWeight) -> i32 {
        weight.0
    }
}

impl From<FontWeight> for f32 {
    fn from(weight: FontWeight) -> f32 {
        weight.0 as f32
    }
}

impl From<FontWeight> for SkWeight {
    fn from(value: FontWeight) -> Self {
        value.0.into()
    }
}
