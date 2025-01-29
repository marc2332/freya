use torin::geometry::CursorPoint;
pub use winit::event::MouseButton;

use crate::{
    events::ErasedEventData,
    impl_event,
};
impl_event! [
    MouseData;

    /// The `click` event fires when the user starts and ends a click in an element with the left button of the mouse.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
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
    ///             onclick: |_| println!("Clicked!")
    ///         }
    ///     )
    /// }
    /// ```
    onclick

    /// The `globalclick` event fires when the user clicks anywhere.
    /// Note that this fires for all mouse buttons.
    /// You can check the specific variant with the [`MouseData`](crate::events::MouseData)'s `trigger_button` property.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             onglobalclick: |_| println!("Clicked somewhere else!")
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
    onglobalclick

    /// The `click` event fires when the user clicks an element with the middle button of the mouse.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
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
    ///             onmiddleclick: |_| println!("Clicked!")
    ///         }
    ///     )
    /// }
    /// ```
    onmiddleclick

    /// The `click` event fires when the user clicks an element with the right button of the mouse.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
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
    ///             onrightclick: |_| println!("Clicked!")
    ///         }
    ///     )
    /// }
    /// ```
    onrightclick

    /// The `mouseup` event fires when the user ends the click in an element with the left button of the mouse.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
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
    ///             onmouseup: |_| println!("Clicked!")
    ///         }
    ///     )
    /// }
    /// ```
    onmouseup

    /// The `mousedown` event fires when the user starts clicking an element.
    /// Note that this fires for all mouse buttons.
    /// You can check the specific variant with the [`MouseData`](crate::events::MouseData)'s `trigger_button` property.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
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
    ///             onmousedown: |_| println!("Started clicking!")
    ///         }
    ///     )
    /// }
    /// ```
    onmousedown

    /// The `globalmousedown` event fires when the user starts clicking anywhere.
    /// Note that this fires for all mouse buttons.
    /// You can check the specific variant with the [`MouseData`](crate::events::MouseData)'s `trigger_button` property.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             onglobalmousedown: |_| println!("Started clicking somewhere else!")
    ///         }
    ///         rect {
    ///             width: "100",
    ///             height: "100",
    ///             background: "red",
    ///             onmousedown: |_| println!("Started clicking!")
    ///         }
    ///     )
    /// }
    /// ```
    onglobalmousedown

    /// The `mousemove` event fires when the user moves the mouse over an element.
    /// Unlike [`onmouseenter`](crate::events::onmouseenter()), this fires even if the user was already hovering over
    /// the element. For that reason, it's less efficient.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
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
    ///             onmousemove: |_| println!("Hovering!")
    ///         }
    ///     )
    /// }
    /// ```
    onmousemove

    /// The `globalmousemove` event fires when the user moves the mouse anywhere in the app.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             onglobalmousemove: |_| println!("Moving the mouse anywhere!")
    ///         }
    ///         rect {
    ///             width: "100",
    ///             height: "100",
    ///             background: "red",
    ///             onmousemove: |_| println!("Moving the mouse here!")
    ///         }
    ///     )
    /// }
    /// ```
    onglobalmousemove

    /// The `mouseleave` event fires when the user stops hovering an element.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
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
    ///             onmouseleave: |_| println!("Stopped hovering!")
    ///         }
    ///     )
    /// }
    /// ```
    onmouseleave

    /// The `mouseenter` event fires when the user starts hovering an element.
    ///
    /// Event Data: [`MouseData`](crate::events::MouseData)
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
    ///             onmouseenter: |_| println!("Started hovering!")
    ///         }
    ///     )
    /// }
    /// ```
    onmouseenter
];

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

impl From<&ErasedEventData> for MouseData {
    fn from(val: &ErasedEventData) -> Self {
        val.downcast::<MouseData>().cloned().unwrap()
    }
}
