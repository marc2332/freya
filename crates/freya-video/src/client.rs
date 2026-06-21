use std::{
    io::{
        BufReader,
        Read as _,
        Write as _,
    },
    path::{
        Path,
        PathBuf,
    },
    process::{
        ChildStdin,
        ChildStdout,
    },
    rc::Rc,
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            AtomicU32,
            Ordering,
        },
    },
    time::{
        Duration,
        Instant,
    },
};

use async_io::Timer;
use ffmpeg_sidecar::{
    child::FfmpegChild,
    command::FfmpegCommand,
    event::{
        FfmpegEvent,
        OutputVideoFrame,
    },
};
use freya_core::{
    elements::image::ImageHolder,
    notify::ArcNotify,
    prelude::{
        Bytes,
        OwnedTaskHandle,
        provide_root_context,
        spawn,
        try_consume_root_context,
    },
};
use freya_engine::prelude::AlphaType;
use rodio::cpal::traits::{
    DeviceTrait,
    HostTrait,
};

/// Source of a video to decode.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct VideoSource(pub PathBuf);

impl From<PathBuf> for VideoSource {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl From<&Path> for VideoSource {
    fn from(path: &Path) -> Self {
        Self(path.to_path_buf())
    }
}

impl From<&str> for VideoSource {
    fn from(path: &str) -> Self {
        Self(PathBuf::from(path))
    }
}

impl From<String> for VideoSource {
    fn from(path: String) -> Self {
        Self(PathBuf::from(path))
    }
}

impl VideoSource {
    /// Base ffmpeg command for this source with an optional `-ss` seek.
    fn ffmpeg_command(&self, start_offset: Duration) -> FfmpegCommand {
        let mut cmd = FfmpegCommand::new();
        let start_secs = start_offset.as_secs_f32();
        if start_secs > 0.0 {
            cmd.args(["-ss", &start_secs.to_string()]);
        }
        cmd.input(self.0.to_string_lossy().as_ref());
        cmd
    }
}

/// Max decoded frames buffered ahead of the pacing loop.
const FRAME_BUFFER: usize = 2;

/// Max outgoing events buffered before the pacing loop blocks.
const EVENTS_BUFFER: usize = 2;

/// Audio format used when the output device's default config can't be queried.
const FALLBACK_AUDIO_CONFIG: (u32, u16) = (48_000, 2);

/// Event emitted by a [`VideoClient`].
#[derive(Clone)]
pub enum VideoEvent {
    Duration(Duration),
    Frame {
        frame: ImageHolder,
        position: Duration,
    },
    Ended,
    Errored,
}

/// Decoding pipeline for one video. Drop to stop.
pub struct VideoClient {
    events: async_channel::Receiver<VideoEvent>,
    paused: Arc<AtomicBool>,
    resumed: ArcNotify,
    volume: Arc<AtomicU32>,
    _task: OwnedTaskHandle,
}

impl VideoClient {
    /// Start decoding `source` at `start_offset`, optionally paused, at `volume`.
    pub fn new(
        source: VideoSource,
        start_offset: Duration,
        start_paused: bool,
        volume: f32,
    ) -> Self {
        let (sender, receiver) = async_channel::bounded(EVENTS_BUFFER);
        let paused = Arc::new(AtomicBool::new(start_paused));
        let resumed = ArcNotify::new();
        let volume = Arc::new(AtomicU32::new(volume.to_bits()));
        let task = spawn(Self::run(
            source,
            start_offset,
            paused.clone(),
            resumed.clone(),
            volume.clone(),
            sender,
        ))
        .owned();
        Self {
            events: receiver,
            paused,
            resumed,
            volume,
            _task: task,
        }
    }

    /// Stream of decoded frames and lifecycle events.
    pub fn events(&self) -> &async_channel::Receiver<VideoEvent> {
        &self.events
    }

    /// Pause playback.
    pub fn pause(&self) {
        self.paused.store(true, Ordering::Relaxed);
    }

    /// Resume playback, waking the pacing loop if it is waiting.
    pub fn play(&self) {
        self.paused.store(false, Ordering::Relaxed);
        self.resumed.notify();
    }

    /// Set the audio volume, where `1.0` is the original level.
    pub fn set_volume(&self, volume: f32) {
        self.volume.store(volume.to_bits(), Ordering::Relaxed);
    }

    /// Decode `source` and emit pacing-corrected frames into `events`.
    async fn run(
        source: VideoSource,
        start_offset: Duration,
        paused: Arc<AtomicBool>,
        resumed: ArcNotify,
        volume: Arc<AtomicU32>,
        events: async_channel::Sender<VideoEvent>,
    ) {
        let mut cmd = source.ffmpeg_command(start_offset);
        cmd.format("rawvideo").pix_fmt("rgba").pipe_stdout();
        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(err) => {
                tracing::error!("Failed to spawn ffmpeg: {err}");
                let _ = events.send(VideoEvent::Errored).await;
                return;
            }
        };
        let _quitter = child.take_stdin().map(FfmpegQuitter);

        let audio = AudioPlayback::start(
            &source,
            start_offset,
            f32::from_bits(volume.load(Ordering::Relaxed)),
        );
        if paused.load(Ordering::Relaxed)
            && let Some(audio) = audio.as_ref()
        {
            audio.sink.pause();
        }

        let (sender, receiver) = async_channel::bounded::<DecoderEvent>(FRAME_BUFFER);
        let decoder = blocking::unblock(move || Self::run_decoder(child, sender));

        let mut wall_start: Option<Instant> = None;
        let mut paused_for = Duration::ZERO;

        while let Ok(event) = receiver.recv().await {
            let frame = match event {
                DecoderEvent::Duration(duration) => {
                    let _ = events.send(VideoEvent::Duration(duration)).await;
                    continue;
                }
                DecoderEvent::Frame(frame) => frame,
            };

            // Show the first frame even when paused, so a seek reveals a preview.
            if wall_start.is_some() {
                paused_for += Self::wait_for_resume(&paused, &resumed, audio.as_ref()).await;
            }

            if let Some(audio) = audio.as_ref() {
                audio
                    .sink
                    .set_volume(f32::from_bits(volume.load(Ordering::Relaxed)));
            }

            let wall_start = *wall_start.get_or_insert_with(Instant::now);
            let frame_offset = Duration::from_secs_f32(frame.timestamp.max(0.0));
            let elapsed = wall_start.elapsed().saturating_sub(paused_for);
            if elapsed < frame_offset {
                Timer::after(frame_offset - elapsed).await;
            }

            let Some(frame) = Self::decode_frame(frame) else {
                tracing::warn!("Dropping frame: failed to decode raw RGBA into a Skia image");
                continue;
            };
            if events
                .send(VideoEvent::Frame {
                    frame,
                    position: start_offset + frame_offset,
                })
                .await
                .is_err()
            {
                tracing::warn!("Video event consumer dropped, stopping pacing loop");
                break;
            }
        }

        match decoder.await {
            Ok(()) => {
                let _ = events.send(VideoEvent::Ended).await;
            }
            Err(err) => {
                tracing::error!("Video decoder failed: {err}");
                let _ = events.send(VideoEvent::Errored).await;
            }
        }
    }

    /// Wrap a raw RGBA frame as a Skia raster image.
    fn decode_frame(frame: OutputVideoFrame) -> Option<ImageHolder> {
        ImageHolder::from_rgba(
            frame.width,
            frame.height,
            Bytes::from(frame.data),
            AlphaType::Unpremul,
        )
    }

    /// If paused, suspend audio and await a resume notification. Returns the paused-for delta.
    async fn wait_for_resume(
        paused: &AtomicBool,
        resumed: &ArcNotify,
        audio: Option<&AudioPlayback>,
    ) -> Duration {
        if !paused.load(Ordering::Relaxed) {
            return Duration::ZERO;
        }
        if let Some(audio) = audio {
            audio.sink.pause();
        }
        let pause_start = Instant::now();
        while paused.load(Ordering::Relaxed) {
            resumed.notified().await;
        }
        if let Some(audio) = audio {
            audio.sink.play();
        }
        pause_start.elapsed()
    }

    fn run_decoder(
        mut child: FfmpegChild,
        sender: async_channel::Sender<DecoderEvent>,
    ) -> anyhow::Result<()> {
        for event in child.iter()? {
            let item = match event {
                FfmpegEvent::ParsedDuration(d) if d.duration.is_finite() && d.duration >= 0.0 => {
                    DecoderEvent::Duration(Duration::from_secs_f64(d.duration))
                }
                FfmpegEvent::OutputFrame(frame) => DecoderEvent::Frame(frame),
                _ => continue,
            };
            if sender.send_blocking(item).is_err() {
                tracing::warn!("Decoder consumer dropped, stopping ffmpeg ingest");
                break;
            }
        }

        // Always reap the child to avoid a zombie process (ffmpeg-sidecar#72).
        let _ = child.kill();
        let _ = child.wait();

        Ok(())
    }
}

enum DecoderEvent {
    Duration(Duration),
    Frame(OutputVideoFrame),
}

/// Asks ffmpeg to exit gracefully when dropped.
struct FfmpegQuitter(ChildStdin);

impl Drop for FfmpegQuitter {
    fn drop(&mut self) {
        let _ = self.0.write_all(b"q\n");
        let _ = self.0.flush();
    }
}

/// PCM audio samples streamed from an ffmpeg process.
struct PcmSource {
    reader: BufReader<ChildStdout>,
    sample_rate: u32,
    channels: u16,
}

impl Iterator for PcmSource {
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).ok()?;
        Some(i16::from_le_bytes(buf))
    }
}

impl rodio::Source for PcmSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        self.channels
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// Audio side of a running playback.
struct AudioPlayback {
    _quitter: Option<FfmpegQuitter>,
    sink: rodio::Sink,
    _child: FfmpegChild,
}

impl AudioPlayback {
    /// Start an audio-only ffmpeg pipeline feeding into rodio at `volume`.
    fn start(source: &VideoSource, start_offset: Duration, volume: f32) -> Option<Self> {
        let handle = Self::handle()?;
        let (sample_rate, channels) = Self::output_config();
        let mut cmd = source.ffmpeg_command(start_offset);
        cmd.args([
            "-vn",
            "-f",
            "s16le",
            "-ar",
            &sample_rate.to_string(),
            "-ac",
            &channels.to_string(),
        ])
        .pipe_stdout();
        let mut child = cmd
            .spawn()
            .map_err(|err| tracing::warn!("Failed to spawn audio ffmpeg: {err}"))
            .ok()?;
        let stdout = child.take_stdout()?;
        let quitter = child.take_stdin().map(FfmpegQuitter);
        let sink = rodio::Sink::try_new(&handle)
            .map_err(|err| tracing::warn!("Failed to create audio sink: {err}"))
            .ok()?;
        sink.set_volume(volume);
        sink.append(PcmSource {
            reader: BufReader::new(stdout),
            sample_rate,
            channels,
        });
        Some(Self {
            _quitter: quitter,
            sink,
            _child: child,
        })
    }

    /// Shared audio output handle, created once and cached in the root context.
    fn handle() -> Option<Rc<rodio::OutputStreamHandle>> {
        if let Some(handle) = try_consume_root_context::<Rc<rodio::OutputStreamHandle>>() {
            return Some(handle);
        }

        let (stream, handle) = rodio::OutputStream::try_default()
            .map_err(|err| tracing::info!("No audio output device: {err}"))
            .ok()?;
        let stream = Rc::new(stream);
        let handle = Rc::new(handle);

        provide_root_context(stream);
        provide_root_context(handle.clone());

        Some(handle)
    }

    /// Default output device's sample rate and channels, to avoid a second resample.
    fn output_config() -> (u32, u16) {
        rodio::cpal::default_host()
            .default_output_device()
            .and_then(|device| device.default_output_config().ok())
            .map(|config| (config.sample_rate().0, config.channels()))
            .unwrap_or(FALLBACK_AUDIO_CONFIG)
    }
}
