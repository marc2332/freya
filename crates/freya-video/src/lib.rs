mod client;
mod player;
mod viewer;

pub use self::{
    client::{
        VideoClient,
        VideoEvent,
        VideoFrame,
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
