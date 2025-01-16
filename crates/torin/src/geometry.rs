use core::f32::consts::PI;
extern crate alloc;
use alloc::vec::Vec;
use core::{
    cmp::Eq,
    default::Default,
    option::{
        Option,
        Option::{
            None,
            Some,
        },
    },
};

use crate::{
    node::Node,
    prelude::{
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
    /// The area without any outer gap (e.g margin)
    fn without_gaps(self, gap: &Gaps) -> Area;

    /// Adjust the available area with the node offsets (mainly used by scrollviews)
    fn move_with_offsets(&mut self, offset_x: &Length, offset_y: &Length);

    /// Adjust the size given the Node data
    fn adjust_size(&mut self, node: &Node);

    fn expand(&mut self, size: &Size2D);

    fn max_area_when_rotated(&self, center: Point2D) -> Area;

    fn clip(&mut self, other: &Self);
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
    let biggest_size = area.width().max(area.height());

    let corners = [
        Point2D::new(center.x - biggest_size, center.y - biggest_size),
        Point2D::new(center.x - biggest_size, center.y + biggest_size),
        Point2D::new(center.x + biggest_size, center.y - biggest_size),
        Point2D::new(center.x + biggest_size, center.y + biggest_size),
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

impl AlignAxis {
    pub fn new(direction: &DirectionMode, alignment_direction: AlignmentDirection) -> Self {
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
