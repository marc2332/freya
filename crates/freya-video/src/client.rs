use std::{
    io::{
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
use freya_core::prelude::{
    OwnedTaskHandle,
    ScopeId,
    provide_context_for_scope_id,
    spawn,
    try_consume_root_context,
};
use freya_engine::prelude::{
    AlphaType,
    ColorType,
    Data,
    ISize,
    ImageInfo,
    SkImage,
    raster_from_data,
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
    /// Common ffmpeg command for this source: input + optional `-ss` seek.
    fn ffmpeg_command(&self, start_offset: Duration) -> FfmpegCommand {
        let mut cmd = FfmpegCommand::new();
        // `-ss` before `-i` = fast keyframe-aligned input seek; output timestamps
        // reset to 0, which is what the pacing loop expects.
        let start_secs = start_offset.as_secs_f32();
        if start_secs > 0.0 {
            cmd.args(["-ss", &start_secs.to_string()]);
        }
        cmd.input(self.0.to_string_lossy().as_ref());
        cmd
    }
}

/// Single decoded frame, backed by a Skia image.
#[derive(Clone)]
pub struct VideoFrame {
    pub(crate) image: SkImage,
}

impl VideoFrame {
    pub fn image(&self) -> &SkImage {
        &self.image
    }

    /// Wrap a raw RGBA frame as a Skia raster image.
    fn from_raw(frame: &OutputVideoFrame) -> Option<Self> {
        let row_bytes = frame.width.checked_mul(4)? as usize;
        let info = ImageInfo::new(
            ISize::new(frame.width as i32, frame.height as i32),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let image = raster_from_data(&info, Data::new_copy(&frame.data), row_bytes)?;
        Some(Self { image })
    }
}

impl PartialEq for VideoFrame {
    fn eq(&self, other: &Self) -> bool {
        self.image.unique_id() == other.image.unique_id()
    }
}

/// Max decoded frames buffered ahead of the pacing loop.
const FRAME_BUFFER: usize = 2;

/// Max outgoing events buffered before the pacing loop blocks.
const EVENTS_BUFFER: usize = 2;

const AUDIO_SAMPLE_RATE: u32 = 48_000;
const AUDIO_CHANNELS: u16 = 2;

/// Polling interval while paused: trades resume latency for idle CPU.
const PAUSE_POLL: Duration = Duration::from_millis(32);

/// Event emitted by a [`VideoClient`].
#[derive(Clone)]
pub enum VideoEvent {
    Duration(Duration),
    Frame {
        frame: VideoFrame,
        position: Duration,
    },
    Ended,
    Errored,
}

/// Decoding pipeline for one video. Drop to stop.
pub struct VideoClient {
    events: async_channel::Receiver<VideoEvent>,
    paused: Arc<AtomicBool>,
    _task: OwnedTaskHandle,
}

impl VideoClient {
    /// Start decoding `source` at `start_offset`.
    pub fn new(source: VideoSource, start_offset: Duration) -> Self {
        let (sender, receiver) = async_channel::bounded(EVENTS_BUFFER);
        let paused = Arc::new(AtomicBool::new(false));
        let task = spawn(Self::run(source, start_offset, paused.clone(), sender)).owned();
        Self {
            events: receiver,
            paused,
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

    /// Resume playback.
    pub fn play(&self) {
        self.paused.store(false, Ordering::Relaxed);
    }

    /// Whether playback is currently paused.
    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    /// Decode `source` and emit pacing-corrected frames into `events`.
    async fn run(
        source: VideoSource,
        start_offset: Duration,
        paused: Arc<AtomicBool>,
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

        let audio = AudioPlayback::start(&source, start_offset);

        let (sender, receiver) = async_channel::bounded::<DecoderEvent>(FRAME_BUFFER);
        let decoder = blocking::unblock(move || run_decoder(child, sender));

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

            // Bank pause time so the wall-clock pacing stays correct on resume.
            paused_for += wait_for_resume(&paused, audio.as_ref()).await;

            let wall_start = *wall_start.get_or_insert_with(Instant::now);
            let frame_offset = Duration::from_secs_f32(frame.timestamp.max(0.0));
            let elapsed = wall_start.elapsed().saturating_sub(paused_for);
            if elapsed < frame_offset {
                Timer::after(frame_offset - elapsed).await;
            }

            let Some(frame) = VideoFrame::from_raw(&frame) else {
                tracing::warn!("Dropping frame: failed to wrap raw RGBA as Skia image");
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
}

/// Shared audio output handle.
fn audio_handle() -> Option<Rc<rodio::OutputStreamHandle>> {
    if let Some(handle) = try_consume_root_context::<Rc<rodio::OutputStreamHandle>>() {
        return Some(handle);
    }

    let (stream, handle) = rodio::OutputStream::try_default()
        .map_err(|err| tracing::info!("No audio output device: {err}"))
        .ok()?;
    let stream = Rc::new(stream);
    let handle = Rc::new(handle);

    provide_context_for_scope_id(stream, ScopeId::ROOT);
    provide_context_for_scope_id(handle.clone(), ScopeId::ROOT);

    Some(handle)
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
struct PcmSource(ChildStdout);

impl Iterator for PcmSource {
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
        let mut buf = [0u8; 2];
        self.0.read_exact(&mut buf).ok()?;
        Some(i16::from_le_bytes(buf))
    }
}

impl rodio::Source for PcmSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        AUDIO_CHANNELS
    }
    fn sample_rate(&self) -> u32 {
        AUDIO_SAMPLE_RATE
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
    /// Start an audio-only ffmpeg pipeline feeding into rodio.
    fn start(source: &VideoSource, start_offset: Duration) -> Option<Self> {
        let handle = audio_handle()?;
        let mut cmd = source.ffmpeg_command(start_offset);
        cmd.args([
            "-vn",
            "-f",
            "s16le",
            "-ar",
            &AUDIO_SAMPLE_RATE.to_string(),
            "-ac",
            &AUDIO_CHANNELS.to_string(),
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
        sink.append(PcmSource(stdout));
        Some(Self {
            _quitter: quitter,
            sink,
            _child: child,
        })
    }
}

/// If paused, suspend audio and spin until resumed. Returns the paused-for delta.
async fn wait_for_resume(paused: &AtomicBool, audio: Option<&AudioPlayback>) -> Duration {
    if !paused.load(Ordering::Relaxed) {
        return Duration::ZERO;
    }
    if let Some(audio) = audio {
        audio.sink.pause();
    }
    let pause_start = Instant::now();
    while paused.load(Ordering::Relaxed) {
        Timer::after(PAUSE_POLL).await;
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
        // Parks the thread when the bounded channel is full, which backpressures
        // ffmpeg via its stdout pipe. Err = receiver dropped (decode cancelled).
        if sender.send_blocking(item).is_err() {
            tracing::warn!("Decoder consumer dropped, stopping ffmpeg ingest");
            break;
        }
    }

    // Reap regardless of how we exited the iter (ffmpeg-sidecar#72).
    let _ = child.kill();
    let _ = child.wait();

    Ok(())
}
