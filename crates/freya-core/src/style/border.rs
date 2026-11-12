use std::fmt;

use freya_engine::prelude::{
    SkPath,
    SkRRect,
};
use torin::scaled::Scaled;

use crate::prelude::Color;

/// Width of each side of a [`Border`], in pixels.
///
/// Use [`BorderWidth::new_all`] for uniform widths or [`BorderWidth::new_symmetric`]
/// for vertical and horizontal widths, then override individual sides with builder methods.
/// Also accepts a scalar or tuples in `(vertical, horizontal)` or `(top, right, bottom, left)` order.
///
/// ```
/// # use freya::prelude::*;
/// let width = BorderWidth::new_symmetric(1.0, 2.0).bottom(3.0);
/// let divider = BorderWidth::default().bottom(1.0);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct BorderWidth {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl BorderWidth {
    /// Create widths in `(top, right, bottom, left)` order.
    pub const fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Create the same width on all four sides.
    pub const fn new_all(width: f32) -> Self {
        Self::new(width, width, width, width)
    }

    /// Create widths for `(vertical, horizontal)` pairs of sides.
    pub const fn new_symmetric(vertical: f32, horizontal: f32) -> Self {
        Self::new(vertical, horizontal, vertical, horizontal)
    }

    /// Set the top width in pixels, keeping the other sides unchanged.
    pub const fn top(mut self, width: f32) -> Self {
        self.top = width;
        self
    }

    /// Set the right width in pixels, keeping the other sides unchanged.
    pub const fn right(mut self, width: f32) -> Self {
        self.right = width;
        self
    }

    /// Set the bottom width in pixels, keeping the other sides unchanged.
    pub const fn bottom(mut self, width: f32) -> Self {
        self.bottom = width;
        self
    }

    /// Set the left width in pixels, keeping the other sides unchanged.
    pub const fn left(mut self, width: f32) -> Self {
        self.left = width;
        self
    }
}

impl Scaled for BorderWidth {
    fn scale(&mut self, scale_factor: f32) {
        self.top *= scale_factor;
        self.left *= scale_factor;
        self.bottom *= scale_factor;
        self.right *= scale_factor;
    }
}

impl From<f32> for BorderWidth {
    fn from(value: f32) -> Self {
        Self::new_all(value)
    }
}

impl From<(f32, f32)> for BorderWidth {
    fn from((vertical, horizontal): (f32, f32)) -> Self {
        Self::new_symmetric(vertical, horizontal)
    }
}

impl From<(f32, f32, f32, f32)> for BorderWidth {
    fn from((top, right, bottom, left): (f32, f32, f32, f32)) -> Self {
        Self::new(top, right, bottom, left)
    }
}

impl fmt::Display for BorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.top, self.right, self.bottom, self.left,
        )
    }
}

/// Where a [`Border`] is drawn relative to the element's edge.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderAlignment {
    /// Draw the border inside the element's bounds. This is the default.
    #[default]
    Inner,
    /// Draw the border outside the element's bounds.
    Outer,
    /// Draw the border centered on the element's edge, half inside and half outside.
    Center,
}

pub enum BorderShape {
    DRRect(SkRRect, SkRRect),
    Path(SkPath),
}

/// An outline drawn around an element, with a [`fill`](Border::fill) color,
/// a [`width`](Border::width) per side and an [`alignment`](Border::alignment).
///
/// Start from [`Border::new`] or a single-side shortcut such as [`Border::bottom`],
/// then chain the methods you need:
///
/// ```
/// # use freya::prelude::*;
/// let border = Border::new()
///     .fill(Color::RED)
///     .width(2.0)
///     .alignment(BorderAlignment::Inner);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Border {
    pub fill: Color,
    pub width: BorderWidth,
    pub alignment: BorderAlignment,
}

impl Border {
    /// Create a new [`Border`] with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a border on only the top edge, with inner alignment.
    pub fn top(color: impl Into<Color>, width: f32) -> Self {
        Self::new()
            .fill(color)
            .width(BorderWidth::default().top(width))
    }

    /// Create a border on only the right edge, with inner alignment.
    pub fn right(color: impl Into<Color>, width: f32) -> Self {
        Self::new()
            .fill(color)
            .width(BorderWidth::default().right(width))
    }

    /// Create a border on only the bottom edge, with inner alignment.
    pub fn bottom(color: impl Into<Color>, width: f32) -> Self {
        Self::new()
            .fill(color)
            .width(BorderWidth::default().bottom(width))
    }

    /// Create a border on only the left edge, with inner alignment.
    pub fn left(color: impl Into<Color>, width: f32) -> Self {
        Self::new()
            .fill(color)
            .width(BorderWidth::default().left(width))
    }

    /// Set the [`Color`] the border is painted with.
    pub fn fill(mut self, color: impl Into<Color>) -> Self {
        self.fill = color.into();
        self
    }

    /// Set the [`BorderWidth`] of the border, in pixels.
    pub fn width(mut self, width: impl Into<BorderWidth>) -> Self {
        self.width = width.into();
        self
    }

    /// Set how the border is aligned to the element's edge. See [`BorderAlignment`].
    pub fn alignment(mut self, alignment: impl Into<BorderAlignment>) -> Self {
        self.alignment = alignment.into();
        self
    }

    #[inline]
    pub fn is_visible(&self) -> bool {
        !(self.width.top == 0.0
            && self.width.left == 0.0
            && self.width.bottom == 0.0
            && self.width.right == 0.0)
            && self.fill != Color::TRANSPARENT
    }

    pub fn pretty(&self) -> String {
        format!("{} {:?}", self.width, self.alignment)
    }
}

impl Scaled for Border {
    fn scale(&mut self, scale_factor: f32) {
        self.width.scale(scale_factor);
    }
}
