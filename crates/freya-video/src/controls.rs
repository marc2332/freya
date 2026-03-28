use std::rc::Rc;

/// Current playback state and actions passed to a custom controls builder.
///
/// # Example
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use std::path::PathBuf;
/// # use std::rc::Rc;
/// fn app() -> impl IntoElement {
///     VideoPlayer::new(PathBuf::from("./video.mp4"))
///         .custom_controls(Rc::new(|c: VideoControls| {
///             rect()
///                 .horizontal()
///                 .child(format!("{:.0}s / {:.0}s", c.current_secs, c.total_secs))
///                 .into_element()
///         }))
/// }
/// ```
pub struct VideoControls {
    /// Whether the video is currently playing.
    pub is_playing: bool,
    /// Whether the video is currently paused.
    pub is_paused: bool,
    /// Whether the video has finished playing.
    pub is_finished: bool,
    /// Playback progress as a fraction (0.0..=1.0).
    pub progress: f64,
    /// Current playback position in seconds.
    pub current_secs: f64,
    /// Total video duration in seconds.
    pub total_secs: f64,
    /// Current volume level (0.0..=1.0).
    pub volume: f64,
    /// Toggle between play and pause. If the video has finished, restarts from the beginning.
    pub toggle_play: Rc<dyn Fn()>,
    /// Seek to a position given as a fraction of total duration (0.0..=1.0).
    pub seek: Rc<dyn Fn(f64)>,
    /// Set the playback volume (0.0..=1.0).
    pub set_volume: Rc<dyn Fn(f64)>,
}
