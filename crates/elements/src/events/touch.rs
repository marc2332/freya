use torin::geometry::CursorPoint;
pub use winit::event::{Force, TouchPhase};

use crate::definitions::PlatformEventData;

/// Data of a Touch event.
#[derive(Debug, Clone, PartialEq)]
pub struct TouchData {
    pub screen_coordinates: CursorPoint,
    pub element_coordinates: CursorPoint,
    pub finger_id: u64,
    pub phase: TouchPhase,
    pub force: Option<Force>,
}

impl TouchData {
    pub fn new(
        screen_coordinates: CursorPoint,
        element_coordinates: CursorPoint,
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
    pub fn get_screen_coordinates(&self) -> CursorPoint {
        self.screen_coordinates
    }

    /// Get the touch coordinates relatives to the element bounds.
    pub fn get_element_coordinates(&self) -> CursorPoint {
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

impl From<&PlatformEventData> for TouchData {
    fn from(val: &PlatformEventData) -> Self {
        val.downcast::<TouchData>().cloned().unwrap()
    }
}
