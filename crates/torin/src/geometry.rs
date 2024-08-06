use crate::prelude::{
    DirectionMode,
    Gaps,
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
