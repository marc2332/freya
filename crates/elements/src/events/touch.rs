use torin::geometry::CursorPoint;
pub use winit::event::{
    Force,
    TouchPhase,
};

use crate::{
    events::ErasedEventData,
    impl_event,
};

impl_event! [
    TouchData;

    /// The `touchcancel` event fires when the user cancels the touching, this is usually caused by the hardware or the OS.
    /// Also see [`ontouchend`](crate::elements::ontouchend()).
    ///
    /// Event Data: [`TouchData`](crate::events::TouchData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100",
    ///             height: "100",
    ///             background: "red",
    ///             ontouchcancel: |_| println!("Touching canceled!")
    ///         }
    ///     )
    /// }
    /// ```
    ontouchcancel

    /// The `touchend` event fires when the user stops touching an element.
    ///
    /// Event Data: [`TouchData`](crate::events::TouchData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100",
    ///             height: "100",
    ///             background: "red",
    ///             ontouchend: |_| println!("Stopped touching!")
    ///         }
    ///     )
    /// }
    /// ```
    ontouchend

    /// The `touchmove` event fires when the user is touching over an element.
    ///
    /// Event Data: [`TouchData`](crate::events::TouchData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100",
    ///             height: "100",
    ///             background: "red",
    ///             ontouchmove: |_| println!("Touching!")
    ///         }
    ///     )
    /// }
    /// ```
    ontouchmove

    /// The `touchstart` event fires when the user starts touching an element.
    ///
    /// Event Data: [`TouchData`](crate::events::TouchData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100",
    ///             height: "100",
    ///             background: "red",
    ///             ontouchstart: |_| println!("Started touching!")
    ///         }
    ///     )
    /// }
    /// ```
    ontouchstart
];

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

impl From<&ErasedEventData> for TouchData {
    fn from(val: &ErasedEventData) -> Self {
        val.downcast::<TouchData>().cloned().unwrap()
    }
}
