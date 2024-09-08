use std::sync::{
    Arc,
    Mutex,
};

pub use config::{
    WindowConfig,
    *,
};
use freya_native_core::NodeId;
pub use renderer::DesktopRenderer;

mod accessibility;
mod app;
mod config;
pub mod devtools;
mod renderer;
mod size;
mod window_state;
mod winit_waker;

pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;
