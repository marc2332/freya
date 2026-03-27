use std::{
    any::Any,
    borrow::Cow,
    collections::VecDeque,
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc,
        Mutex,
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
        mpsc,
    },
    time::Duration,
};

use anyhow::Context as AnyhowContext;
use async_io::Timer;
use blocking::unblock;
use ffmpeg_next as ffmpeg;
use ffmpeg::{
    ChannelLayout,
    format::{Pixel, sample::{self, Sample}},
    frame::Audio as AudioFrame,
    media::Type as MediaType,
    software::{
        resampling::context::Context as ResamplerCtx,
        scaling::{context::Context as ScalerContext, flag::Flags as ScalerFlags},
    },
};
use rodio::{OutputStream, Sink, Source};
use freya_core::{
    integration::*,
    prelude::*,
};
use freya_engine::prelude::{
    AlphaType,
    ClipOp,
    Color,
    ColorType,
    Data,
    FilterMode,
    ISize,
    ImageInfo,
    MipmapMode,
    Paint,
    PathBuilder,
    PaintStyle,
    SamplingOptions,
    SkImage,
    SkRect,
    raster_from_data,
};
use torin::{
    geometry::Area,
    size::Size,
};

use crate::loader::CircularLoader;

const AUDIO_SAMPLE_RATE: u32 = 44100;

/// If audio is this many seconds AHEAD of video, hold the frame and return silence.
/// Kept generous (300ms) so normal playback variance never triggers a hold.
const SYNC_LEAD_SECS: f64 = 0.3;
/// If audio is this many seconds BEHIND video, skip the frame.
/// Used at startup and after seeks to discard pre-position audio frames.
const SYNC_BEHIND_SECS: f64 = 0.05;

struct AudioFrameData {
    samples: Vec<f32>,
    /// Presentation timestamp in seconds (from the audio stream's time_base).
    pts_secs: f64,
}

/// Rodio source backed by a channel with PTS-based A/V sync.
///
/// - Holds silence until `video_started` is set (first video frame shown).
/// - Skips audio frames that are too far behind the current video PTS.
/// - Holds frames (returns silence) when audio is too far ahead of video.
struct ChannelSource {
    rx: mpsc::Receiver<AudioFrameData>,
    buffer: VecDeque<f32>,
    paused: Arc<AtomicBool>,
    volume: Arc<AtomicU32>,
    video_started: Arc<AtomicBool>,
    /// Current video frame PTS in seconds (f64 bits stored atomically).
    video_pts: Arc<AtomicU64>,
    /// Frame held back because audio was too far ahead of video.
    pending: Option<AudioFrameData>,
}

impl ChannelSource {
    fn new(
        rx: mpsc::Receiver<AudioFrameData>,
        paused: Arc<AtomicBool>,
        volume: Arc<AtomicU32>,
        video_started: Arc<AtomicBool>,
        video_pts: Arc<AtomicU64>,
    ) -> Self {
        Self {
            rx,
            buffer: VecDeque::new(),
            paused,
            volume,
            video_started,
            video_pts,
            pending: None,
        }
    }
}

impl Iterator for ChannelSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if !self.video_started.load(Ordering::Relaxed) {
            return Some(0.0);
        }
        if self.paused.load(Ordering::Relaxed) {
            while self.rx.try_recv().is_ok() {}
            self.pending = None;
            self.buffer.clear();
            return Some(0.0);
        }
        while self.buffer.is_empty() {
            let frame = match self.pending.take() {
                Some(f) => f,
                None => match self.rx.try_recv() {
                    Ok(f) => f,
                    Err(mpsc::TryRecvError::Disconnected) => return None,
                    Err(mpsc::TryRecvError::Empty) => return Some(0.0),
                },
            };
            let v_pts = f64::from_bits(self.video_pts.load(Ordering::Relaxed));
            if frame.pts_secs > v_pts + SYNC_LEAD_SECS {
                // Audio too far ahead — hold frame, return silence until video catches up
                self.pending = Some(frame);
                return Some(0.0);
            } else if frame.pts_secs < v_pts - SYNC_BEHIND_SECS {
                // Audio too far behind — discard frame and try the next one
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
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 2 }
    fn sample_rate(&self) -> u32 { AUDIO_SAMPLE_RATE }
    fn total_duration(&self) -> Option<Duration> { None }
}

/// Raw frame data sent from the decoder thread to the async task.
struct VideoFrameData {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
    pts_secs: f64,
    total_secs: f64,
    frame_duration: Duration,
}

/// Source for a [`VideoPlayer`].
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// # use std::path::PathBuf;
/// let source: VideoSource = PathBuf::from("./my_video.mp4").into();
/// ```
#[derive(Clone, PartialEq)]
pub enum VideoSource {
    /// Load a video from a file path.
    Path(PathBuf),
}

impl From<PathBuf> for VideoSource {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl From<&str> for VideoSource {
    fn from(s: &str) -> Self {
        Self::Path(PathBuf::from(s))
    }
}

impl From<String> for VideoSource {
    fn from(s: String) -> Self {
        Self::Path(PathBuf::from(s))
    }
}

#[derive(PartialEq)]
enum VideoStatus {
    Loading,
    Playing,
    Paused,
    Finished,
    Error(String),
}

/// Decodes only the video stream. Runs in its own OS thread.
fn decode_video_thread(
    path: PathBuf,
    start_secs: f64,
    tx_video: mpsc::SyncSender<VideoFrameData>,
) -> anyhow::Result<()> {
    ffmpeg::init()?;

    let mut ictx = ffmpeg::format::input(&path)?;

    if start_secs > 0.0 {
        let target_pts = (start_secs * ffmpeg::ffi::AV_TIME_BASE as f64) as i64;
        ictx.seek(target_pts, target_pts..).ok();
    }

    let total_secs = if ictx.duration() > 0 {
        ictx.duration() as f64 / ffmpeg::ffi::AV_TIME_BASE as f64
    } else {
        0.0
    };

    let input = ictx
        .streams()
        .best(MediaType::Video)
        .context("No video stream found")?;
    let video_stream_index = input.index();
    let time_base = input.time_base();
    let avg_frame_rate = input.avg_frame_rate();
    let frame_duration = if avg_frame_rate.numerator() != 0 {
        Duration::from_secs_f64(
            avg_frame_rate.denominator() as f64 / avg_frame_rate.numerator() as f64,
        )
    } else {
        Duration::from_millis(33)
    };
    let codec_ctx = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = codec_ctx.decoder().video()?;
    let width = decoder.width();
    let height = decoder.height();
    let mut scaler = ScalerContext::get(
        decoder.format(),
        width,
        height,
        Pixel::RGBA,
        width,
        height,
        ScalerFlags::BILINEAR,
    )?;

    let mut flush_video = |decoder: &mut ffmpeg::decoder::Video| -> anyhow::Result<bool> {
        let mut decoded = ffmpeg::util::frame::video::Video::empty();
        while decoder.receive_frame(&mut decoded).is_ok() {
            let mut rgba_frame = ffmpeg::util::frame::video::Video::empty();
            scaler.run(&decoded, &mut rgba_frame)?;

            let pts_secs = decoded
                .pts()
                .map(|pts| pts as f64 * f64::from(time_base))
                .unwrap_or(0.0);

            let stride = rgba_frame.stride(0);
            let data = rgba_frame.data(0);
            let row_bytes = (width * 4) as usize;

            let rgba = if stride == row_bytes {
                data.to_vec()
            } else {
                let mut buf = Vec::with_capacity(row_bytes * height as usize);
                for row in 0..height as usize {
                    let start = row * stride;
                    buf.extend_from_slice(&data[start..start + row_bytes]);
                }
                buf
            };

            if tx_video
                .send(VideoFrameData {
                    rgba,
                    width,
                    height,
                    pts_secs,
                    total_secs,
                    frame_duration,
                })
                .is_err()
            {
                return Ok(false);
            }
        }
        Ok(true)
    };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            if !flush_video(&mut decoder)? {
                return Ok(());
            }
        }
    }

    decoder.send_eof().ok();
    flush_video(&mut decoder)?;

    Ok(())
}

/// Decodes only the audio stream and resamples to stereo f32 at [`AUDIO_SAMPLE_RATE`].
/// Runs in its own OS thread, independently of video decoding.
fn decode_audio_thread(
    path: PathBuf,
    start_secs: f64,
    tx_audio: mpsc::SyncSender<AudioFrameData>,
) -> anyhow::Result<()> {
    ffmpeg::init()?;

    let mut ictx = ffmpeg::format::input(&path)?;

    if start_secs > 0.0 {
        let target_pts = (start_secs * ffmpeg::ffi::AV_TIME_BASE as f64) as i64;
        ictx.seek(target_pts, target_pts..).ok();
    }

    let audio_stream = match ictx.streams().best(MediaType::Audio) {
        Some(s) => s,
        None => return Ok(()),
    };
    let audio_idx = audio_stream.index();
    let audio_time_base = audio_stream.time_base();
    let ctx = ffmpeg::codec::context::Context::from_parameters(audio_stream.parameters())?;
    let mut dec = ctx.decoder().audio()?;
    let mut resampler = ResamplerCtx::get(
        dec.format(),
        dec.channel_layout(),
        dec.rate(),
        Sample::F32(sample::Type::Planar),
        ChannelLayout::STEREO,
        AUDIO_SAMPLE_RATE,
    )?;

    for (stream, packet) in ictx.packets() {
        if stream.index() != audio_idx {
            continue;
        }
        dec.send_packet(&packet).ok();
        let mut frame = AudioFrame::empty();
        while dec.receive_frame(&mut frame).is_ok() {
            let pts_secs = frame
                .pts()
                .map(|p| p as f64 * f64::from(audio_time_base))
                .unwrap_or(0.0);
            let mut resampled = AudioFrame::empty();
            if resampler.run(&frame, &mut resampled).is_ok() {
                let n = resampled.samples();
                if n > 0 {
                    let left = resampled.plane::<f32>(0);
                    let left_n = n.min(left.len());
                    let mut samples = Vec::with_capacity(left_n * 2);
                    if resampled.planes() >= 2 {
                        let right = resampled.plane::<f32>(1);
                        let right_n = left_n.min(right.len());
                        for i in 0..right_n {
                            samples.push(left[i].clamp(-1.0, 1.0));
                            samples.push(right[i].clamp(-1.0, 1.0));
                        }
                    } else {
                        for i in 0..left_n {
                            let s = left[i].clamp(-1.0, 1.0);
                            samples.push(s);
                            samples.push(s);
                        }
                    }
                    if tx_audio.send(AudioFrameData { samples, pts_secs }).is_err() {
                        return Ok(());
                    }
                }
            }
        }
    }

    Ok(())
}

fn format_time(secs: f64) -> String {
    let total = secs as u64;
    let m = total / 60;
    let s = total % 60;
    format!("{m}:{s:02}")
}

fn make_sk_image(rgba: &[u8], width: u32, height: u32) -> anyhow::Result<SkImage> {
    let row_bytes = (width * 4) as usize;
    let data = unsafe { Data::new_bytes(rgba) };
    raster_from_data(
        &ImageInfo::new(
            ISize::new(width as i32, height as i32),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        ),
        data,
        row_bytes,
    )
    .context("Failed to create SkImage from video frame")
}

/// Video player component. Plays a video from a [`VideoSource`].
///
/// Click to toggle play/pause. A progress bar is shown at the bottom.
///
/// Requires FFmpeg to be installed on the system.
///
/// # Example
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use std::path::PathBuf;
/// fn app() -> impl IntoElement {
///     VideoPlayer::new(PathBuf::from("./video.mp4"))
///         .width(Size::px(640.))
///         .height(Size::px(360.))
/// }
/// ```
#[derive(PartialEq)]
pub struct VideoPlayer {
    source: VideoSource,
    layout: LayoutData,
    key: DiffKey,
}

impl VideoPlayer {
    pub fn new(source: impl Into<VideoSource>) -> Self {
        VideoPlayer {
            source: source.into(),
            layout: LayoutData::default(),
            key: DiffKey::None,
        }
    }
}

impl KeyExt for VideoPlayer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for VideoPlayer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for VideoPlayer {}

impl Component for VideoPlayer {
    fn render(&self) -> impl IntoElement {
        let mut current_frame = use_state::<Option<Rc<SkImage>>>(|| None);
        let mut status = use_state(|| VideoStatus::Loading);
        let mut progress = use_state(|| 0.0f64);
        let mut tasks = use_state::<Vec<TaskHandle>>(Vec::new);
        let mut hovered = use_state(|| false);
        let mut total_secs = use_state(|| 0.0f64);
        // Seek offset in seconds; writing here triggers the side effect via reactive read
        let mut seek_offset = use_state(|| 0.0f64);
        let mut timeline_area = use_state(|| Area::default());
        let mut seeking = use_state(|| false);
        // Visual-only progress during drag; None when not scrubbing
        let mut drag_fraction: State<Option<f64>> = use_state(|| None);
        // Shared pause flag for the audio ChannelSource; updated on play/pause
        let mut audio_pause: State<Option<Arc<AtomicBool>>> = use_state(|| None);
        // Persistent volume Arc — shared with ChannelSource across seeks
        let volume_arc: State<Arc<AtomicU32>> =
            use_state(|| Arc::new(AtomicU32::new(f32::to_bits(1.0))));
        let mut volume = use_state(|| 1.0f64);
        let mut volume_area = use_state(|| Area::default());
        let mut volume_seeking = use_state(|| false);
        let mut volume_drag: State<Option<f64>> = use_state(|| None);

        use_side_effect_with_deps(&self.source, move |source: &VideoSource| {
            let source = source.clone();

            // Reactive read: this effect re-runs whenever seek_offset changes
            let offset = *seek_offset.read();
            // peek() — we don't want total_secs changes to restart the decoder
            let total_secs_val = *total_secs.peek();
            let has_ever_played = total_secs_val > 0.0;

            for task in tasks.write().drain(..) {
                task.cancel();
            }

            let is_currently_paused = matches!(*status.peek(), VideoStatus::Paused);

            if has_ever_played {
                // Seeking: keep current frame visible, avoid flashing the loading screen.
                // If the video had finished, restart playback.
                let was_finished = matches!(*status.peek(), VideoStatus::Finished);
                if was_finished {
                    *status.write() = VideoStatus::Playing;
                }
                *progress.write() = (offset / total_secs_val).clamp(0.0, 1.0);
            } else {
                *status.write() = VideoStatus::Loading;
                *current_frame.write() = None;
            }

            let (tx_video, rx_video) = mpsc::sync_channel::<VideoFrameData>(32);
            let rx_video = Arc::new(Mutex::new(rx_video));

            // Bounded audio channel — 200 frames ≈ 4.5s of pre-buffer; audio thread blocks
            // when full so it stays close to the playback position without wasting memory.
            let (tx_audio, rx_audio) = mpsc::sync_channel::<AudioFrameData>(200);

            let paused_flag = Arc::new(AtomicBool::new(is_currently_paused));
            let paused_flag_audio = paused_flag.clone();
            *audio_pause.write() = Some(paused_flag);

            let vol_arc = volume_arc.peek().clone();

            // For seeks (has_ever_played) video is already visible — start audio immediately.
            // For initial load, wait for the first video frame before consuming audio.
            let video_started = Arc::new(AtomicBool::new(has_ever_played));
            let video_started_audio = video_started.clone();

            // Shared video PTS (f64 bits) — written by recv_task, read by ChannelSource for sync.
            // Initialize with the seek offset so audio and video start at the same reference.
            let video_pts_arc: Arc<AtomicU64> =
                Arc::new(AtomicU64::new(f64::to_bits(offset)));
            let video_pts_recv = video_pts_arc.clone();

            let VideoSource::Path(path) = source;
            let audio_path = path.clone();

            // Audio output thread — keeps OutputStream alive until the source is exhausted
            std::thread::spawn(move || {
                if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
                    if let Ok(sink) = Sink::try_new(&stream_handle) {
                        sink.append(ChannelSource::new(
                            rx_audio,
                            paused_flag_audio,
                            vol_arc,
                            video_started_audio,
                            video_pts_arc,
                        ));
                        sink.sleep_until_end();
                    }
                }
            });

            // Audio decoder runs independently so video processing never starves audio
            std::thread::spawn(move || {
                if let Err(e) = decode_audio_thread(audio_path, offset, tx_audio) {
                    #[cfg(debug_assertions)]
                    tracing::error!("Audio decode error: {e:?}");
                }
            });

            std::thread::spawn(move || {
                if let Err(e) = decode_video_thread(path, offset, tx_video) {
                    #[cfg(debug_assertions)]
                    tracing::error!("Video decode error: {e:?}");
                }
            });

            let recv_task = spawn(async move {
                loop {
                    if matches!(*status.read(), VideoStatus::Paused) {
                        Timer::after(Duration::from_millis(16)).await;
                        continue;
                    }

                    let rx = rx_video.clone();
                    let frame_result = unblock(move || rx.lock().unwrap().recv()).await;

                    match frame_result {
                        Ok(frame_data) => {
                            let duration = frame_data.frame_duration;
                            let pts = frame_data.pts_secs;
                            let total = frame_data.total_secs;

                            match make_sk_image(&frame_data.rgba, frame_data.width, frame_data.height) {
                                Ok(img) => {
                                    // Update video PTS before ungating audio so ChannelSource
                                    // sees the correct reference timestamp immediately.
                                    video_pts_recv.store(f64::to_bits(pts), Ordering::Relaxed);
                                    *current_frame.write() = Some(Rc::new(img));
                                    let is_loading = matches!(*status.read(), VideoStatus::Loading);
                                    if is_loading {
                                        *status.write() = VideoStatus::Playing;
                                        video_started.store(true, Ordering::Relaxed);
                                    }
                                    if total > 0.0 {
                                        let needs_total = *total_secs.read() == 0.0;
                                        if needs_total {
                                            *total_secs.write() = total;
                                        }
                                        *progress.write() = (pts / total).clamp(0.0, 1.0);
                                    }
                                }
                                Err(e) => {
                                    *status.write() = VideoStatus::Error(e.to_string());
                                    break;
                                }
                            }

                            Timer::after(duration).await;
                        }
                        Err(_) => {
                            *status.write() = VideoStatus::Finished;
                            break;
                        }
                    }
                }
            });

            tasks.write().push(recv_task);
        });

        let on_click = move |_: Event<MouseEventData>| {
            let is_playing = matches!(*status.read(), VideoStatus::Playing);
            let is_pausable = matches!(*status.read(), VideoStatus::Paused | VideoStatus::Finished);
            if is_playing {
                *status.write() = VideoStatus::Paused;
                if let Some(flag) = &*audio_pause.peek() {
                    flag.store(true, Ordering::Relaxed);
                }
            } else if is_pausable {
                *status.write() = VideoStatus::Playing;
                if let Some(flag) = &*audio_pause.peek() {
                    flag.store(false, Ordering::Relaxed);
                }
            }
        };

        // Seek to a fraction [0, 1] of the video; writing seek_offset triggers the side effect
        let mut do_seek = move |fraction: f64| {
            let total = *total_secs.peek();
            if total > 0.0 {
                *seek_offset.write() = fraction * total;
                *progress.write() = fraction;
            }
        };

        let calc_fraction = move |global_x: f64| -> f64 {
            let area = *timeline_area.read();
            let rel_x = (global_x - area.min_x() as f64).max(0.0);
            (rel_x / area.width() as f64).clamp(0.0, 1.0)
        };

        let on_timeline_pointer_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            seeking.set(true);
            let coords = e.element_location();
            let area = *timeline_area.read();
            let fraction = (coords.x / area.width() as f64).clamp(0.0, 1.0);
            drag_fraction.set(Some(fraction));
        };

        let on_timeline_global_move = move |e: Event<PointerEventData>| {
            if *seeking.peek() {
                let fraction = calc_fraction(e.global_location().x);
                drag_fraction.set(Some(fraction));
            }
        };

        // GlobalPointerPress fires on mouse UP — do the actual seek here
        let on_timeline_global_press = move |_: Event<PointerEventData>| {
            let is_seeking = *seeking.peek();
            if is_seeking {
                seeking.set(false);
                let pending = *drag_fraction.peek();
                if let Some(fraction) = pending {
                    drag_fraction.set(None);
                    do_seek(fraction);
                }
            }
        };

        let calc_volume_fraction = move |global_x: f64| -> f64 {
            let area = *volume_area.read();
            let rel_x = (global_x - area.min_x() as f64).max(0.0);
            (rel_x / area.width() as f64).clamp(0.0, 1.0)
        };

        let mut apply_volume = move |fraction: f64| {
            volume_arc.peek().store(f32::to_bits(fraction as f32), Ordering::Relaxed);
            *volume.write() = fraction;
        };

        let on_volume_pointer_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            volume_seeking.set(true);
            let coords = e.element_location();
            let area = *volume_area.read();
            let fraction = (coords.x / area.width() as f64).clamp(0.0, 1.0);
            volume_drag.set(Some(fraction));
        };

        let on_volume_global_move = move |e: Event<PointerEventData>| {
            if *volume_seeking.peek() {
                let fraction = calc_volume_fraction(e.global_location().x);
                volume_drag.set(Some(fraction));
            }
        };

        let on_volume_global_press = move |_: Event<PointerEventData>| {
            let is_seeking = *volume_seeking.peek();
            if is_seeking {
                volume_seeking.set(false);
                let pending = *volume_drag.peek();
                if let Some(fraction) = pending {
                    volume_drag.set(None);
                    apply_volume(fraction);
                }
            }
        };

        let frame = current_frame.read().clone();
        // While scrubbing show the drag position; otherwise show actual playback position
        let progress_val = drag_fraction.read().unwrap_or(*progress.read());
        let volume_val = volume_drag.read().unwrap_or(*volume.read());
        let total_secs_val = *total_secs.read();
        let current_secs = progress_val * total_secs_val;
        let is_loading = frame.is_none() && matches!(*status.read(), VideoStatus::Loading);
        let is_paused = matches!(*status.read(), VideoStatus::Paused | VideoStatus::Finished);
        let is_hovered = *hovered.read();
        let error_msg = if let VideoStatus::Error(e) = &*status.read() {
            Some(e.clone())
        } else {
            None
        };

        if is_loading {
            return rect()
                .layout(self.layout.clone())
                .center()
                .child(CircularLoader::new())
                .into_element();
        }

        if let Some(msg) = error_msg {
            return msg.into_element();
        }

        rect()
            .layout(self.layout.clone())
            .background((0, 0, 0))
            .on_pointer_enter(move |_| *hovered.write() = true)
            .on_pointer_leave(move |_| *hovered.write() = false)
            .on_mouse_up(on_click)
            // Video frame with overlay rendered directly via Skia
            .child(
                video_frame(frame, is_hovered, is_paused)
                    .width(Size::fill())
                    .height(Size::fill()),
            )
            // Controls bar at the bottom
            .child(
                rect()
                    .width(Size::fill())
                    .background((15, 15, 15))
                    .padding((6.0, 10.0))
                    // Seekable timeline
                    .child(
                        rect()
                            .width(Size::fill())
                            .height(Size::px(6.0))
                            .corner_radius(3.0)
                            .background((60, 60, 60))
                            .on_sized(move |e: Event<SizedEventData>| timeline_area.set(e.area))
                            .on_pointer_down(on_timeline_pointer_down)
                            .on_mouse_up(move |e: Event<MouseEventData>| e.stop_propagation())
                            .on_global_pointer_move(on_timeline_global_move)
                            .on_global_pointer_press(on_timeline_global_press)
                            .child(
                                rect()
                                    .width(Size::percent((progress_val * 100.0) as f32))
                                    .height(Size::fill())
                                    .corner_radius(3.0)
                                    .background((210, 210, 210)),
                            ),
                    )
                    // Bottom row: time on the left, volume slider on the right
                    .child(
                        rect()
                            .width(Size::fill())
                            .padding((4.0, 0.0))
                            .horizontal()
                            .font_size(11.0)
                            .color((160, 160, 160))
                            // Time display
                            .child(
                                rect()
                                    .width(Size::fill())
                                    .child(format!(
                                        "{} / {}",
                                        format_time(current_secs),
                                        format_time(total_secs_val)
                                    )),
                            )
                            // Volume slider
                            .child(
                                rect()
                                    .width(Size::px(100.0))
                                    .horizontal()
                                    .on_mouse_up(move |e: Event<MouseEventData>| {
                                        e.stop_propagation()
                                    })
                                    // Speaker label
                                    .child("Vol ")
                                    // Track
                                    .child(
                                        rect()
                                            .width(Size::fill())
                                            .height(Size::px(4.0))
                                            .corner_radius(2.0)
                                            .background((60, 60, 60))
                                            .on_sized(move |e: Event<SizedEventData>| {
                                                volume_area.set(e.area)
                                            })
                                            .on_pointer_down(on_volume_pointer_down)
                                            .on_global_pointer_move(on_volume_global_move)
                                            .on_global_pointer_press(on_volume_global_press)
                                            .child(
                                                rect()
                                                    .width(Size::percent(
                                                        (volume_val * 100.0) as f32,
                                                    ))
                                                    .height(Size::fill())
                                                    .corner_radius(2.0)
                                                    .background((210, 210, 210)),
                                            ),
                                    ),
                            ),
                    ),
            )
            .into_element()
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

// --- VideoFrame element (internal) ---

pub struct VideoFrame {
    element: VideoFrameElement,
    key: DiffKey,
}

fn video_frame(
    current_frame: Option<Rc<SkImage>>,
    show_overlay: bool,
    is_paused: bool,
) -> VideoFrame {
    VideoFrame {
        key: DiffKey::None,
        element: VideoFrameElement {
            current_frame,
            show_overlay,
            is_paused,
            layout: LayoutData::default(),
        },
    }
}

impl KeyExt for VideoFrame {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for VideoFrame {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerSizeExt for VideoFrame {}

impl MaybeExt for VideoFrame {}

impl From<VideoFrame> for Element {
    fn from(value: VideoFrame) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: vec![],
        }
    }
}

#[derive(Clone)]
struct VideoFrameElement {
    layout: LayoutData,
    current_frame: Option<Rc<SkImage>>,
    show_overlay: bool,
    is_paused: bool,
}

impl PartialEq for VideoFrameElement {
    fn eq(&self, other: &Self) -> bool {
        self.layout == other.layout
            && self.show_overlay == other.show_overlay
            && self.is_paused == other.is_paused
            && match (&self.current_frame, &other.current_frame) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

impl ElementExt for VideoFrameElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(other) = (other.as_ref() as &dyn Any).downcast_ref::<VideoFrameElement>()
        else {
            return false;
        };
        self != other
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(other) = (other.as_ref() as &dyn Any).downcast_ref::<VideoFrameElement>()
        else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.layout != other.layout {
            diff.insert(DiffModifies::LAYOUT);
        }

        let frame_changed = match (&self.current_frame, &other.current_frame) {
            (Some(a), Some(b)) => !Rc::ptr_eq(a, b),
            (None, None) => false,
            _ => true,
        };
        if frame_changed
            || self.show_overlay != other.show_overlay
            || self.is_paused != other.is_paused
        {
            diff.insert(DiffModifies::STYLE);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout)
    }

    fn effect(&'_ self) -> Option<Cow<'_, EffectData>> {
        None
    }

    fn style(&'_ self) -> Cow<'_, StyleState> {
        Cow::Owned(StyleState::default())
    }

    fn text_style(&'_ self) -> Cow<'_, TextStyleData> {
        Cow::Owned(TextStyleData::default())
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Owned(AccessibilityData::default())
    }

    fn should_measure_inner_children(&self) -> bool {
        false
    }

    fn should_hook_measurement(&self) -> bool {
        false
    }

    fn clip(&self, context: ClipContext) {
        let area = context.visible_area;
        context.canvas.clip_rect(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            ClipOp::Intersect,
            true,
        );
    }

    fn render(&self, context: RenderContext) {
        let area = context.layout_node.visible_area();
        let rect = SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());

        // Draw video frame
        if let Some(frame) = &self.current_frame {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            context.canvas.draw_image_rect_with_sampling_options(
                frame.as_ref(),
                None,
                rect,
                SamplingOptions::new(FilterMode::Linear, MipmapMode::None),
                &paint,
            );
        }

        // Draw hover overlay: dark tint + play/pause icon
        if self.show_overlay {
            // Dark tint
            let mut tint = Paint::default();
            tint.set_style(PaintStyle::Fill);
            tint.set_color(Color::from_argb(140, 0, 0, 0));
            tint.set_anti_alias(true);
            context.canvas.draw_rect(rect, &tint);

            // Icon centered over the video
            let cx = (area.min_x() + area.max_x()) / 2.0;
            let cy = (area.min_y() + area.max_y()) / 2.0;
            let icon_size = 36.0_f32;

            let mut icon_paint = Paint::default();
            icon_paint.set_color(Color::WHITE);
            icon_paint.set_anti_alias(true);
            icon_paint.set_style(PaintStyle::Fill);

            if self.is_paused {
                // Play triangle
                let mut builder = PathBuilder::new();
                builder.move_to((cx - icon_size * 0.4, cy - icon_size * 0.6));
                builder.line_to((cx + icon_size * 0.7, cy));
                builder.line_to((cx - icon_size * 0.4, cy + icon_size * 0.6));
                builder.close();
                context.canvas.draw_path(&builder.snapshot(), &icon_paint);
            } else {
                // Pause bars
                let bar_w = icon_size * 0.28;
                let bar_h = icon_size * 1.1;
                let gap = icon_size * 0.22;
                context.canvas.draw_rect(
                    SkRect::from_xywh(cx - gap - bar_w, cy - bar_h / 2.0, bar_w, bar_h),
                    &icon_paint,
                );
                context.canvas.draw_rect(
                    SkRect::from_xywh(cx + gap, cy - bar_h / 2.0, bar_w, bar_h),
                    &icon_paint,
                );
            }
        }
    }
}
