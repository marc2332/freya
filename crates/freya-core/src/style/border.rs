use std::fmt;

use freya_engine::prelude::{
    SkPath,
    SkRRect,
};
use torin::scaled::Scaled;

use crate::prelude::Color;

/// Width of each side of a [`Border`], in pixels.
///
/// Implements `From<f32>`, applied to all sides.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct BorderWidth {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
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
        BorderWidth {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
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

/// Whether a [`Border`] is drawn as a continuous outline or a dashed one.
///
/// A dashed border is painted by **stroking** the outline's centreline rather than filling the
/// region between its outer and inner edges, which is what makes the pattern possible at all.
/// Two consequences, both deliberate: it uses one width for all four sides
/// ([`BorderWidth::top`]), since a stroke has a single width, and it ignores
/// [`CornerRadius::smoothing`](crate::style::CornerRadius): a squircle outline has no stroked
/// equivalent.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderStyle {
    /// One continuous outline. This is the default.
    #[default]
    Solid,
    /// A repeating pattern of `dash` pixels drawn and `gap` pixels skipped.
    Dashed { dash: f32, gap: f32 },
}

impl Scaled for BorderStyle {
    fn scale(&mut self, scale_factor: f32) {
        if let Self::Dashed { dash, gap } = self {
            *dash *= scale_factor;
            *gap *= scale_factor;
        }
    }
}

/// An outline drawn around an element, with a [`fill`](Border::fill) color,
/// a [`width`](Border::width) per side, an [`alignment`](Border::alignment) and a
/// [`style`](Border::style).
///
/// Start from [`Border::new`] and chain the methods you need:
///
/// ```
/// # use freya::prelude::*;
/// let border = Border::new()
///     .fill(Color::RED)
///     .width(2.0)
///     .alignment(BorderAlignment::Inner);
/// ```
///
/// A dashed outline, for a slot that is waiting to be filled rather than a bound edge:
///
/// ```
/// # use freya::prelude::*;
/// let border = Border::new().fill(Color::RED).width(1.0).dashed(4.0, 3.0);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Border {
    pub fill: Color,
    pub width: BorderWidth,
    pub alignment: BorderAlignment,
    pub style: BorderStyle,
}

impl Border {
    /// Create a new [`Border`] with default values.
    pub fn new() -> Self {
        Self::default()
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

    /// Set the [`BorderStyle`] the outline is drawn in. See [`BorderStyle`].
    pub fn style(mut self, style: impl Into<BorderStyle>) -> Self {
        self.style = style.into();
        self
    }

    /// Draw the outline dashed: `dash` pixels painted, `gap` pixels skipped, repeating. See
    /// [`BorderStyle::Dashed`] for what a stroked outline costs.
    pub fn dashed(self, dash: f32, gap: f32) -> Self {
        self.style(BorderStyle::Dashed { dash, gap })
    }

    #[inline]
    pub(crate) fn is_visible(&self) -> bool {
        !(self.width.top == 0.0
            && self.width.left == 0.0
            && self.width.bottom == 0.0
            && self.width.right == 0.0)
            && self.fill != Color::TRANSPARENT
    }

    pub fn pretty(&self) -> String {
        format!("{} {:?} {:?}", self.width, self.alignment, self.style)
    }
}

impl Scaled for Border {
    fn scale(&mut self, scale_factor: f32) {
        self.width.scale(scale_factor);
        self.style.scale(scale_factor);
    }
}
