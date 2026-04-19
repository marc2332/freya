use std::time::Duration;

use async_io::Timer;
use freya_core::prelude::*;

use crate::{
    VideoClient,
    VideoEvent,
    VideoFrame,
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

/// Wait window before a seek actually spawns ffmpeg.
const SEEK_DEBOUNCE: Duration = Duration::from_millis(150);

/// Reactive handle to a video decoding pipeline.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VideoPlayer {
    frame: State<Option<VideoFrame>>,
    playback: State<PlaybackState>,
    forwarder: State<Option<OwnedTaskHandle>>,
    source: State<Option<VideoSource>>,
    position: State<Duration>,
    duration: State<Option<Duration>>,
    client: State<Option<VideoClient>>,
}

impl VideoPlayer {
    /// Allocate the reactive slots in the current scope.
    pub fn create() -> Self {
        Self {
            frame: State::create(None),
            playback: State::create(PlaybackState::Idle),
            forwarder: State::create(None),
            source: State::create(None),
            position: State::create(Duration::ZERO),
            duration: State::create(None),
            client: State::create(None),
        }
    }

    /// Latest decoded frame, if any.
    pub fn frame(&self) -> Option<VideoFrame> {
        self.frame.read().as_ref().cloned()
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

    /// Playback progress in `0.0..=100.0`.
    pub fn progress(&self) -> f64 {
        let position = *self.position.read();
        let Some(duration) = *self.duration.read() else {
            return 0.0;
        };
        if duration.is_zero() {
            return 0.0;
        }
        (position.as_secs_f64() / duration.as_secs_f64() * 100.0).clamp(0.0, 100.0)
    }

    /// Stop playback and reset to the beginning.
    pub fn stop(&mut self) {
        self.forwarder.set(None);
        self.client.set(None);
        self.frame.set(None);
        self.playback.set(PlaybackState::Idle);
        self.position.set(Duration::ZERO);
    }

    /// Resume playback.
    pub fn play(&mut self) {
        if (self.playback)() == PlaybackState::Paused {
            self.playback.set(PlaybackState::Playing);
            if let Some(client) = self.client.peek().as_ref() {
                client.play();
            }
        }
    }

    /// Pause playback.
    pub fn pause(&mut self) {
        if (self.playback)() == PlaybackState::Playing {
            self.playback.set(PlaybackState::Paused);
            if let Some(client) = self.client.peek().as_ref() {
                client.pause();
            }
        }
    }

    /// Toggle play/pause, or restart from the beginning when ended.
    pub fn toggle(&mut self) {
        match (self.playback)() {
            PlaybackState::Playing => self.pause(),
            PlaybackState::Paused => self.play(),
            PlaybackState::Ended => self.seek(Duration::ZERO),
            _ => {}
        }
    }

    /// Seek to `position`.
    pub fn seek(&mut self, position: Duration) {
        self.position.set(position);
        self.client.set(None);
        self.playback.set(PlaybackState::Loading);

        let Some(source) = self.source.peek().as_ref().cloned() else {
            self.forwarder.set(None);
            return;
        };
        let player = *self;
        let handle = spawn(async move {
            Timer::after(SEEK_DEBOUNCE).await;
            run_playback(source, position, player).await;
        })
        .owned();
        self.forwarder.set(Some(handle));
    }
}

/// Create a [`VideoPlayer`] and start playing `init()`.
pub fn use_video(init: impl FnOnce() -> VideoSource + 'static) -> VideoPlayer {
    use_hook(move || {
        let source = init();
        let mut player = VideoPlayer::create();
        player.source.set(Some(source.clone()));
        player.playback.set(PlaybackState::Loading);
        let handle = spawn(run_playback(source, Duration::ZERO, player)).owned();
        player.forwarder.set(Some(handle));
        player
    })
}

/// Play `source` from `start_offset` into `player`.
async fn run_playback(source: VideoSource, start_offset: Duration, mut player: VideoPlayer) {
    let client = VideoClient::new(source, start_offset);
    let events = client.events().clone();
    player.client.set(Some(client));

    while let Ok(event) = events.recv().await {
        match event {
            VideoEvent::Duration(duration) => {
                player.duration.set(Some(duration));
            }
            VideoEvent::Frame { frame, position } => {
                player.frame.set(Some(frame));
                player.position.set(position);
                if (player.playback)() == PlaybackState::Loading {
                    player.playback.set(PlaybackState::Playing);
                }
            }
            VideoEvent::Ended => {
                player.playback.set(PlaybackState::Ended);
                break;
            }
            VideoEvent::Errored => {
                player.playback.set(PlaybackState::Errored);
                break;
            }
        }
    }
}
