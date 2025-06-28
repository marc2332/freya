use crate::{
    events::ErasedEventData,
    impl_event,
};

impl_event! [
    WheelData;

    /// The `wheel` event fires when the user scrolls the mouse wheel while hovering over the element.
    ///
    /// Event Data: [`WheelData`](crate::events::WheelData)
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
    ///             onwheel: |_| println!("Scrolling with the wheel!")
    ///         }
    ///     )
    /// }
    /// ```
    onwheel
];

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WheelSource {
    Device,
    Custom,
}

/// Data of a Wheel event.
#[derive(Debug, Clone, PartialEq)]
pub struct WheelData {
    source: WheelSource,
    delta_x: f64,
    delta_y: f64,
}

impl WheelData {
    pub fn new(source: WheelSource, delta_x: f64, delta_y: f64) -> Self {
        Self {
            source,
            delta_x,
            delta_y,
        }
    }
}

impl WheelData {
    /// Get the source of the wheel event.
    pub fn get_source(&self) -> WheelSource {
        self.source
    }

    /// Get the X delta.
    pub fn get_delta_x(&self) -> f64 {
        self.delta_x
    }

    /// Get the Y delta.
    pub fn get_delta_y(&self) -> f64 {
        self.delta_y
    }
}

impl From<&ErasedEventData> for WheelData {
    fn from(val: &ErasedEventData) -> Self {
        val.downcast::<WheelData>().cloned().unwrap()
    }
}
