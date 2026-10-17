/// Fades an element's content out towards two opposite edges: everything inside it, its
/// children included, goes from transparent at the edge to opaque `start` (top or left) and
/// `end` (bottom or right) logical pixels in.
///
/// Useful for a scroll list whose items should melt into the background instead of being
/// cut, or for text scrolling sideways.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fade {
    /// How long the fade at the top (or the left) is, in logical pixels.
    pub start: f32,
    /// How long the fade at the bottom (or the right) is, in logical pixels.
    pub end: f32,
    /// Whether it fades towards the left and right edges instead of the top and bottom.
    pub horizontal: bool,
}

impl Fade {
    /// Fades towards the top and bottom edges.
    pub fn vertical(start: f32, end: f32) -> Self {
        Self {
            start,
            end,
            horizontal: false,
        }
    }

    /// Fades towards the left and right edges.
    pub fn horizontal(start: f32, end: f32) -> Self {
        Self {
            start,
            end,
            horizontal: true,
        }
    }
}
