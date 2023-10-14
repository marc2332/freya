pub use euclid::Rect;

use crate::{
    alignment::Alignment, direction::DirectionMode, gaps::Gaps, geometry::Length, size::Size,
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

    // Axis alignments for the children
    pub main_alignment: Alignment,
    pub cross_alignment: Alignment,

    /// Inner padding
    pub padding: Gaps,

    /// Inner margin
    pub margin: Gaps,

    /// Inner position offsets
    pub offset_x: Length,
    pub offset_y: Length,

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
        offset_x: Length,
        offset_y: Length,
    ) -> Self {
        Self {
            width,
            height,
            offset_x,
            offset_y,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and padding
    pub fn from_size_and_padding(width: Size, height: Size, padding: Gaps) -> Self {
        Self {
            width,
            height,
            padding,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size, alignments and a direction
    pub fn from_size_and_alignments_and_direction(
        width: Size,
        height: Size,
        main_alignment: Alignment,
        cross_alignment: Alignment,
        direction: DirectionMode,
    ) -> Self {
        Self {
            width,
            height,
            main_alignment,
            cross_alignment,
            direction,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and a direction
    pub fn from_size_and_margin(width: Size, height: Size, margin: Gaps) -> Self {
        Self {
            width,
            height,
            margin,
            ..Default::default()
        }
    }

    /// Has properties that depend on the inner Nodes?
    pub fn does_depend_on_inner(&self) -> bool {
        Size::Inner == self.width || Size::Inner == self.height || self.has_layout_references
        // || self.main_alignment.is_not_start()
        // || self.cross_alignment.is_not_start()
    }
}
