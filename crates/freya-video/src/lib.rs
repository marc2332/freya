//! Video playback for Freya, backed by ffmpeg and rodio.
//!
//! - [`use_video`]: a hook that decodes a video into reactive state and returns a
//!   [`VideoPlayer`] to control playback.
//! - [`VideoViewer`]: a component that renders the current frame.
//!
//! Call [`ensure_ffmpeg`] once before `launch()` to auto-download an ffmpeg binary.
//!
//! This crate is reexported in `freya::video`.
//!
//! # Example
//!
//! ```rust, no_run
//! use freya::{
//!     prelude::*,
//!     video::*,
//! };
//!
//! fn app() -> impl IntoElement {
//!     let player = use_video(|| "video.mp4");
//!
//!     VideoViewer::new(player)
//! }
//! ```

mod client;
mod player;
mod viewer;

pub use self::{
    client::{
        VideoClient,
        VideoEvent,
        VideoSource,
    },
    player::{
        PlaybackState,
        VideoPlayer,
        use_video,
    },
    viewer::VideoViewer,
};

/// Download an ffmpeg binary if one isn't already available on `PATH`.
/// Call from `main` before `launch()` to opt in to auto-install.
pub fn ensure_ffmpeg() -> anyhow::Result<()> {
    ffmpeg_sidecar::download::auto_download()
}
