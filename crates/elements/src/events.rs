pub mod file;
pub mod keyboard;
pub mod mouse;
pub mod pointer;
pub mod touch;
pub mod wheel;

use std::any::Any;

use dioxus_core::Event;
pub use file::*;
pub use keyboard::*;
pub use mouse::*;
pub use pointer::*;
pub use touch::*;
pub use wheel::*;

pub type KeyboardEvent = Event<KeyboardData>;
pub type MouseEvent = Event<MouseData>;
pub type WheelEvent = Event<WheelData>;
pub type TouchEvent = Event<TouchData>;
pub type PointerEvent = Event<PointerData>;

/// A platform specific event.
#[doc(hidden)]
pub struct ErasedEventData {
    event: Box<dyn Any>,
}

impl ErasedEventData {
    pub fn new(event: Box<dyn Any>) -> Self {
        Self { event }
    }

    pub fn downcast<T: 'static>(&self) -> Option<&T> {
        self.event.downcast_ref::<T>()
    }

    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.event.downcast_mut::<T>()
    }

    pub fn into_inner<T: 'static>(self) -> Option<T> {
        self.event.downcast::<T>().ok().map(|e| *e)
    }
}
