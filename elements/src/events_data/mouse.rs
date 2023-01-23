use euclid::Point2D;
pub use glutin::event::MouseButton;

#[derive(Debug, Clone)]
pub struct MouseData {
    pub screen_coordinates: Point2D<f64, f64>,
    pub element_coordinates: Point2D<f64, f64>,
    pub trigger_button: Option<MouseButton>,
}

impl MouseData {
    pub fn new(
        screen_coordinates: Point2D<f64, f64>,
        element_coordinates: Point2D<f64, f64>,
        trigger_button: Option<MouseButton>,
    ) -> Self {
        Self {
            screen_coordinates,
            element_coordinates,
            trigger_button,
        }
    }
}

impl MouseData {
    pub fn get_screen_coordinates(&self) -> Point2D<f64, f64> {
        self.screen_coordinates
    }

    pub fn get_element_coordinates(&self) -> Point2D<f64, f64> {
        self.element_coordinates
    }

    pub fn get_trigger_button(&self) -> Option<MouseButton> {
        self.trigger_button
    }
}
