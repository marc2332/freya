use std::ops::Deref;

use crate::prelude::{Area, Point2D, Size2D};

#[derive(Default, PartialEq, Clone, Debug)]
pub struct AbsolutePosition {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub enum Position {
    #[default]
    Stacked,

    Absolute(Box<AbsolutePosition>),
}

impl Position {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Absolute(absolute_position) => {
                let AbsolutePosition {
                    top,
                    right,
                    bottom,
                    left,
                } = absolute_position.deref();
                top.is_some() && right.is_some() && bottom.is_some() && left.is_some()
            }
            Self::Stacked => true,
        }
    }

    pub fn new_absolute() -> Self {
        Self::Absolute(Box::new(AbsolutePosition {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }))
    }

    pub fn is_absolute(&self) -> bool {
        matches!(self, Self::Absolute { .. })
    }

    pub fn set_top(&mut self, value: f32) {
        if !self.is_absolute() {
            *self = Self::new_absolute();
        }
        if let Self::Absolute(absolute_position) = self {
            absolute_position.top = Some(value)
        }
    }

    pub fn set_right(&mut self, value: f32) {
        if !self.is_absolute() {
            *self = Self::new_absolute();
        }
        if let Self::Absolute(absolute_position) = self {
            absolute_position.right = Some(value)
        }
    }

    pub fn set_bottom(&mut self, value: f32) {
        if !self.is_absolute() {
            *self = Self::new_absolute();
        }
        if let Self::Absolute(absolute_position) = self {
            absolute_position.bottom = Some(value)
        }
    }

    pub fn set_left(&mut self, value: f32) {
        if !self.is_absolute() {
            *self = Self::new_absolute();
        }
        if let Self::Absolute(absolute_position) = self {
            absolute_position.left = Some(value)
        }
    }

    pub fn get_origin(
        &self,
        available_parent_area: &Area,
        parent_area: &Area,
        area_size: &Size2D,
    ) -> Point2D {
        match self {
            Position::Stacked => available_parent_area.origin,
            Position::Absolute(absolute_position) => {
                let AbsolutePosition {
                    top,
                    right,
                    bottom,
                    left,
                } = absolute_position.deref();
                let y = {
                    let mut y = parent_area.min_y();
                    if let Some(top) = top {
                        y += top;
                    } else if let Some(bottom) = bottom {
                        y = parent_area.max_y() - bottom - area_size.height;
                    }
                    y
                };
                let x = {
                    let mut x = parent_area.min_x();
                    if let Some(left) = left {
                        x += left;
                    } else if let Some(right) = right {
                        x = parent_area.max_x() - right - area_size.width;
                    }
                    x
                };
                Point2D::new(x, y)
            }
        }
    }
}
