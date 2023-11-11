use crate::prelude::{Area, Point2D, Size2D};

#[derive(Default, PartialEq, Debug, Clone)]
pub enum Position {
    #[default]
    Stacked,

    Absolute {
        top: Option<f32>,
        right: Option<f32>,
        bottom: Option<f32>,
        left: Option<f32>,
    },
}

impl Position {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Absolute {
                top,
                right,
                bottom,
                left,
            } => top.is_some() && right.is_some() && bottom.is_some() && left.is_some(),
            Self::Stacked => true,
        }
    }

    pub fn new_absolute() -> Self {
        Self::Absolute {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }

    pub fn is_absolute(&self) -> bool {
        matches!(self, Self::Absolute { .. })
    }

    pub fn set_top(&mut self, value: f32) {
        if !self.is_absolute() {
            *self = Self::new_absolute();
        }
        if let Self::Absolute { top, .. } = self {
            *top = Some(value)
        }
    }

    pub fn set_right(&mut self, value: f32) {
        if !self.is_absolute() {
            *self = Self::new_absolute();
        }
        if let Self::Absolute { right, .. } = self {
            *right = Some(value)
        }
    }

    pub fn set_bottom(&mut self, value: f32) {
        if !self.is_absolute() {
            *self = Self::new_absolute();
        }
        if let Self::Absolute { bottom, .. } = self {
            *bottom = Some(value)
        }
    }

    pub fn set_left(&mut self, value: f32) {
        if !self.is_absolute() {
            *self = Self::new_absolute();
        }
        if let Self::Absolute { left, .. } = self {
            *left = Some(value)
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
            Position::Absolute {
                top,
                right,
                bottom,
                left,
            } => {
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
