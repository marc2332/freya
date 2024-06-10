pub use config::*;
use freya_native_core::NodeId;
use std::sync::{Arc, Mutex};

pub use config::WindowConfig;
pub use renderer::DesktopRenderer;

mod accessibility;
mod app;
mod config;
pub mod devtools;
mod elements;
mod render;
mod renderer;
mod winit_waker;
mod wireframe;

pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;
