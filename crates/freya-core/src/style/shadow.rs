use std::fmt::Display;

use torin::scaled::Scaled;

use crate::prelude::Color;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq)]
pub enum ShadowPosition {
    #[default]
    Normal,
    Inset,
}

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

    pub fn position(mut self, position: ShadowPosition) -> Self {
        self.position = position;
        self
    }

    pub fn x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    pub fn blur(mut self, blur: f32) -> Self {
        self.blur = blur;
        self
    }

    pub fn spread(mut self, spread: f32) -> Self {
        self.spread = spread;
        self
    }

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
