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
        mpsc::sync_channel,
    },
    thread::{
        Builder,
        park,
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
    event::FfmpegEvent,
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
pub enum VideoSource {
    Path(PathBuf),
}

impl From<PathBuf> for VideoSource {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl From<&Path> for VideoSource {
    fn from(path: &Path) -> Self {
        Self::Path(path.to_path_buf())
    }
}

impl From<&str> for VideoSource {
    fn from(path: &str) -> Self {
        Self::Path(PathBuf::from(path))
    }
}

impl From<String> for VideoSource {
    fn from(path: String) -> Self {
        Self::Path(PathBuf::from(path))
    }
}

/// Single decoded frame, backed by a Skia image.
#[derive(Clone)]
pub struct VideoFrame {
    image: SkImage,
}

impl VideoFrame {
    pub fn image(&self) -> &SkImage {
        &self.image
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
        let task = spawn(run_client(
            source,
            start_offset,
            Arc::clone(&paused),
            sender,
        ))
        .owned();
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
}

/// Shared audio output handle. `None` if the host has no audio device.
fn audio_handle() -> Option<Rc<rodio::OutputStreamHandle>> {
    if let Some(handle) = try_consume_root_context::<Rc<rodio::OutputStreamHandle>>() {
        return Some(handle);
    }

    let (sender, receiver) = sync_channel::<Option<rodio::OutputStreamHandle>>(0);
    Builder::new()
        .name("freya-audio".into())
        .spawn(move || match rodio::OutputStream::try_default() {
            Ok((stream, handle)) => {
                if sender.send(Some(handle)).is_ok() {
                    let _keepalive = stream;
                    park();
                }
            }
            Err(err) => {
                tracing::info!(%err, "no audio output device");
                let _ = sender.send(None);
            }
        })
        .ok()?;

    let handle = Rc::new(receiver.recv().ok().flatten()?);
    provide_context_for_scope_id(handle.clone(), ScopeId::ROOT);
    Some(handle)
}

struct RawFrame {
    width: u32,
    height: u32,
    timestamp: f32,
    data: Vec<u8>,
}

enum DecoderEvent {
    Duration(f64),
    Frame(RawFrame),
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
    reader: ChildStdout,
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

async fn run_client(
    source: VideoSource,
    start_offset: Duration,
    paused: Arc<AtomicBool>,
    events: async_channel::Sender<VideoEvent>,
) {
    let mut child = match spawn_ffmpeg(&source, start_offset) {
        Ok(child) => child,
        Err(err) => {
            tracing::error!(%err, "failed to spawn ffmpeg");
            let _ = events.send(VideoEvent::Errored).await;
            return;
        }
    };
    let _quitter = child.take_stdin().map(FfmpegQuitter);

    let audio = start_audio(&source, start_offset);

    let (sender, receiver) = async_channel::bounded::<DecoderEvent>(FRAME_BUFFER);
    let decoder = blocking::unblock(move || decode(child, sender));

    let mut wall_start: Option<Instant> = None;
    let mut paused_for = Duration::ZERO;

    while let Ok(event) = receiver.recv().await {
        let raw = match event {
            DecoderEvent::Duration(seconds) if seconds.is_finite() && seconds >= 0.0 => {
                let _ = events
                    .send(VideoEvent::Duration(Duration::from_secs_f64(seconds)))
                    .await;
                continue;
            }
            DecoderEvent::Duration(_) => continue,
            DecoderEvent::Frame(raw) => raw,
        };

        // While paused, bank the elapsed time so resume pacing stays correct.
        if paused.load(Ordering::Relaxed) {
            if let Some(audio) = audio.as_ref() {
                audio.sink.pause();
            }
            let pause_start = Instant::now();
            while paused.load(Ordering::Relaxed) {
                Timer::after(Duration::from_millis(32)).await;
            }
            paused_for += pause_start.elapsed();
            if let Some(audio) = audio.as_ref() {
                audio.sink.play();
            }
        }

        let wall_start = *wall_start.get_or_insert_with(Instant::now);
        let frame_offset = Duration::from_secs_f32(raw.timestamp.max(0.0));
        let elapsed = wall_start.elapsed().saturating_sub(paused_for);
        if elapsed < frame_offset {
            Timer::after(frame_offset - elapsed).await;
        }

        let Some(image) = raw_frame_to_sk_image(&raw) else {
            continue;
        };
        if events
            .send(VideoEvent::Frame {
                frame: VideoFrame { image },
                position: start_offset + frame_offset,
            })
            .await
            .is_err()
        {
            break;
        }
    }

    match decoder.await {
        Ok(()) => {
            let _ = events.send(VideoEvent::Ended).await;
        }
        Err(err) => {
            tracing::error!(%err, "video decoder failed");
            let _ = events.send(VideoEvent::Errored).await;
        }
    }
}

fn raw_frame_to_sk_image(raw: &RawFrame) -> Option<SkImage> {
    let row_bytes = raw.width.checked_mul(4)? as usize;
    let info = ImageInfo::new(
        ISize::new(raw.width as i32, raw.height as i32),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    raster_from_data(&info, Data::new_copy(&raw.data), row_bytes)
}

fn spawn_ffmpeg(source: &VideoSource, start_offset: Duration) -> anyhow::Result<FfmpegChild> {
    let VideoSource::Path(path) = source;
    let mut cmd = FfmpegCommand::new();

    // `-ss` before `-i` = fast keyframe-aligned input seek; output timestamps
    // reset to 0, which is what the pacing loop expects.
    let start_secs = start_offset.as_secs_f32();
    if start_secs > 0.0 {
        cmd.args(["-ss", &start_secs.to_string()]);
    }
    cmd.input(path.to_string_lossy().as_ref())
        .format("rawvideo")
        .pix_fmt("rgba")
        .pipe_stdout();

    Ok(cmd.spawn()?)
}

/// Start audio playback. `None` if audio can't be started.
fn start_audio(source: &VideoSource, start_offset: Duration) -> Option<AudioPlayback> {
    let handle = audio_handle()?;
    let mut child = spawn_audio_ffmpeg(source, start_offset)
        .map_err(|err| tracing::warn!(%err, "failed to spawn audio ffmpeg"))
        .ok()?;
    let stdout = child.take_stdout()?;
    let quitter = child.take_stdin().map(FfmpegQuitter);
    let sink = rodio::Sink::try_new(&handle)
        .map_err(|err| tracing::warn!(%err, "failed to create audio sink"))
        .ok()?;
    sink.append(PcmSource { reader: stdout });
    Some(AudioPlayback {
        _quitter: quitter,
        sink,
        _child: child,
    })
}

fn spawn_audio_ffmpeg(source: &VideoSource, start_offset: Duration) -> anyhow::Result<FfmpegChild> {
    let VideoSource::Path(path) = source;
    let mut cmd = FfmpegCommand::new();

    let start_secs = start_offset.as_secs_f32();
    if start_secs > 0.0 {
        cmd.args(["-ss", &start_secs.to_string()]);
    }
    cmd.input(path.to_string_lossy().as_ref())
        .args([
            "-vn",
            "-f",
            "s16le",
            "-ar",
            &AUDIO_SAMPLE_RATE.to_string(),
            "-ac",
            &AUDIO_CHANNELS.to_string(),
        ])
        .pipe_stdout();

    Ok(cmd.spawn()?)
}

fn decode(
    mut child: FfmpegChild,
    sender: async_channel::Sender<DecoderEvent>,
) -> anyhow::Result<()> {
    for event in child.iter()? {
        let message = match event {
            FfmpegEvent::ParsedDuration(duration) => DecoderEvent::Duration(duration.duration),
            FfmpegEvent::OutputFrame(frame) => DecoderEvent::Frame(RawFrame {
                width: frame.width,
                height: frame.height,
                timestamp: frame.timestamp,
                data: frame.data,
            }),
            _ => continue,
        };
        // Parks the thread when the bounded channel is full, which backpressures
        // ffmpeg via its stdout pipe. Err = receiver dropped (decode cancelled).
        if sender.send_blocking(message).is_err() {
            break;
        }
    }

    // Reap regardless of how we exited the iter (ffmpeg-sidecar#72).
    let _ = child.kill();
    let _ = child.wait();

    Ok(())
}
