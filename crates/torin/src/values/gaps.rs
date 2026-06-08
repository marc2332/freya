pub use euclid::Rect;

use crate::{
    geometry::Length,
    scaled::Scaled,
};

/// Spacing applied around the four sides of an element, used for `padding` and `margin`.
///
/// Prefer the constructors [`Gaps::new_all`], [`Gaps::new_symmetric`] and [`Gaps::new`].
/// It also implements `From<f32>`, `From<(f32, f32)>` and `From<(f32, f32, f32, f32)>`.
///
/// ```
/// # use torin::prelude::*;
/// let all = Gaps::new_all(10.0);
/// let symmetric = Gaps::new_symmetric(5.0, 20.0); // (vertical, horizontal)
/// let each = Gaps::new(1.0, 2.0, 3.0, 4.0); // (top, right, bottom, left)
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Clone, Debug, Default, Copy)]
pub struct Gaps {
    top: Length,
    right: Length,
    bottom: Length,
    left: Length,
}

impl From<f32> for Gaps {
    fn from(padding: f32) -> Self {
        Gaps::new_all(padding)
    }
}

impl From<(f32, f32)> for Gaps {
    fn from((vertical, horizontal): (f32, f32)) -> Self {
        Gaps::new_symmetric(vertical, horizontal)
    }
}

impl From<(f32, f32, f32, f32)> for Gaps {
    fn from((top, right, bottom, left): (f32, f32, f32, f32)) -> Self {
        Gaps::new(top, right, bottom, left)
    }
}

impl Gaps {
    /// Create [`Gaps`] with an individual value for each side, in `(top, right, bottom, left)` order.
    pub const fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: Length::new(top),
            right: Length::new(right),
            bottom: Length::new(bottom),
            left: Length::new(left),
        }
    }

    /// Create [`Gaps`] with the same value on all four sides.
    pub const fn new_all(gaps: f32) -> Self {
        Self::new(gaps, gaps, gaps, gaps)
    }

    /// Create [`Gaps`] with one value for the top and bottom, and another for the left and right.
    pub const fn new_symmetric(vertical: f32, horizontal: f32) -> Self {
        Self::new(vertical, horizontal, vertical, horizontal)
    }

    pub fn fill_vertical(&mut self, value: f32) {
        self.top = Length::new(value);
        self.bottom = Length::new(value);
    }

    pub fn fill_horizontal(&mut self, value: f32) {
        self.right = Length::new(value);
        self.left = Length::new(value);
    }

    pub fn fill_all(&mut self, value: f32) {
        self.fill_horizontal(value);
        self.fill_vertical(value);
    }

    pub fn horizontal(&self) -> f32 {
        (self.right + self.left).get()
    }

    pub fn vertical(&self) -> f32 {
        (self.top + self.bottom).get()
    }

    pub fn top(&self) -> f32 {
        self.top.get()
    }

    pub fn right(&self) -> f32 {
        self.right.get()
    }

    pub fn bottom(&self) -> f32 {
        self.bottom.get()
    }

    pub fn left(&self) -> f32 {
        self.left.get()
    }

    pub fn pretty(&self) -> String {
        format!(
            "({}, {}, {}, {})",
            self.top(),
            self.right(),
            self.bottom(),
            self.left()
        )
    }
}

impl Scaled for Gaps {
    fn scale(&mut self, scale: f32) {
        self.left *= scale;
        self.right *= scale;
        self.top *= scale;
        self.bottom *= scale;
    }
}
