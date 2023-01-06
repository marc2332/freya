mod keyboard;
mod mouse;
mod wheel;

use dioxus_core::Event;
pub use keyboard::*;
pub use mouse::*;
pub use wheel::*;

pub type KeyboardEvent = Event<KeyboardData>;
pub type MouseEvent = Event<MouseData>;
pub type WheelEvent = Event<WheelData>;
