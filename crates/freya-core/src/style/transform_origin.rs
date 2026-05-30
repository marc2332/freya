use torin::prelude::{
    Area,
    Point2D,
};

/// Position of a [`TransformOrigin`] along a single axis.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OriginValue {
    /// Fraction of the element's length along the axis, where `0.0` is the start
    /// and `1.0` is the end.
    Fraction(f32),
    /// Absolute pixels measured from the element's top-left corner.
    Pixels(f32),
}

impl OriginValue {
    /// Resolve this value into an absolute offset given the element's length on the axis.
    fn resolve(self, length: f32) -> f32 {
        match self {
            OriginValue::Fraction(fraction) => length * fraction,
            OriginValue::Pixels(pixels) => pixels,
        }
    }
}

/// Reference point that the scale and rotation effects of an element pivot around.
///
/// Defaults to the element's center, matching the CSS `transform-origin: 50% 50%`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformOrigin {
    pub x: OriginValue,
    pub y: OriginValue,
}

impl Default for TransformOrigin {
    fn default() -> Self {
        Self::center()
    }
}

impl TransformOrigin {
    /// Resolve the origin point in absolute coordinates for the given element area.
    pub fn origin(&self, area: &Area) -> Point2D {
        Point2D::new(
            area.min_x() + self.x.resolve(area.width()),
            area.min_y() + self.y.resolve(area.height()),
        )
    }

    pub fn center() -> Self {
        Self {
            x: OriginValue::Fraction(0.5),
            y: OriginValue::Fraction(0.5),
        }
    }

    pub fn top_left() -> Self {
        Self {
            x: OriginValue::Fraction(0.0),
            y: OriginValue::Fraction(0.0),
        }
    }

    pub fn top() -> Self {
        Self {
            x: OriginValue::Fraction(0.5),
            y: OriginValue::Fraction(0.0),
        }
    }

    pub fn top_right() -> Self {
        Self {
            x: OriginValue::Fraction(1.0),
            y: OriginValue::Fraction(0.0),
        }
    }

    pub fn left() -> Self {
        Self {
            x: OriginValue::Fraction(0.0),
            y: OriginValue::Fraction(0.5),
        }
    }

    pub fn right() -> Self {
        Self {
            x: OriginValue::Fraction(1.0),
            y: OriginValue::Fraction(0.5),
        }
    }

    pub fn bottom_left() -> Self {
        Self {
            x: OriginValue::Fraction(0.0),
            y: OriginValue::Fraction(1.0),
        }
    }

    pub fn bottom() -> Self {
        Self {
            x: OriginValue::Fraction(0.5),
            y: OriginValue::Fraction(1.0),
        }
    }

    pub fn bottom_right() -> Self {
        Self {
            x: OriginValue::Fraction(1.0),
            y: OriginValue::Fraction(1.0),
        }
    }
}

/// Build a fractional [`TransformOrigin`], where `0.0` is the start and `1.0` the end of each axis.
impl From<(f32, f32)> for TransformOrigin {
    fn from((x, y): (f32, f32)) -> Self {
        Self {
            x: OriginValue::Fraction(x),
            y: OriginValue::Fraction(y),
        }
    }
}
