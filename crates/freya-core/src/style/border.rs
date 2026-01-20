use std::fmt;

use freya_engine::prelude::{
    SkPath,
    SkRRect,
};
use torin::scaled::Scaled;

use crate::prelude::Color;

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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderAlignment {
    #[default]
    Inner,
    Outer,
    Center,
}

pub enum BorderShape {
    DRRect(SkRRect, SkRRect),
    Path(SkPath),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Border {
    pub fill: Color,
    pub width: BorderWidth,
    pub alignment: BorderAlignment,
}

impl Border {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill(mut self, color: impl Into<Color>) -> Self {
        self.fill = color.into();
        self
    }

    pub fn width(mut self, width: impl Into<BorderWidth>) -> Self {
        self.width = width.into();
        self
    }

    pub fn alignment(mut self, alignment: impl Into<BorderAlignment>) -> Self {
        self.alignment = alignment.into();
        self
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
        format!("{} {:?}", self.width, self.alignment)
    }
}

impl Scaled for Border {
    fn scale(&mut self, scale_factor: f32) {
        self.width.scale(scale_factor);
    }
}
