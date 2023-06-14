pub use euclid::Rect;

use crate::geometry::Length;

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


pub fn parse_radius(value: &str, scale_factor: f32) -> Option<Radius> {
    let mut radius_config = Radius::default();
    let mut radius = value.split_ascii_whitespace();

    match radius.clone().count() {
        // Same in all corners
        1 => {
            radius_config.fill_all(radius.next()?.parse::<f32>().ok()? * scale_factor);
        }
        // By Top and Bottom
        2 => {
            // Top
            radius_config.fill_top(radius.next()?.parse::<f32>().ok()? * scale_factor);

            // Bottom
            radius_config.fill_bottom(radius.next()?.parse::<f32>().ok()? * scale_factor)
        }
        // Each corner
        4 => {
            radius_config = Radius::new(
                radius.next()?.parse::<f32>().ok()? * scale_factor,
                radius.next()?.parse::<f32>().ok()? * scale_factor,
                radius.next()?.parse::<f32>().ok()? * scale_factor,
                radius.next()?.parse::<f32>().ok()? * scale_factor,
            );
        }
        _ => {}
    }

    Some(radius_config)
}