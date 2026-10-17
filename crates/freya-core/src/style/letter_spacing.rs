use std::hash::Hash;

/// Space between letters, in pixels. Defaults to `0.0`.
///
/// Implements `From<f32>` and `From<i32>`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LetterSpacing(f32);

impl Default for LetterSpacing {
    fn default() -> Self {
        LetterSpacing(0.0)
    }
}

impl Hash for LetterSpacing {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl From<f32> for LetterSpacing {
    fn from(value: f32) -> Self {
        LetterSpacing(value)
    }
}

impl From<i32> for LetterSpacing {
    fn from(value: i32) -> Self {
        LetterSpacing(value as f32)
    }
}

impl From<LetterSpacing> for f32 {
    fn from(value: LetterSpacing) -> Self {
        value.0
    }
}
