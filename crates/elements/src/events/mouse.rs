use torin::geometry::CursorPoint;
pub use winit::event::MouseButton;

use crate::definitions::PlatformEventData;

/// Data of a Mouse event.
#[derive(Debug, Clone, PartialEq)]
pub struct MouseData {
    pub screen_coordinates: CursorPoint,
    pub element_coordinates: CursorPoint,
    pub trigger_button: Option<MouseButton>,
}

impl MouseData {
    pub fn new(
        screen_coordinates: CursorPoint,
        element_coordinates: CursorPoint,
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
    pub fn get_screen_coordinates(&self) -> CursorPoint {
        self.screen_coordinates
    }

    /// Get the mouse coordinates relatives to the element bounds.
    pub fn get_element_coordinates(&self) -> CursorPoint {
        self.element_coordinates
    }

    /// Get the button that triggered this event.
    pub fn get_trigger_button(&self) -> Option<MouseButton> {
        self.trigger_button
    }
}

impl From<&PlatformEventData> for MouseData {
    fn from(val: &PlatformEventData) -> Self {
        val.downcast::<MouseData>().cloned().unwrap()
    }
}
