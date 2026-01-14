use freya_engine::prelude::Width as SkWidth;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FontWidth(i32);

impl Default for FontWidth {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl FontWidth {
    pub const ULTRA_CONDENSED: Self = Self(1);
    pub const EXTRA_CONDENSED: Self = Self(2);
    pub const CONDENSED: Self = Self(3);
    pub const SEMI_CONDENSED: Self = Self(4);
    pub const NORMAL: Self = Self(5);
    pub const SEMI_EXPANDED: Self = Self(6);
    pub const EXPANDED: Self = Self(7);
    pub const EXTRA_EXPANDED: Self = Self(8);
    pub const ULTRA_EXPANDED: Self = Self(9);

    pub fn get(self) -> i32 {
        self.0
    }
}

impl From<i32> for FontWidth {
    fn from(width: i32) -> Self {
        FontWidth(width)
    }
}

impl From<FontWidth> for i32 {
    fn from(width: FontWidth) -> i32 {
        width.0
    }
}

impl From<FontWidth> for f32 {
    fn from(width: FontWidth) -> f32 {
        width.0 as f32
    }
}

impl From<FontWidth> for SkWidth {
    fn from(value: FontWidth) -> Self {
        value.0.into()
    }
}
