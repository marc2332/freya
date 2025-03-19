use crate::{
    prelude::{
        Area,
        Point2D,
        Size2D,
    },
    scaled::Scaled,
};

#[derive(Default, PartialEq, Clone, Debug)]
pub struct PositionSides {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Position {
    Stacked(Box<PositionSides>),

    Absolute(Box<PositionSides>),
    Global(Box<PositionSides>),
}

impl Default for Position {
    fn default() -> Self {
        Self::new_stacked()
    }
}

impl Position {
    pub fn swap_for(&mut self, mut other: Self) {
        let old_positions = match self {
            Self::Global(positions) | Self::Absolute(positions) | Self::Stacked(positions) => {
                positions.clone()
            }
        };

        match &mut other {
            Self::Absolute(_) => {
                *self = Self::new_absolute();
            }
            Self::Global(_) => {
                *self = Self::new_global();
            }
            Self::Stacked(_) => {
                *self = Self::new_stacked();
            }
        };

        match self {
            Self::Absolute(positions) | Self::Global(positions) | Self::Stacked(positions) => {
                *positions = old_positions;
            }
        };
    }

    pub fn new_absolute() -> Self {
        Self::Absolute(Box::new(PositionSides {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }))
    }

    pub fn new_global() -> Self {
        Self::Global(Box::new(PositionSides {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }))
    }

    pub fn new_stacked() -> Self {
        Self::Stacked(Box::new(PositionSides {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }))
    }

    pub fn is_stacked(&self) -> bool {
        matches!(self, Self::Stacked { .. })
    }

    pub fn is_absolute(&self) -> bool {
        matches!(self, Self::Absolute { .. })
    }

    pub fn is_global(&self) -> bool {
        matches!(self, Self::Global { .. })
    }

    pub fn set_top(&mut self, value: f32) {
        match self {
            Self::Absolute(position) | Self::Global(position) | Self::Stacked(position) => {
                position.top = Some(value);
            }
        }
    }

    pub fn set_right(&mut self, value: f32) {
        match self {
            Self::Absolute(position) | Self::Global(position) | Self::Stacked(position) => {
                position.right = Some(value);
            }
        }
    }

    pub fn set_bottom(&mut self, value: f32) {
        match self {
            Self::Absolute(position) | Self::Global(position) | Self::Stacked(position) => {
                position.bottom = Some(value);
            }
        }
    }

    pub fn set_left(&mut self, value: f32) {
        match self {
            Self::Absolute(position) | Self::Global(position) | Self::Stacked(position) => {
                position.left = Some(value);
            }
        }
    }

    pub fn get_origin(
        &self,
        available_parent_area: &Area,
        parent_area: &Area,
        area_size: &Size2D,
        root_area: &Area,
    ) -> Point2D {
        match self {
            Self::Stacked(_) => available_parent_area.origin,
            Self::Absolute(absolute_position) => {
                let PositionSides {
                    top,
                    right,
                    bottom,
                    left,
                } = &**absolute_position;
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
            Self::Global(global_position) => {
                let PositionSides {
                    top,
                    right,
                    bottom,
                    left,
                } = &**global_position;
                let y = {
                    let mut y = 0.;
                    if let Some(top) = top {
                        y = *top;
                    } else if let Some(bottom) = bottom {
                        y = root_area.max_y() - bottom;
                    }
                    y
                };
                let x = {
                    let mut x = 0.;
                    if let Some(left) = left {
                        x = *left;
                    } else if let Some(right) = right {
                        x = root_area.max_x() - right;
                    }
                    x
                };
                Point2D::new(x, y)
            }
        }
    }
}

impl Scaled for Position {
    fn scale(&mut self, scale_factor: f32) {
        if let Self::Absolute(absolute_postion) = self {
            if let Some(top) = &mut absolute_postion.top {
                *top *= scale_factor;
            }
            if let Some(right) = &mut absolute_postion.right {
                *right *= scale_factor;
            }
            if let Some(bottom) = &mut absolute_postion.bottom {
                *bottom *= scale_factor;
            }
            if let Some(left) = &mut absolute_postion.left {
                *left *= scale_factor;
            }
        }
    }
}

impl Position {
    pub fn pretty(&self) -> String {
        match self {
            Self::Stacked(_) => "stacked".to_string(),
            Self::Absolute(positions) | Self::Global(positions) => format!(
                "{}, {}, {}, {}",
                positions.top.unwrap_or_default(),
                positions.right.unwrap_or_default(),
                positions.bottom.unwrap_or_default(),
                positions.left.unwrap_or_default()
            ),
        }
    }
}
