pub use euclid::Rect;

use crate::{
    direction::DirectionMode, display::DisplayMode, geometry::Length, padding::Paddings, size::Size,
};

/// Node layout configuration
#[derive(PartialEq, Clone, Debug, Default)]
pub struct Node {
    /// Dimentions
    pub width: Size,
    pub height: Size,

    // Minimum dimensions
    pub minimum_width: Size,
    pub minimum_height: Size,

    // Maximum dimensions
    pub maximum_width: Size,
    pub maximum_height: Size,

    /// Inner layout mode
    pub display: DisplayMode,

    /// Inner padding
    pub padding: Paddings,

    /// Inner position offsets
    pub scroll_x: Length,
    pub scroll_y: Length,

    /// Direction in which it's inner Nodes will be stacked
    pub direction: DirectionMode,

    /// A Node might depend on inner sizes but have a fixed position, like scroll views.
    pub has_layout_references: bool,
}

impl Node {
    /// Create a Node with the default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a new Node given a size and a direction
    pub fn from_size_and_direction(width: Size, height: Size, direction: DirectionMode) -> Self {
        Self {
            width,
            height,
            direction,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and a scroll
    pub fn from_size_and_scroll(
        width: Size,
        height: Size,
        scroll_x: Length,
        scroll_y: Length,
    ) -> Self {
        Self {
            width,
            height,
            scroll_x,
            scroll_y,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and padding
    pub fn from_size_and_padding(width: Size, height: Size, padding: Paddings) -> Self {
        Self {
            width,
            height,
            padding,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and a display
    pub fn from_size_and_display_and_direction(
        width: Size,
        height: Size,
        display: DisplayMode,
        direction: DirectionMode,
    ) -> Self {
        Self {
            width,
            height,
            display,
            direction,
            ..Default::default()
        }
    }

    /// Has properties that depend on the inner Nodes?
    pub fn does_depend_on_inner(&self) -> bool {
        Size::Inner == self.width || Size::Inner == self.height || self.has_layout_references  || self.display == DisplayMode::Center
    }
}
