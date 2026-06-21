#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug, Default, Copy)]
pub enum Direction {
    /// Stack children vertically. This is the default.
    #[default]
    Vertical,
    Horizontal,
    /// Stack children vertically, in reverse order.
    VerticalReverse,
    /// Stack children horizontally, in reverse order.
    HorizontalReverse,
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

    /// Use a [`VerticalReverse`](Direction::VerticalReverse) direction.
    pub fn vertical_reverse() -> Direction {
        Direction::VerticalReverse
    }

    /// Use a [`HorizontalReverse`](Direction::HorizontalReverse) direction.
    pub fn horizontal_reverse() -> Direction {
        Direction::HorizontalReverse
    }

    /// Whether children are stacked in reverse order.
    pub fn is_reverse(&self) -> bool {
        matches!(self, Self::VerticalReverse | Self::HorizontalReverse)
    }

    pub fn pretty(&self) -> String {
        match self {
            Self::Horizontal => "horizontal".to_string(),
            Self::Vertical => "vertical".to_string(),
            Self::HorizontalReverse => "horizontal-reverse".to_string(),
            Self::VerticalReverse => "vertical-reverse".to_string(),
        }
    }
}
