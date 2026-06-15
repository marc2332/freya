//! Camera capture for Freya.
//!
//! - [`use_camera`](use_camera::use_camera): a hook that streams frames from a
//!   camera into reactive state.
//! - [`CameraViewer`](camera_viewer::CameraViewer): a component that renders
//!   the captured frames.
//!
//! This crate is reexported in `freya::camera`.
//!
//! # Example
//!
//! ```rust, no_run
//! use freya::{
//!     camera::*,
//!     prelude::*,
//! };
//!
//! fn app() -> impl IntoElement {
//!     let camera = use_camera(CameraConfig::default);
//!     CameraViewer::new(camera)
//! }
//! ```
//!
//! On macOS the system requires a one time authorization step before any
//! camera can be opened. Call [`init`] from `main` to request it.

pub mod camera;
pub mod camera_viewer;
pub(crate) mod capture;
pub mod use_camera;

pub use nokhwa;

/// Request access to the system cameras. Always `true` on Linux/Windows.
/// On macOS, blocks on the authorization prompt and returns whether access was granted;
/// call once from `main` before launching the app.
#[cfg(target_os = "macos")]
pub fn init() -> bool {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    nokhwa::nokhwa_initialize(move |granted| {
        let _ = tx.send(granted);
    });
    rx.recv().unwrap_or(false)
}

#[cfg(not(target_os = "macos"))]
pub fn init() -> bool {
    true
}

pub mod prelude {
    pub use crate::{
        camera::*,
        camera_viewer::*,
        use_camera::*,
    };
}
