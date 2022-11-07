mod keyboard;
mod mouse;
mod wheel;

use dioxus_core::UiEvent;
pub use keyboard::*;
pub use mouse::*;
pub use wheel::*;

pub type KeyboardEvent = UiEvent<KeyboardData>;
pub type MouseEvent = UiEvent<MouseData>;
pub type WheelEvent = UiEvent<WheelData>;
