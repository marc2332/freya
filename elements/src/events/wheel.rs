/// Data of a Wheel event.
#[derive(Debug, Clone)]
pub struct WheelData {
    #[allow(dead_code)]
    delta_x: f64,
    delta_y: f64,
}

impl WheelData {
    pub fn new(delta_x: f64, delta_y: f64) -> Self {
        Self { delta_x, delta_y }
    }
}

impl WheelData {
    /// Get the X delta.
    pub fn get_delta_x(&self) -> f64 {
        self.delta_x
    }

    /// Get the Y delta.
    pub fn get_delta_y(&self) -> f64 {
        self.delta_y
    }
}
