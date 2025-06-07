use std::num::NonZeroU32;

use torin::prelude::Size2D;
use winit::dpi::PhysicalSize;

pub trait WinitSize {
    fn to_skia(self) -> (i32, i32);

    fn to_torin(self) -> Size2D;

    fn as_gl_width(&self) -> NonZeroU32;

    fn as_gl_height(&self) -> NonZeroU32;
}

impl WinitSize for PhysicalSize<u32> {
    fn to_skia(self) -> (i32, i32) {
        (self.width.max(1) as i32, self.height.max(1) as i32)
    }

    fn to_torin(self) -> Size2D {
        Size2D::new(self.width as f32, self.height as f32)
    }

    fn as_gl_width(&self) -> NonZeroU32 {
        NonZeroU32::new(self.width.max(1)).unwrap()
    }

    fn as_gl_height(&self) -> NonZeroU32 {
        NonZeroU32::new(self.height.max(1)).unwrap()
    }
}
