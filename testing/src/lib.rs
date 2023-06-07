pub use freya_core::events::FreyaEvent;
pub use freya_elements::events::mouse::MouseButton;

mod config;
mod launch;
mod test_handler;
mod test_node;
mod test_utils;

const SCALE_FACTOR: f64 = 1.0;

pub use config::*;
pub use launch::*;
pub use test_handler::*;
pub use test_node::*;
pub use test_utils::*;
