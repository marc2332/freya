//! Camera capture for Freya.
//!
//! - [`use_camera`](use_camera::use_camera): a hook that streams frames from a
//!   camera into reactive state.
//! - [`CameraViewer`](camera_viewer::CameraViewer): a component that renders
//!   the live feed.
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

/// Request access to the system cameras.
///
/// On Linux and Windows always returns `true`. On macOS it triggers the
/// authorization prompt, blocks until the user has answered, and returns
/// whether access was granted. Call it once from `main` before launching your
/// app; if it returns `false`, [`use_camera`](use_camera::use_camera) will
/// still run but the capture thread will fail to open any device.
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
