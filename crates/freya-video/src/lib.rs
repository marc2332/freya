//! Video playback for Freya, backed by ffmpeg and rodio.
//!
//! - [`use_video`]: a hook that decodes a video into reactive state and returns a
//!   [`VideoPlayer`] to control playback.
//!
//! Render the current frame yourself with the `image()` element from `player.frame()`.
//!
//! Call [`ensure_ffmpeg`] once before `launch()` to auto-download an ffmpeg binary.
//!
//! This crate is reexported in `freya::video`.
//!
//! # Example
//!
//! ```rust, no_run
//! use freya::{
//!     elements::image::image,
//!     prelude::*,
//!     video::*,
//! };
//!
//! fn app() -> impl IntoElement {
//!     let player = use_video(|| "video.mp4");
//!
//!     rect().maybe_child(player.frame().map(image))
//! }
//! ```

mod client;
mod player;

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
};

/// Download an ffmpeg binary if one isn't already available on `PATH`.
/// Call from `main` before `launch()` to opt in to auto-install.
pub fn ensure_ffmpeg() -> anyhow::Result<()> {
    ffmpeg_sidecar::download::auto_download()
}
