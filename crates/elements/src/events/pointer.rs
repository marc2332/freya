use torin::geometry::CursorPoint;
pub use winit::event::MouseButton;
use winit::event::{
    Force,
    TouchPhase,
};

use crate::{
    events::ErasedEventData,
    impl_event,
};
impl_event! [
    PointerData;

    /// The `pointerdown` event fires when the user clicks/starts touching an element.
    ///
    /// Event Data: [`PointerData`](crate::events::PointerData)
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
    ///             onpointerdown: |_| println!("Clicked/started pressing!")
    ///         }
    ///     )
    /// }
    /// ```
    onpointerdown

    /// The `pointerup` event fires when the user releases their mouse button or stops touching the element.
    ///
    /// Event Data: [`PointerData`](crate::events::PointerData)
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
    ///             onpointerup: |_| println!("Released mouse button, or no longer touching!")
    ///         }
    ///     )
    /// }
    /// ```
    onpointerup

    /// The `globalpointerup` event fires when the user releases the point anywhere in the app.
    ///
    /// Event Data: [`PointerData`](crate::events::PointerData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             onglobalpointerup: |_| println!("Pointer released somewhere else!")
    ///         }
    ///         rect {
    ///             width: "100",
    ///             height: "100",
    ///             background: "red",
    ///             onclick: |_| println!("Clicked!")
    ///         }
    ///     )
    /// }
    /// ```
    onglobalpointerup

    /// The `pointermove` event fires when the user moves the cursor or touches over an element.
    /// Unlike [`onpointerenter`](crate::events::onpointerenter()), this fires even if the user was already hovering over
    /// the element.
    ///
    /// Event Data: [`PointerData`](crate::events::PointerData)
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
    ///             onpointermove: |_| println!("Moving or touching!")
    ///         }
    ///     )
    /// }
    /// ```
    onpointermove

    /// The `pointerenter` event fires when the user starts hovering/touching an element.
    ///
    /// Event Data: [`PointerData`](crate::events::PointerData)
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
    ///             onpointerenter: |_| println!("Started hovering or touching!")
    ///         }
    ///     )
    /// }
    /// ```
    onpointerenter

    /// The `pointerleave` event fires when the user stops hovering/touching an element.
    ///
    /// Event Data: [`PointerData`](crate::events::PointerData)
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
    ///             onpointerleave: |_| println!("Started hovering or touching!")
    ///         }
    ///     )
    /// }
    /// ```
    onpointerleave
];

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
