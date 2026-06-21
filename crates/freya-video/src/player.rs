use std::time::Duration;

use async_io::Timer;
use freya_core::{
    elements::image::ImageHolder,
    prelude::*,
};

use crate::{
    VideoClient,
    VideoEvent,
    VideoSource,
};

/// Current playback state of a [`VideoPlayer`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PlaybackState {
    #[default]
    Idle,
    Loading,
    Playing,
    Paused,
    Ended,
    Errored,
}

/// Reactive handle to a video decoding pipeline.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VideoPlayer {
    frame: State<Option<ImageHolder>>,
    playback: State<PlaybackState>,
    forwarder: State<Option<OwnedTaskHandle>>,
    source: State<VideoSource>,
    position: State<Duration>,
    duration: State<Option<Duration>>,
    volume: State<f32>,
    client: State<Option<VideoClient>>,
}

impl VideoPlayer {
    pub fn create(source: VideoSource) -> Self {
        Self {
            frame: State::create(None),
            playback: State::create(PlaybackState::default()),
            forwarder: State::create(None),
            source: State::create(source),
            position: State::create(Duration::ZERO),
            duration: State::create(None),
            volume: State::create(1.0),
            client: State::create(None),
        }
    }

    /// Latest decoded frame, if any.
    pub fn frame(&self) -> Option<ImageHolder> {
        self.frame.read().clone()
    }

    /// Current [`PlaybackState`].
    pub fn state(&self) -> PlaybackState {
        *self.playback.read()
    }

    /// Current playback position.
    pub fn position(&self) -> Duration {
        *self.position.read()
    }

    /// Total duration, if known.
    pub fn duration(&self) -> Option<Duration> {
        *self.duration.read()
    }

    /// Audio volume in `0.0..=1.0`, where `1.0` is the original level.
    pub fn volume(&self) -> f32 {
        *self.volume.read()
    }

    /// Set the audio volume, clamped to `0.0..=1.0`.
    pub fn set_volume(&mut self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        self.volume.set(volume);
        if let Some(client) = self.client.peek().as_ref() {
            client.set_volume(volume);
        }
    }

    /// Playback progress in `0.0..=100.0`.
    pub fn progress(&self) -> f64 {
        let Some(duration) = *self.duration.read() else {
            return 0.0;
        };
        if duration.is_zero() {
            return 0.0;
        }
        (self.position().as_secs_f64() / duration.as_secs_f64() * 100.0).clamp(0.0, 100.0)
    }

    /// Stop playback and reset to the beginning.
    pub fn stop(&mut self) {
        self.forwarder.set(None);
        self.client.set(None);
        self.frame.set(None);
        self.playback.set(PlaybackState::Idle);
        self.position.set(Duration::ZERO);
    }

    /// Resume playback, or start from the current position when idle.
    pub fn play(&mut self) {
        match self.state() {
            PlaybackState::Paused => {
                self.playback.set(PlaybackState::Playing);
                if let Some(client) = self.client.peek().as_ref() {
                    client.play();
                }
            }
            PlaybackState::Idle => self.seek(self.position(), Duration::ZERO),
            _ => {}
        }
    }

    /// Pause playback.
    pub fn pause(&mut self) {
        if self.state() == PlaybackState::Playing {
            self.playback.set(PlaybackState::Paused);
            if let Some(client) = self.client.peek().as_ref() {
                client.pause();
            }
        }
    }

    /// Toggle play/pause, or restart from the beginning when ended.
    pub fn toggle(&mut self) {
        match self.state() {
            PlaybackState::Playing => self.pause(),
            PlaybackState::Paused => self.play(),
            PlaybackState::Ended => self.seek(Duration::ZERO, Duration::ZERO),
            _ => {}
        }
    }

    /// Replace the video source and start playing it from the beginning.
    pub fn set_source(&mut self, source: impl Into<VideoSource>) {
        self.source.set(source.into());
        self.seek(Duration::ZERO, Duration::ZERO);
    }

    /// Seek to `position` after `debounce`, where a newer seek within the wait replaces it.
    pub fn seek(&mut self, position: Duration, debounce: Duration) {
        let start_paused = self.state() == PlaybackState::Paused;
        self.position.set(position);
        self.client.set(None);
        self.playback.set(PlaybackState::Loading);

        let source = self.source.peek().clone();
        let player = *self;
        let handle = spawn(async move {
            Timer::after(debounce).await;
            player.run(source, position, start_paused).await;
        })
        .owned();
        self.forwarder.set(Some(handle));
    }

    /// Drive this player from a [`VideoClient`] decoding `source`.
    async fn run(mut self, source: VideoSource, start_offset: Duration, start_paused: bool) {
        let client = VideoClient::new(source, start_offset, start_paused, *self.volume.peek());
        let events = client.events().clone();
        self.client.set(Some(client));

        while let Ok(event) = events.recv().await {
            match event {
                VideoEvent::Duration(duration) => {
                    self.duration.set(Some(duration));
                }
                VideoEvent::Frame { frame, position } => {
                    self.frame.set(Some(frame));
                    self.position.set(position);
                    if self.state() == PlaybackState::Loading {
                        self.playback.set(if start_paused {
                            PlaybackState::Paused
                        } else {
                            PlaybackState::Playing
                        });
                    }
                }
                VideoEvent::Ended => {
                    self.playback.set(PlaybackState::Ended);
                    break;
                }
                VideoEvent::Errored => {
                    tracing::warn!("Video stream errored");
                    self.playback.set(PlaybackState::Errored);
                    break;
                }
            }
        }
    }
}

/// Create a [`VideoPlayer`] and start playing `video_source()`.
///
/// # Example
///
/// ```rust, no_run
/// use freya::{
///     elements::image::image,
///     prelude::*,
///     video::*,
/// };
///
/// fn app() -> impl IntoElement {
///     let player = use_video(|| "video.mp4");
///
///     rect().maybe_child(player.frame().map(image))
/// }
/// ```
pub fn use_video<Source: Into<VideoSource>>(
    video_source: impl FnOnce() -> Source + 'static,
) -> VideoPlayer {
    use_hook(move || {
        let mut player = VideoPlayer::create(video_source().into());
        player.play();
        player
    })
}
