pub use config::*;
use freya_native_core::NodeId;
use std::sync::{Arc, Mutex};

pub use config::WindowConfig;
pub use window::DesktopRenderer;

mod accessibility;
mod app;
mod config;
mod elements;
mod renderer;
mod window;
mod winit_waker;
mod wireframe;

pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;
