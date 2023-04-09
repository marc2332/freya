use freya_common::Point2D;
pub use winit::event::{Force, TouchPhase};

/// Data of a Touch event.
#[derive(Debug, Clone)]
pub struct TouchData {
    pub screen_coordinates: Point2D,
    pub element_coordinates: Point2D,
    pub finger_id: u64,
    pub phase: TouchPhase,
    pub force: Option<Force>,
}

impl TouchData {
    pub fn new(
        screen_coordinates: Point2D,
        element_coordinates: Point2D,
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    ) -> Self {
        Self {
            screen_coordinates,
            element_coordinates,
            finger_id,
            phase,
            force,
        }
    }

    /// Get the touch coordinates relative to the window bounds.
    pub fn get_screen_coordinates(&self) -> Point2D {
        self.screen_coordinates
    }

    /// Get the touch coordinates relatives to the element bounds.
    pub fn get_element_coordinates(&self) -> Point2D {
        self.element_coordinates
    }

    /// Get the finger that triggered this event.
    pub fn get_finger_id(&self) -> u64 {
        self.finger_id
    }

    /// Get the touch phase of this event.
    pub fn get_touch_phase(&self) -> TouchPhase {
        self.phase
    }

    /// Get the touch force of this event.
    pub fn get_touch_force(&self) -> Option<Force> {
        self.force
    }
}
