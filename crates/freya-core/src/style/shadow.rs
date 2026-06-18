use std::fmt::Display;

use torin::scaled::Scaled;

use crate::prelude::Color;

/// Whether a [`Shadow`] is cast outside the element or inside it.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq)]
pub enum ShadowPosition {
    /// Drop shadow cast outside the element. This is the default.
    #[default]
    Normal,
    /// Shadow cast inside the element's bounds.
    Inset,
}

/// A shadow cast by an element.
///
/// Build it from a `(x, y, blur, spread, color)` tuple, or start from [`Shadow::new`]
/// and chain the methods you need:
///
/// ```
/// # use freya::prelude::*;
/// let shadow = Shadow::new()
///     .x(0.0)
///     .y(4.0)
///     .blur(8.0)
///     .spread(0.0)
///     .color(Color::BLACK)
///     .inset();
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Shadow {
    pub(crate) position: ShadowPosition,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) blur: f32,
    pub(crate) spread: f32,
    pub color: Color,
}

impl Display for Shadow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{:?} {} {} {} {}",
            self.position, self.x, self.y, self.blur, self.spread
        ))
    }
}

impl<C: Into<Color>> From<(f32, f32, f32, f32, C)> for Shadow {
    fn from((x, y, blur, spread, color): (f32, f32, f32, f32, C)) -> Self {
        Self {
            position: ShadowPosition::default(),
            x,
            y,
            blur,
            spread,
            color: color.into(),
        }
    }
}

impl Shadow {
    /// Create a new [`Shadow`] with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Shorthand for [Self::position].
    pub fn inset(mut self) -> Self {
        self.position = ShadowPosition::Inset;
        self
    }

    /// Shorthand for [Self::position].
    pub fn normal(mut self) -> Self {
        self.position = ShadowPosition::Normal;
        self
    }

    /// Set whether the shadow is cast outside or inside the element. See [`ShadowPosition`].
    pub fn position(mut self, position: ShadowPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the horizontal offset of the shadow, in pixels.
    pub fn x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    /// Set the vertical offset of the shadow, in pixels.
    pub fn y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    /// Set the blur radius of the shadow, in pixels.
    pub fn blur(mut self, blur: f32) -> Self {
        self.blur = blur;
        self
    }

    /// Set how much the shadow expands beyond the element's bounds, in pixels.
    pub fn spread(mut self, spread: f32) -> Self {
        self.spread = spread;
        self
    }

    /// Set the [`Color`] of the shadow.
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }
}

impl Scaled for Shadow {
    fn scale(&mut self, scale_factor: f32) {
        self.x *= scale_factor;
        self.y *= scale_factor;
        self.spread *= scale_factor;
        self.blur *= scale_factor;
    }
}
