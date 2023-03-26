mod keyboard;
mod mouse;
mod touch;
mod wheel;

use dioxus_core::Event;
pub use keyboard::*;
pub use mouse::*;
pub use touch::*;
pub use wheel::*;

pub type KeyboardEvent = Event<KeyboardData>;
pub type MouseEvent = Event<MouseData>;
pub type WheelEvent = Event<WheelData>;
pub type TouchEvent = Event<TouchData>;
