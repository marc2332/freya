use std::ops::Deref;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Hash, Debug, PartialEq, Clone, Copy)]
pub struct FontSize(i32);

impl Default for FontSize {
    fn default() -> Self {
        FontSize(16)
    }
}

impl From<i32> for FontSize {
    fn from(value: i32) -> Self {
        FontSize(value)
    }
}

impl From<f32> for FontSize {
    fn from(value: f32) -> Self {
        FontSize(value as i32)
    }
}

impl From<FontSize> for f32 {
    fn from(value: FontSize) -> Self {
        value.0 as f32
    }
}

impl Deref for FontSize {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
