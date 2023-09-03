use crate::prelude::Gaps;

#[derive(PartialEq)]
pub struct Measure;

pub type Area = euclid::Rect<f32, Measure>;
pub type Size2D = euclid::Size2D<f32, Measure>;
pub type Point2D = euclid::Point2D<f32, Measure>;
pub type CursorPoint = euclid::Point2D<f64, Measure>;
pub type Length = euclid::Length<f32, Measure>;

pub trait BoxModel {
    // The area without any outer gap (e.g margin)
    fn box_area(&self, margin: &Gaps) -> Area;
}

impl BoxModel for Area {
    fn box_area(&self, margin: &Gaps) -> Area {
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
}
