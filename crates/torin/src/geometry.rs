use std::f32::consts::PI;

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

    fn max_area_when_rotated(&self, center: Point2D) -> Area;

    fn clip(&mut self, other: &Self);
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

    fn max_area_when_rotated(&self, center: Point2D) -> Area {
        let (top_left_extreme, bottom_right_extreme) = calculate_extreme_corners(self, center);

        Area::new(
            Point2D::new(top_left_extreme.x, top_left_extreme.y),
            Size2D::new(
                bottom_right_extreme.x - top_left_extreme.x,
                bottom_right_extreme.y - top_left_extreme.y,
            ),
        )
    }

    fn clip(&mut self, other: &Self) {
        self.origin.x = self.origin.x.max(other.origin.x);
        self.origin.y = self.origin.y.max(other.origin.y);
        self.size.width = self.size.width.min(other.size.width);
        self.size.height = self.size.height.min(other.size.height);
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

fn rotate_point_around_center(point: Point2D, center: Point2D, angle_radians: f32) -> Point2D {
    let sin_theta = angle_radians.sin();
    let cos_theta = angle_radians.cos();

    let x_shifted = point.x - center.x;
    let y_shifted = point.y - center.y;

    let x_rotated = x_shifted * cos_theta - y_shifted * sin_theta;
    let y_rotated = x_shifted * sin_theta + y_shifted * cos_theta;

    Point2D::new(x_rotated + center.x, y_rotated + center.y)
}

fn calculate_extreme_corners(area: &Area, center: Point2D) -> (Point2D, Point2D) {
    let biggest_side_width = (center.x - area.min_x()).max(area.max_x() - center.x);
    let biggest_side_height = (center.y - area.min_y()).max(area.max_y() - center.y);

    let corners = [
        Point2D::new(
            center.x - biggest_side_width,
            center.y - biggest_side_height,
        ),
        Point2D::new(
            center.x - biggest_side_width,
            center.y + biggest_side_height,
        ),
        Point2D::new(
            center.x + biggest_side_width,
            center.y - biggest_side_height,
        ),
        Point2D::new(
            center.x + biggest_side_width,
            center.y + biggest_side_height,
        ),
    ];

    let angle_45_radians = 45.0 * PI / 180.0;

    let rotated_corners: Vec<Point2D> = corners
        .iter()
        .map(|&corner| rotate_point_around_center(corner, center, angle_45_radians))
        .collect();

    let min_x = rotated_corners
        .iter()
        .map(|p| p.x)
        .fold(f32::INFINITY, |a, b| a.min(b));
    let min_y = rotated_corners
        .iter()
        .map(|p| p.y)
        .fold(f32::INFINITY, |a, b| a.min(b));
    let max_x = rotated_corners
        .iter()
        .map(|p| p.x)
        .fold(f32::NEG_INFINITY, |a, b| a.max(b));
    let max_y = rotated_corners
        .iter()
        .map(|p| p.y)
        .fold(f32::NEG_INFINITY, |a, b| a.max(b));

    (Point2D::new(min_x, min_y), Point2D::new(max_x, max_y))
}
