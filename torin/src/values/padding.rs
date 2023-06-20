pub use euclid::Rect;

use crate::geometry::Length;
use crate::scaled::Scaled;

#[derive(PartialEq, Clone, Debug, Default, Copy)]
pub struct Paddings {
    top: Length,
    right: Length,
    bottom: Length,
    left: Length,
}

impl Paddings {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: Length::new(top),
            right: Length::new(right),
            bottom: Length::new(bottom),
            left: Length::new(left),
        }
    }

    pub fn fill_vertical(&mut self, value: f32) {
        self.top = Length::new(value);
        self.bottom = Length::new(value);
    }

    pub fn fill_horizontal(&mut self, value: f32) {
        self.right = Length::new(value);
        self.left = Length::new(value);
    }

    pub fn fill_all(&mut self, value: f32) {
        self.fill_horizontal(value);
        self.fill_vertical(value);
    }

    pub fn horizontal_paddings(&self) -> f32 {
        (self.right + self.left).get()
    }

    pub fn vertical_paddings(&self) -> f32 {
        (self.top + self.bottom).get()
    }

    pub fn top(&self) -> f32 {
        self.top.get()
    }

    pub fn right(&self) -> f32 {
        self.right.get()
    }

    pub fn bottom(&self) -> f32 {
        self.bottom.get()
    }

    pub fn left(&self) -> f32 {
        self.left.get()
    }

    pub fn pretty(&self) -> String {
        format!(
            "({}, {}, {}, {})",
            self.top(),
            self.right(),
            self.bottom(),
            self.left()
        )
    }
}

impl Scaled for Paddings {
    fn scale(&mut self, scale: f32) {
        let logical_subpixels = (
            self.left.get().fract(),
            self.right.get().fract(),
            self.top.get().fract(),
            self.bottom.get().fract()
        );

        self.left *= scale;
        self.right *= scale;
        self.top *= scale;
        self.bottom *= scale;

        if logical_subpixels.0 == 0.0 && logical_subpixels.1 == 0.0 {
            self.left = Length::new(self.left.get().round());
            self.right = Length::new(self.right.get().round());
        }
        if logical_subpixels.2 == 0.0 && logical_subpixels.3 == 0.0 {
            self.top = Length::new(self.top.get().round());
            self.bottom = Length::new(self.bottom.get().round());
        }
    }
}