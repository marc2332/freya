#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug, Default, Copy)]
pub enum Direction {
    /// Stack children vertically. This is the default.
    #[default]
    Vertical,
    Horizontal,
}

impl Direction {
    /// Use a [`Vertical`](Direction::Vertical) direction.
    pub fn vertical() -> Direction {
        Direction::Vertical
    }

    /// Use a [`Horizontal`](Direction::Horizontal) direction.
    pub fn horizontal() -> Direction {
        Direction::Horizontal
    }

    pub fn pretty(&self) -> String {
        match self {
            Self::Horizontal => "horizontal".to_string(),
            Self::Vertical => "vertical".to_string(),
        }
    }
}
