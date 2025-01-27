use torin::geometry::CursorPoint;
pub use winit::event::MouseButton;
use winit::event::{
    Force,
    TouchPhase,
};

use crate::definitions::ErasedEventData;

/// The type of device that triggered a Pointer event.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PointerType {
    Mouse {
        trigger_button: Option<MouseButton>,
    },
    Touch {
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    },
}

/// Data of a Mouse event.
#[derive(Debug, Clone, PartialEq)]
pub struct PointerData {
    pub screen_coordinates: CursorPoint,
    pub element_coordinates: CursorPoint,
    pub pointer_type: PointerType,
}

impl PointerData {
    pub fn new(
        screen_coordinates: CursorPoint,
        element_coordinates: CursorPoint,
        point_type: PointerType,
    ) -> Self {
        Self {
            screen_coordinates,
            element_coordinates,
            pointer_type: point_type,
        }
    }
}

impl PointerData {
    /// Get the mouse coordinates relative to the window bounds.
    pub fn get_screen_coordinates(&self) -> CursorPoint {
        self.screen_coordinates
    }

    /// Get the mouse coordinates relatives to the element bounds.
    pub fn get_element_coordinates(&self) -> CursorPoint {
        self.element_coordinates
    }

    /// Get the pointer type that triggered this event.
    pub fn get_pointer_type(&self) -> PointerType {
        self.pointer_type
    }
}

impl From<&ErasedEventData> for PointerData {
    fn from(val: &ErasedEventData) -> Self {
        val.downcast::<PointerData>().cloned().unwrap()
    }
}
