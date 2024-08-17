use crate::{
    node::Node,
    prelude::{
        Alignment,
        DirectionMode,
        Gaps,
        Size,
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
    // The area without any outer gap (e.g margin)
    fn after_gaps(&self, margin: &Gaps) -> Area;

    // Adjust the available area with the node offsets (mainly used by scrollviews)
    fn move_with_offsets(&mut self, offset_x: &Length, offset_y: &Length);

    // Align the content of this node.
    fn align_content(
        &mut self,
        available_area: &Area,
        contents_area: &Size2D,
        alignment: &Alignment,
        direction: &DirectionMode,
        alignment_direction: AlignmentDirection,
    );

    // Align the position of this node.
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

    fn adjust_size(&mut self, node: &Node);

    fn expand(&mut self, size: &Size2D);
}

impl AreaModel for Area {
    /// Get the area inside after including the gaps (margins or paddings)
    fn after_gaps(&self, margin: &Gaps) -> Area {
        let origin = self.origin;
        let size = self.size;
        Area::new(
            Point2D::new(origin.x + margin.left(), origin.y + margin.top()),
            Size2D::new(
                size.width - margin.horizontal(),
                size.height - margin.vertical(),
            ),
        )
    }

    /// Get the area inside after including the gaps (margins or paddings)
    fn move_with_offsets(&mut self, offset_x: &Length, offset_y: &Length) {
        self.origin.x += offset_x.get();
        self.origin.y += offset_y.get();
    }

    fn align_content(
        &mut self,
        available_area: &Area,
        contents_size: &Size2D,
        alignment: &Alignment,
        direction: &DirectionMode,
        alignment_direction: AlignmentDirection,
    ) {
        let axis = get_align_axis(direction, alignment_direction);

        match axis {
            AlignAxis::Height => match alignment {
                Alignment::Center => {
                    let new_origin_y =
                        (available_area.height() / 2.0) - (contents_size.height / 2.0);

                    self.origin.y = available_area.min_y() + new_origin_y;
                }
                Alignment::End => {
                    self.origin.y = available_area.max_y() - contents_size.height;
                }
                _ => {}
            },
            AlignAxis::Width => match alignment {
                Alignment::Center => {
                    let new_origin_x = (available_area.width() / 2.0) - (contents_size.width / 2.0);

                    self.origin.x = available_area.min_x() + new_origin_x;
                }
                Alignment::End => {
                    self.origin.x = available_area.max_x() - contents_size.width;
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

    fn adjust_size(&mut self, node: &Node) {
        if let Size::InnerPercentage(p) = node.width {
            self.size.width *= p.get() / 100.;
        }
        if let Size::InnerPercentage(p) = node.height {
            self.size.height *= p.get() / 100.;
        }
    }

    fn expand(&mut self, size: &Size2D) {
        self.origin.x -= size.width;
        self.origin.y -= size.height;
        self.size.width += size.width * 2.;
        self.size.height += size.height * 2.;
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
