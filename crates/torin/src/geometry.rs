use crate::{
    node::Node,
    prelude::{
        Alignment,
        DirectionMode,
        Gaps,
    },
};

#[derive(PartialEq)]
pub struct Measure;

pub type Area = euclid::Rect<f32, Measure>;
pub type Size2D = euclid::Size2D<f32, Measure>;
pub type Point2D = euclid::Point2D<f32, Measure>;
pub type CursorPoint = euclid::Point2D<f64, Measure>;
pub type Length = euclid::Length<f32, Measure>;

pub trait AreaModel {
    /// The area without any outer gap (e.g margin)
    fn without_gaps(self, gap: &Gaps) -> Area;

    /// Adjust the available area with the node offsets (mainly used by scrollviews)
    fn move_with_offsets(&mut self, offset_x: &Length, offset_y: &Length);

    /// Align the content of this node.
    fn align_content(
        &mut self,
        available_area: &Area,
        contents_area: &Size2D,
        alignment: &Alignment,
        direction: &DirectionMode,
        alignment_direction: AlignmentDirection,
    );

    /// Align the position of this node.
    #[allow(clippy::too_many_arguments)]
    fn align_position(
        &mut self,
        initial_available_area: &Area,
        inner_sizes: &Size2D,
        alignment: &Alignment,
        direction: &DirectionMode,
        alignment_direction: AlignmentDirection,
        siblings_len: usize,
        child_position: usize,
    );

    fn stack_into_node(
        &mut self,
        parent_node: &Node,
        parent_area: &mut Area,
        inner_area: &mut Area,
        content_area: &Area,
        inner_sizes: &mut Size2D,
        node_data: &Node,
    );

    fn fit_bounds_when_unspecified(
        &mut self,
        parent_area: &mut Area,
        inner_area: &mut Area,
        parent_node: &Node,
        alignment_direction: AlignmentDirection,
    );
}

impl AreaModel for Area {
    fn without_gaps(self, gaps: &Gaps) -> Area {
        let origin = self.origin;
        let size = self.size;
        Area::new(
            Point2D::new(origin.x + gaps.left(), origin.y + gaps.top()),
            Size2D::new(
                size.width - gaps.horizontal(),
                size.height - gaps.vertical(),
            ),
        )
    }

    fn move_with_offsets(&mut self, offset_x: &Length, offset_y: &Length) {
        self.origin.x += offset_x.get();
        self.origin.y += offset_y.get();
    }

    fn align_content(
        &mut self,
        inner_area: &Area,
        contents_size: &Size2D,
        alignment: &Alignment,
        direction: &DirectionMode,
        alignment_direction: AlignmentDirection,
    ) {
        let axis = get_align_axis(direction, alignment_direction);

        match axis {
            AlignAxis::Height => match alignment {
                Alignment::Center => {
                    let new_origin_y = (inner_area.height() / 2.0) - (contents_size.height / 2.0);

                    self.origin.y = inner_area.min_y() + new_origin_y;
                }
                Alignment::End => {
                    self.origin.y = inner_area.max_y() - contents_size.height;
                }
                _ => {}
            },
            AlignAxis::Width => match alignment {
                Alignment::Center => {
                    let new_origin_x = (inner_area.width() / 2.0) - (contents_size.width / 2.0);
                    self.origin.x = inner_area.min_x() + new_origin_x;
                }
                Alignment::End => {
                    self.origin.x = inner_area.max_x() - contents_size.width;
                }
                _ => {}
            },
        }
    }

    fn align_position(
        &mut self,
        initial_available_area: &Area,
        inner_sizes: &Size2D,
        alignment: &Alignment,
        direction: &DirectionMode,
        alignment_direction: AlignmentDirection,
        siblings_len: usize,
        child_position: usize,
    ) {
        let axis = get_align_axis(direction, alignment_direction);

        match axis {
            AlignAxis::Height => match alignment {
                Alignment::SpaceBetween if child_position > 0 => {
                    let all_gaps_sizes = initial_available_area.height() - inner_sizes.height;
                    let gap_size = all_gaps_sizes / (siblings_len - 1) as f32;
                    self.origin.y += gap_size;
                }
                Alignment::SpaceEvenly => {
                    let all_gaps_sizes = initial_available_area.height() - inner_sizes.height;
                    let gap_size = all_gaps_sizes / (siblings_len + 1) as f32;
                    self.origin.y += gap_size;
                }
                Alignment::SpaceAround => {
                    let all_gaps_sizes = initial_available_area.height() - inner_sizes.height;
                    let one_gap_size = all_gaps_sizes / siblings_len as f32;
                    let gap_size = if child_position == 0 || child_position == siblings_len {
                        one_gap_size / 2.
                    } else {
                        one_gap_size
                    };
                    self.origin.y += gap_size;
                }
                _ => {}
            },
            AlignAxis::Width => match alignment {
                Alignment::SpaceBetween if child_position > 0 => {
                    let all_gaps_sizes = initial_available_area.width() - inner_sizes.width;
                    let gap_size = all_gaps_sizes / (siblings_len - 1) as f32;
                    self.origin.x += gap_size;
                }
                Alignment::SpaceEvenly => {
                    let all_gaps_sizes = initial_available_area.width() - inner_sizes.width;
                    let gap_size = all_gaps_sizes / (siblings_len + 1) as f32;
                    self.origin.x += gap_size;
                }
                Alignment::SpaceAround => {
                    let all_gaps_sizes = initial_available_area.width() - inner_sizes.width;
                    let one_gap_size = all_gaps_sizes / siblings_len as f32;
                    let gap_size = if child_position == 0 || child_position == siblings_len {
                        one_gap_size / 2.
                    } else {
                        one_gap_size
                    };
                    self.origin.x += gap_size;
                }
                _ => {}
            },
        }
    }
    /// Stack a Node into another Node
    fn stack_into_node(
        &mut self,
        parent_node: &Node,
        parent_area: &mut Area,
        inner_area: &mut Area,
        content_area: &Area,
        inner_sizes: &mut Size2D,
        node_data: &Node,
    ) {
        if node_data.position.is_absolute() {
            return;
        }

        match parent_node.direction {
            DirectionMode::Horizontal => {
                // Move the available area
                self.origin.x = content_area.max_x();
                self.size.width -= content_area.size.width;

                inner_sizes.height = content_area.height().max(inner_sizes.height);
                inner_sizes.width += content_area.width();

                // Keep the biggest height
                if parent_node.height.inner_sized() {
                    parent_area.size.height = parent_area.size.height.max(
                        content_area.size.height
                            + parent_node.padding.vertical()
                            + parent_node.margin.vertical(),
                    );
                    // Keep the inner area in sync
                    inner_area.size.height = parent_area.size.height
                        - parent_node.padding.vertical()
                        - parent_node.margin.vertical();
                }

                // Accumulate width
                if parent_node.width.inner_sized() {
                    parent_area.size.width += content_area.size.width;
                }
            }
            DirectionMode::Vertical => {
                // Move the available area
                self.origin.y = content_area.max_y();
                self.size.height -= content_area.size.height;

                inner_sizes.width = content_area.width().max(inner_sizes.width);
                inner_sizes.height += content_area.height();

                // Keep the biggest width
                if parent_node.width.inner_sized() {
                    parent_area.size.width = parent_area.size.width.max(
                        content_area.size.width
                            + parent_node.padding.horizontal()
                            + parent_node.margin.horizontal(),
                    );
                    // Keep the inner area in sync
                    inner_area.size.width = parent_area.size.width
                        - parent_node.padding.horizontal()
                        - parent_node.margin.horizontal();
                }

                // Accumulate height
                if parent_node.height.inner_sized() {
                    parent_area.size.height += content_area.size.height;
                }
            }
        }
    }

    /// This will fit the available area and inner area of a parent node when for example height is set to "auto",
    /// direction is vertical and main_alignment is set to "center" or "end" or the content is set to "fit".
    /// The intended usage is to call this after the first measurement and before the second,
    /// this way the second measurement will align the content relatively to the parent element instead
    /// of overflowing due to being aligned relatively to the upper parent element
    fn fit_bounds_when_unspecified(
        &mut self,
        parent_area: &mut Area,
        inner_area: &mut Area,
        parent_node: &Node,
        alignment_direction: AlignmentDirection,
    ) {
        struct NodeData<'a> {
            pub inner_origin: &'a mut f32,
            pub inner_size: &'a mut f32,
            pub area_origin: &'a mut f32,
            pub area_size: &'a mut f32,
            pub one_side_padding: f32,
            pub two_sides_padding: f32,
            pub one_side_margin: f32,
            pub two_sides_margin: f32,
            pub available_size: &'a mut f32,
        }

        let axis = get_align_axis(&parent_node.direction, alignment_direction);
        let (is_vertical_not_start, is_horizontal_not_start) = match parent_node.direction {
            DirectionMode::Vertical => (
                parent_node.main_alignment.is_not_start(),
                parent_node.cross_alignment.is_not_start() || parent_node.content.is_fit(),
            ),
            DirectionMode::Horizontal => (
                parent_node.cross_alignment.is_not_start() || parent_node.content.is_fit(),
                parent_node.main_alignment.is_not_start(),
            ),
        };
        let NodeData {
            inner_origin,
            inner_size,
            area_origin,
            area_size,
            one_side_padding,
            two_sides_padding,
            one_side_margin,
            two_sides_margin,
            available_size,
        } = match axis {
            AlignAxis::Height if parent_node.height.inner_sized() && is_vertical_not_start => {
                NodeData {
                    inner_origin: &mut inner_area.origin.y,
                    inner_size: &mut inner_area.size.height,
                    area_origin: &mut parent_area.origin.y,
                    area_size: &mut parent_area.size.height,
                    one_side_padding: parent_node.padding.top(),
                    two_sides_padding: parent_node.padding.vertical(),
                    one_side_margin: parent_node.margin.top(),
                    two_sides_margin: parent_node.margin.vertical(),
                    available_size: &mut self.size.height,
                }
            }
            AlignAxis::Width if parent_node.width.inner_sized() && is_horizontal_not_start => {
                NodeData {
                    inner_origin: &mut inner_area.origin.x,
                    inner_size: &mut inner_area.size.width,
                    area_origin: &mut parent_area.origin.x,
                    area_size: &mut parent_area.size.width,
                    one_side_padding: parent_node.padding.left(),
                    two_sides_padding: parent_node.padding.horizontal(),
                    one_side_margin: parent_node.margin.left(),
                    two_sides_margin: parent_node.margin.horizontal(),
                    available_size: &mut self.size.width,
                }
            }
            _ => return,
        };

        // Set the origin of the inner area to the origin of the area plus the padding and margin for the given axis
        *inner_origin = *area_origin + one_side_padding + one_side_margin;
        // Set the size of the inner area to the size of the area minus the padding and margin for the given axis
        *inner_size = *area_size - two_sides_padding - two_sides_margin;
        // Set the same available size as the inner area for the given axis
        *available_size = *inner_size;
    }
}

pub fn get_align_axis(
    direction: &DirectionMode,
    alignment_direction: AlignmentDirection,
) -> AlignAxis {
    match direction {
        DirectionMode::Vertical => match alignment_direction {
            AlignmentDirection::Main => AlignAxis::Height,
            AlignmentDirection::Cross => AlignAxis::Width,
        },
        DirectionMode::Horizontal => match alignment_direction {
            AlignmentDirection::Main => AlignAxis::Width,
            AlignmentDirection::Cross => AlignAxis::Height,
        },
    }
}

pub enum AlignmentDirection {
    Main,
    Cross,
}

#[derive(Debug)]
pub enum AlignAxis {
    Height,
    Width,
}

pub trait SizeModel {
    /// Get the size with the given gap, e.g padding.
    fn with_gaps(self, gap: &Gaps) -> Size2D;
}

impl SizeModel for Size2D {
    fn with_gaps(self, gap: &Gaps) -> Size2D {
        Size2D::new(self.width + gap.horizontal(), self.height + gap.vertical())
    }
}
