pub use euclid::Rect;

use crate::geometry::Length;

#[derive(PartialEq, Clone, Debug, Default, Copy)]
pub struct Radius {
    top_left: Length,
    top_right: Length,
    bottom_left: Length,
    bottom_right: Length,
}

impl Radius {
    pub fn new(top_left: f32, top_right: f32, bottom_left: f32, bottom_right: f32) -> Self {
        Self {
            top_left: Length::new(top_left),
            top_right: Length::new(top_right),
            bottom_left: Length::new(bottom_left),
            bottom_right: Length::new(bottom_right),
        }
    }

    pub fn fill_top(&mut self, value: f32) {
        self.top_left = Length::new(value);
        self.top_right = Length::new(value);
    }

    pub fn fill_bottom(&mut self, value: f32) {
        self.bottom_left = Length::new(value);
        self.bottom_right = Length::new(value);
    }

    pub fn fill_all(&mut self, value: f32) {
        self.fill_bottom(value);
        self.fill_top(value);
    }

    pub fn top_left(&self) -> f32 {
        self.top_left.get()
    }

    pub fn top_right(&self) -> f32 {
        self.top_right.get()
    }

    pub fn bottom_left(&self) -> f32 {
        self.bottom_left.get()
    }

    pub fn bottom_right(&self) -> f32 {
        self.bottom_right.get()
    }

    pub fn pretty(&self) -> String {
        format!(
            "({}, {}, {}, {})",
            self.top_left(),
            self.top_right(),
            self.bottom_right(),
            self.bottom_left()
        )
    }
}