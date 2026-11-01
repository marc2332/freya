use freya_engine::prelude::SkRect;
use torin::prelude::{
    Area,
    AreaOf,
    Point2D,
    Size2D,
};

pub trait AreaExt {
    fn to_skia(&self) -> SkRect;
}

impl<Unit> AreaExt for AreaOf<Unit> {
    fn to_skia(&self) -> SkRect {
        SkRect::new(self.min_x(), self.min_y(), self.max_x(), self.max_y())
    }
}

pub trait SkRectExt {
    fn to_torin(&self) -> Area;
}

impl SkRectExt for SkRect {
    fn to_torin(&self) -> Area {
        Area::new(
            Point2D::new(self.left, self.top),
            Size2D::new(self.width(), self.height()),
        )
    }
}
