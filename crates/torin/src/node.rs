pub use euclid::Rect;

use crate::{
    alignment::Alignment,
    direction::Direction,
    gaps::Gaps,
    geometry::Length,
    prelude::{
        Content,
        Phase,
        Position,
        VisibleSize,
    },
    scaled::Scaled,
    size::Size,
    wrap_content::WrapContent,
};

/// Node layout configuration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    // Visible dimensions
    pub visible_width: VisibleSize,
    pub visible_height: VisibleSize,

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
    pub direction: Direction,

    /// Position config
    pub position: Position,

    pub content: Content,

    /// Whether children wrap into a new line when there is no more space
    pub wrap_content: WrapContent,

    /// A Node might depend on inner sizes but have a fixed position, like scroll views.
    pub has_layout_references: bool,

    pub contains_text: bool,

    pub spacing: Length,
}

impl Scaled for Node {
    fn scale(&mut self, scale_factor: f32) {
        self.width.scale(scale_factor);
        self.height.scale(scale_factor);
        self.minimum_width.scale(scale_factor);
        self.minimum_height.scale(scale_factor);
        self.maximum_width.scale(scale_factor);
        self.maximum_height.scale(scale_factor);
        self.margin.scale(scale_factor);
        self.padding.scale(scale_factor);
        self.offset_x *= scale_factor;
        self.offset_y *= scale_factor;
        self.position.scale(scale_factor);
        self.spacing *= scale_factor;
    }
}

impl Node {
    /// Create a Node with the default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a new Node given a size and a direction
    pub fn from_size_and_direction(width: Size, height: Size, direction: Direction) -> Self {
        Self {
            width,
            height,
            direction,
            ..Default::default()
        }
    }

    /// Construct a new Node given some sizes
    pub fn from_sizes(
        width: Size,
        height: Size,
        minimum_width: Size,
        minimum_height: Size,
        maximum_width: Size,
        maximum_height: Size,
    ) -> Self {
        Self {
            width,
            height,
            minimum_width,
            minimum_height,
            maximum_width,
            maximum_height,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and a visible size
    pub fn from_size_and_visible_size(
        width: Size,
        height: Size,
        visible_width: VisibleSize,
        visible_height: VisibleSize,
    ) -> Self {
        Self {
            width,
            height,
            visible_width,
            visible_height,
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
        direction: Direction,
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

    /// Construct a new Node given a size, alignments, direction and spacing
    pub fn from_size_and_alignments_and_direction_and_spacing(
        width: Size,
        height: Size,
        main_alignment: Alignment,
        cross_alignment: Alignment,
        direction: Direction,
        spacing: Length,
    ) -> Self {
        Self {
            width,
            height,
            main_alignment,
            cross_alignment,
            direction,
            spacing,
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

    /// Construct a new Node given a size and a direction and some margin,
    pub fn from_size_and_direction_and_margin(
        width: Size,
        height: Size,
        direction: Direction,
        margin: Gaps,
    ) -> Self {
        Self {
            width,
            height,
            margin,
            direction,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size, alignments and a direction
    pub fn from_size_and_alignments_and_direction_and_padding(
        width: Size,
        height: Size,
        main_alignment: Alignment,
        cross_alignment: Alignment,
        direction: Direction,
        padding: Gaps,
    ) -> Self {
        Self {
            width,
            height,
            main_alignment,
            cross_alignment,
            padding,
            direction,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and a position
    pub fn from_size_and_position(width: Size, height: Size, position: Position) -> Self {
        Self {
            width,
            height,
            position,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and content
    pub fn from_size_and_content(width: Size, height: Size, content: Content) -> Self {
        Self {
            width,
            height,
            content,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and spacing
    pub fn from_size_and_direction_and_spacing(
        width: Size,
        height: Size,
        direction: Direction,
        spacing: Length,
    ) -> Self {
        Self {
            width,
            height,
            direction,
            spacing,
            ..Default::default()
        }
    }

    /// Has properties that depend on the inner Nodes?
    pub fn does_depend_on_inner(&self) -> bool {
        self.width.inner_sized(Phase::Initial)
            || self.height.inner_sized(Phase::Initial)
            || self.contains_text
            || self.do_inner_depend_on_parent()
    }

    /// Has properties that make its children dependant on it?
    pub fn do_inner_depend_on_parent(&self) -> bool {
        self.cross_alignment.is_not_start()
            || self.main_alignment.is_not_start()
            || self.has_layout_references
    }
}
