use euclid::Point2D;

/// Data of a Touch event.
#[derive(Debug, Clone)]
pub struct TouchData {
    pub screen_coordinates: Point2D<f64, f64>,
    pub element_coordinates: Point2D<f64, f64>,
    pub finger_id: u64,
}

impl TouchData {
    pub fn new(
        screen_coordinates: Point2D<f64, f64>,
        element_coordinates: Point2D<f64, f64>,
        finger_id: u64,
    ) -> Self {
        Self {
            screen_coordinates,
            element_coordinates,
            finger_id,
        }
    }

    /// Get the touch coordinates relative to the window bounds.
    pub fn get_screen_coordinates(&self) -> Point2D<f64, f64> {
        self.screen_coordinates
    }

    /// Get the touch coordinates relatives to the element bounds.
    pub fn get_element_coordinates(&self) -> Point2D<f64, f64> {
        self.element_coordinates
    }

    /// Get the finger that triggered this event
    pub fn get_finger_id(&self) -> u64 {
        self.finger_id
    }
}
