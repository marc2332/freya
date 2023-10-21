use crate::prelude::{Alignment, DirectionMode, Gaps};

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

    fn move_with_offsets(&mut self, offset_x: &Length, offset_y: &Length);

    fn align_content(
        &mut self,
        available_area: &Area,
        contents_area: &Size2D,
        alignment: &Alignment,
        direction: &DirectionMode,
        alignment_direction: AlignmentDirection,
    );
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

pub enum AlignAxis {
    Height,
    Width,
}
