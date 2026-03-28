use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
        mpsc,
    },
    time::Duration,
};

use rodio::Source;

pub(crate) const AUDIO_SAMPLE_RATE: u32 = 44100;

/// If audio is this many seconds BEHIND video, skip the frame.
/// Used at startup and after seeks to discard pre-position audio frames.
pub(crate) const SYNC_BEHIND_SECS: f64 = 0.05;

pub(crate) struct AudioFrameData {
    pub samples: Vec<f32>,
    /// Presentation timestamp in seconds (from the audio stream's time_base).
    pub pts_secs: f64,
}

/// Rodio source backed by a channel with PTS-based A/V sync.
///
/// - Holds silence until `video_started` is set (first video frame shown).
/// - Skips audio frames that are too far behind the current video PTS.
pub(crate) struct ChannelSource {
    rx: mpsc::Receiver<AudioFrameData>,
    buffer: VecDeque<f32>,
    paused: Arc<AtomicBool>,
    volume: Arc<AtomicU32>,
    video_started: Arc<AtomicBool>,
    /// Current video frame PTS in seconds (f64 bits stored atomically).
    video_pts: Arc<AtomicU64>,
    /// Set to true to stop this source (used when a seek or new playback starts).
    abort: Arc<AtomicBool>,
}

impl ChannelSource {
    pub fn new(
        rx: mpsc::Receiver<AudioFrameData>,
        paused: Arc<AtomicBool>,
        volume: Arc<AtomicU32>,
        video_started: Arc<AtomicBool>,
        video_pts: Arc<AtomicU64>,
        abort: Arc<AtomicBool>,
    ) -> Self {
        Self {
            rx,
            buffer: VecDeque::new(),
            paused,
            volume,
            video_started,
            video_pts,
            abort,
        }
    }
}

impl Iterator for ChannelSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.abort.load(Ordering::Relaxed) {
            return None;
        }
        if !self.video_started.load(Ordering::Relaxed) {
            return Some(0.0);
        }
        if self.paused.load(Ordering::Relaxed) {
            return Some(0.0);
        }
        while self.buffer.is_empty() {
            let frame = match self.rx.try_recv() {
                Ok(f) => f,
                Err(mpsc::TryRecvError::Disconnected) => return None,
                Err(mpsc::TryRecvError::Empty) => return Some(0.0),
            };
            let v_pts = f64::from_bits(self.video_pts.load(Ordering::Relaxed));
            if frame.pts_secs < v_pts - SYNC_BEHIND_SECS {
                // Audio too far behind (seek/startup) — discard and try next frame
                continue;
            }
            self.buffer.extend(frame.samples);
        }
        let sample = self.buffer.pop_front()?;
        let vol = f32::from_bits(self.volume.load(Ordering::Relaxed));
        Some(sample * vol)
    }
}

impl Source for ChannelSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        2
    }
    fn sample_rate(&self) -> u32 {
        AUDIO_SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
