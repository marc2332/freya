use std::{
    ops::Deref,
    rc::Rc,
};

/// Converts a nominal font size into a platform-adjusted logical size.
pub type FontSizeConverter = Rc<dyn Fn(f32) -> f32>;

#[derive(Clone, Copy)]
pub(crate) struct TextScale<'a> {
    pub scale_factor: f64,
    pub converter: Option<&'a FontSizeConverter>,
}

impl TextScale<'_> {
    pub fn physical_font_size(self, size: FontSize) -> f32 {
        let nominal_size = f32::from(size);
        let logical_size = self
            .converter
            .map_or(nominal_size, |convert| convert(nominal_size));
        logical_size * self.scale_factor as f32
    }
}

/// Defaults to `16`.
///
/// Implements `From<i32>` and `From<f32>`.
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
