#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Scale {
    fn from((x, y): (f32, f32)) -> Self {
        Scale { x, y }
    }
}

impl From<f32> for Scale {
    fn from(x_y: f32) -> Self {
        Scale { x: x_y, y: x_y }
    }
}
