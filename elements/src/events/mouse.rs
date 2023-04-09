use freya_common::Point2D;
pub use winit::event::MouseButton;

/// Data of a Mouse event.
#[derive(Debug, Clone)]
pub struct MouseData {
    pub screen_coordinates: Point2D,
    pub element_coordinates: Point2D,
    pub trigger_button: Option<MouseButton>,
}

impl MouseData {
    pub fn new(
        screen_coordinates: Point2D,
        element_coordinates: Point2D,
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
    /// Get the mouse coordinates relative to the window bounds.
    pub fn get_screen_coordinates(&self) -> Point2D {
        self.screen_coordinates
    }

    /// Get the mouse coordinates relatives to the element bounds.
    pub fn get_element_coordinates(&self) -> Point2D {
        self.element_coordinates
    }

    /// Get the button that triggered this event.
    pub fn get_trigger_button(&self) -> Option<MouseButton> {
        self.trigger_button
    }
}
