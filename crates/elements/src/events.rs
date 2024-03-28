pub mod file;
pub mod keyboard;
pub mod mouse;
pub mod pointer;
pub mod touch;
pub mod wheel;

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
