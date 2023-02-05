/// An area starting at point `x` and `y` with a certain `width` and `height`.
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct NodeArea {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl NodeArea {
    /// Checks if an area is slightly outside of this area
    #[inline(always)]
    pub fn is_area_outside(&self, other_area: Self) -> bool {
        other_area.x + other_area.width < self.x
            || other_area.y + other_area.height < self.y
            || other_area.x > self.x + self.width
            || other_area.y > self.y + self.height
    }

    /// Checks if the given cursor is outside this area or not
    #[inline(always)]
    pub fn is_point_outside(&self, cursor: (f64, f64)) -> bool {
        cursor.0 < self.x as f64
            || cursor.0 > (self.x + self.width) as f64
            || cursor.1 < self.y as f64
            || cursor.1 > (self.y + self.height) as f64
    }

    /// Calculates the origin x and y points alongside the destination points
    #[inline(always)]
    pub fn get_rect(&self) -> ((f64, f64), (f64, f64)) {
        let x = self.x as f64;
        let y = self.y as f64;

        let x2 = x + self.width as f64;
        let y2 = y + self.height as f64;

        ((x, y), (x2, y2))
    }

    /// Calculates the origin x and y points
    #[inline(always)]
    pub fn get_origin_points(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
