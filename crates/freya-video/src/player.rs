use std::{
    rc::Rc,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};

use async_io::Timer;
use blocking::unblock;
use freya_core::{integration::*, prelude::*};
use freya_engine::prelude::SkImage;
use rodio::{OutputStream, Sink};
use torin::{
    geometry::Area,
    prelude::{Alignment, Content, Position},
    size::Size,
};

use crate::{
    audio::{AudioFrameData, ChannelSource},
    controls::VideoControls,
    decoder::{VideoFrameData, decode_audio_thread, decode_video_thread},
    frame::video_frame,
    source::{FfmpegInput, VideoSource, VideoStatus},
    utils::{format_time, make_sk_image},
};

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
pub struct VideoPlayer {
    pub(crate) source: VideoSource,
    pub(crate) layout: LayoutData,
    pub(crate) key: DiffKey,
    /// Whether the video starts playing automatically when rendered. Defaults to `true`.
    pub(crate) autoplay: bool,
    /// Whether to hide the playback controls overlay (play/pause icon, timeline, volume). Defaults to `false`.
    pub(crate) hide_controls: bool,
    /// Optional closure to render custom controls. Receives [`VideoControls`] with current
    /// state and actions. When set, the built-in overlay is replaced entirely.
    pub(crate) custom_controls: Option<Rc<dyn Fn(VideoControls) -> Element>>,
}

impl PartialEq for VideoPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.layout == other.layout
            && self.key == other.key
            && self.autoplay == other.autoplay
            && self.hide_controls == other.hide_controls
            && match (&self.custom_controls, &other.custom_controls) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

impl VideoPlayer {
    pub fn new(source: impl Into<VideoSource>) -> Self {
        VideoPlayer {
            source: source.into(),
            layout: LayoutData::default(),
            key: DiffKey::None,
            autoplay: true,
            hide_controls: false,
            custom_controls: None,
        }
    }

    /// Set whether the video plays automatically when rendered. Defaults to `true`.
    pub fn autoplay(mut self, autoplay: bool) -> Self {
        self.autoplay = autoplay;
        self
    }

    /// Set whether to hide all playback controls. Defaults to `false`.
    pub fn hide_controls(mut self, hide: bool) -> Self {
        self.hide_controls = hide;
        self
    }

    /// Provide a custom controls builder. Receives [`VideoControls`] with current playback
    /// state and actions on every render. When set, replaces the built-in overlay entirely.
    pub fn custom_controls(mut self, builder: Rc<dyn Fn(VideoControls) -> Element>) -> Self {
        self.custom_controls = Some(builder);
        self
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
        let autoplay = self.autoplay;
        let hide_controls = self.hide_controls;
        let custom_controls_builder = self.custom_controls.clone();
        let mut current_frame = use_state::<Option<Rc<SkImage>>>(|| None);
        let mut status = use_state(|| VideoStatus::Loading);
        let mut progress = use_state(|| 0.0f64);
        let mut tasks = use_state::<Vec<TaskHandle>>(Vec::new);
        let mut hovered = use_state(|| false);
        let mut hide_task: State<Option<TaskHandle>> = use_state(|| None);
        let mut total_secs = use_state(|| 0.0f64);
        // Seek offset in seconds; writing here triggers the side effect via reactive read
        let mut seek_offset = use_state(|| 0.0f64);
        let mut timeline_area = use_state(|| Area::default());
        let mut seeking = use_state(|| false);
        // Visual-only progress during drag; None when not scrubbing
        let mut drag_fraction: State<Option<f64>> = use_state(|| None);
        // Shared pause flag for the audio ChannelSource; updated on play/pause
        let mut audio_pause: State<Option<Arc<AtomicBool>>> = use_state(|| None);
        // Abort flag for the current audio session; set to true before starting a new one
        let mut audio_abort: State<Option<Arc<AtomicBool>>> = use_state(|| None);
        // Abort flag for the current video decoder thread
        let mut video_abort: State<Option<Arc<AtomicBool>>> = use_state(|| None);
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

            // Stop the previous audio and video decoder threads before starting new ones
            if let Some(old) = audio_abort.peek().as_ref() {
                old.store(true, Ordering::Relaxed);
            }
            if let Some(old) = video_abort.peek().as_ref() {
                old.store(true, Ordering::Relaxed);
            }

            let audio_abort_flag = Arc::new(AtomicBool::new(false));
            let abort_for_decoder = audio_abort_flag.clone();
            let abort_for_source = audio_abort_flag.clone();
            *audio_abort.write() = Some(audio_abort_flag);

            let video_abort_flag = Arc::new(AtomicBool::new(false));
            let abort_for_video = video_abort_flag.clone();
            *video_abort.write() = Some(video_abort_flag);

            let is_currently_paused =
                matches!(*status.peek(), VideoStatus::Paused) || (!has_ever_played && !autoplay);

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

            let (tx_video, rx_video) = mpsc::sync_channel::<VideoFrameData>(4);
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
            let video_pts_arc: Arc<AtomicU64> = Arc::new(AtomicU64::new(f64::to_bits(offset)));
            let video_pts_recv = video_pts_arc.clone();

            let FfmpegInput {
                url: input,
                extra_headers,
            } = source.into_ffmpeg_input();
            let audio_input = input.clone();
            let audio_extra_headers = extra_headers.clone();

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
                            abort_for_source,
                        ));
                        sink.sleep_until_end();
                    }
                }
            });

            // Audio decoder runs independently so video processing never starves audio
            std::thread::spawn(move || {
                if let Err(e) = decode_audio_thread(
                    audio_input,
                    audio_extra_headers,
                    offset,
                    tx_audio,
                    abort_for_decoder,
                ) {
                    #[cfg(debug_assertions)]
                    tracing::error!("Audio decode error: {e:?}");
                }
            });

            let video_error: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
            let video_error_thread = video_error.clone();
            std::thread::spawn(move || {
                if let Err(e) =
                    decode_video_thread(input, extra_headers, offset, tx_video, abort_for_video)
                {
                    if let Ok(mut guard) = video_error_thread.lock() {
                        *guard = Some(e.to_string());
                    }
                }
            });

            let recv_task = spawn(async move {
                // Absolute timing anchor: (wall_clock, pts) of the first displayed frame.
                // Reset to None on pause so timing re-anchors cleanly on resume.
                let mut play_anchor: Option<(Instant, f64)> = None;

                loop {
                    if matches!(*status.read(), VideoStatus::Paused) {
                        play_anchor = None;
                        Timer::after(Duration::from_millis(16)).await;
                        continue;
                    }

                    let rx = rx_video.clone();
                    let frame_result = unblock(move || rx.lock().unwrap().recv()).await;

                    match frame_result {
                        Ok(frame_data) => {
                            let pts = frame_data.pts_secs;
                            let total = frame_data.total_secs;

                            match make_sk_image(
                                &frame_data.rgba,
                                frame_data.width,
                                frame_data.height,
                            ) {
                                Ok(img) => {
                                    video_pts_recv.store(f64::to_bits(pts), Ordering::Relaxed);
                                    *current_frame.write() = Some(Rc::new(img));
                                    let is_loading =
                                        matches!(*status.read(), VideoStatus::Loading);
                                    if is_loading {
                                        *status.write() = if autoplay {
                                            VideoStatus::Playing
                                        } else {
                                            VideoStatus::Paused
                                        };
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

                            // Absolute PTS-based sleep: corrects drift every frame instead
                            // of letting per-frame timer errors accumulate.
                            let now = Instant::now();
                            let (anchor_wall, anchor_pts) =
                                play_anchor.get_or_insert((now, pts));
                            let elapsed = now.saturating_duration_since(*anchor_wall);
                            let target =
                                Duration::from_secs_f64((pts - *anchor_pts).max(0.0));
                            if target > elapsed {
                                Timer::after(target - elapsed).await;
                            }
                        }
                        Err(_) => {
                            if let Ok(guard) = video_error.lock() {
                                if let Some(msg) = guard.as_ref() {
                                    *status.write() = VideoStatus::Error(msg.clone());
                                    break;
                                }
                            }
                            *status.write() = VideoStatus::Finished;
                            break;
                        }
                    }
                }
            });

            tasks.write().push(recv_task);
        });

        let on_click = move |_: Event<MouseEventData>| {
            let is_playing = matches!(*status.peek(), VideoStatus::Playing);
            let is_paused = matches!(*status.peek(), VideoStatus::Paused);
            let is_finished = matches!(*status.peek(), VideoStatus::Finished);
            if is_playing {
                *status.write() = VideoStatus::Paused;
                if let Some(flag) = &*audio_pause.peek() {
                    flag.store(true, Ordering::Relaxed);
                }
            } else if is_paused {
                *status.write() = VideoStatus::Playing;
                if let Some(flag) = &*audio_pause.peek() {
                    flag.store(false, Ordering::Relaxed);
                }
            } else if is_finished {
                *seek_offset.write() = 0.0;
                *progress.write() = 0.0;
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
            volume_arc
                .peek()
                .store(f32::to_bits(fraction as f32), Ordering::Relaxed);
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
                volume_arc
                    .peek()
                    .store(f32::to_bits(fraction as f32), Ordering::Relaxed);
                *volume.write() = fraction;
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

        let custom_controls_el = custom_controls_builder.as_ref().map(|builder| {
            let is_playing = matches!(*status.peek(), VideoStatus::Playing);
            let is_paused_c = matches!(*status.peek(), VideoStatus::Paused);
            let is_finished_c = matches!(*status.peek(), VideoStatus::Finished);
            let progress_c = *progress.peek();
            let total_c = *total_secs.peek();
            let volume_c = *volume.peek();
            builder(VideoControls {
                is_playing,
                is_paused: is_paused_c,
                is_finished: is_finished_c,
                progress: progress_c,
                current_secs: progress_c * total_c,
                total_secs: total_c,
                volume: volume_c,
                toggle_play: Rc::new(move || {
                    let playing = matches!(*status.peek(), VideoStatus::Playing);
                    let paused_v = matches!(*status.peek(), VideoStatus::Paused);
                    let finished_v = matches!(*status.peek(), VideoStatus::Finished);
                    if playing {
                        *status.write_unchecked() = VideoStatus::Paused;
                        if let Some(flag) = &*audio_pause.peek() {
                            flag.store(true, Ordering::Relaxed);
                        }
                    } else if paused_v {
                        *status.write_unchecked() = VideoStatus::Playing;
                        if let Some(flag) = &*audio_pause.peek() {
                            flag.store(false, Ordering::Relaxed);
                        }
                    } else if finished_v {
                        *seek_offset.write_unchecked() = 0.0;
                        *progress.write_unchecked() = 0.0;
                    }
                }),
                seek: Rc::new(move |fraction: f64| {
                    let total = *total_secs.peek();
                    if total > 0.0 {
                        *seek_offset.write_unchecked() = fraction * total;
                        *progress.write_unchecked() = fraction;
                    }
                }),
                set_volume: Rc::new(move |v: f64| {
                    volume_arc
                        .peek()
                        .store(f32::to_bits(v as f32), Ordering::Relaxed);
                    *volume.write_unchecked() = v;
                }),
            })
        });
        let has_custom_controls = custom_controls_el.is_some();

        let on_mouse_activity = move |_: Event<MouseEventData>| {
            *hovered.write() = true;
            if let Some(task) = hide_task.write().take() {
                task.cancel();
            }
            let task = spawn(async move {
                Timer::after(Duration::from_secs(3)).await;
                *hovered.write() = false;
            });
            *hide_task.write() = Some(task);
        };

        let on_pointer_leave = move |_: Event<PointerEventData>| {
            *hovered.write() = false;
            if let Some(task) = hide_task.write().take() {
                task.cancel();
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
                .background((0, 0, 0))
                .center()
                .child("Loading...")
                .into_element();
        }

        if let Some(msg) = error_msg {
            return msg.into_element();
        }

        rect()
            .layout(self.layout.clone())
            .background((0, 0, 0))
            .maybe(!hide_controls && !has_custom_controls, |r| {
                r.on_mouse_move(on_mouse_activity)
                    .on_pointer_leave(on_pointer_leave)
            })
            .on_mouse_up(on_click)
            // Video frame fills the full container
            .child(
                video_frame(frame, is_hovered && !hide_controls && !has_custom_controls, is_paused)
                    .width(Size::fill())
                    .height(Size::fill()),
            )
            // Custom controls: always visible, user handles layout and positioning
            .maybe_child(custom_controls_el)
            // Zero-size anchor stacked at the bottom edge of the video frame.
            // Its absolute child is positioned relative to this anchor's bottom
            // edge, which coincides with the video player's bottom edge.
            .maybe_child((is_hovered && !hide_controls && !has_custom_controls).then(move || {
                rect().width(Size::fill()).height(Size::px(0.)).child(
                    rect()
                        .position(Position::new_absolute().bottom(0.).left(0.))
                        .layer(Layer::Overlay)
                        .width(Size::fill())
                        .background((10, 10, 10, 200))
                        .padding((8.0, 12.0))
                        .on_mouse_up(move |e: Event<MouseEventData>| e.stop_propagation())
                        // Seekable timeline
                        .child(
                            rect()
                                .width(Size::fill())
                                .height(Size::px(4.0))
                                .corner_radius(2.0)
                                .background((80, 80, 80))
                                .on_sized(move |e: Event<SizedEventData>| timeline_area.set(e.area))
                                .on_pointer_down(on_timeline_pointer_down)
                                .on_mouse_up(move |e: Event<MouseEventData>| e.stop_propagation())
                                .on_global_pointer_move(on_timeline_global_move)
                                .on_global_pointer_press(on_timeline_global_press)
                                .child(
                                    rect()
                                        .width(Size::percent((progress_val * 100.0) as f32))
                                        .height(Size::fill())
                                        .corner_radius(2.0)
                                        .background((255, 255, 255)),
                                ),
                        )
                        // Bottom row: time on the left, volume slider on the right
                        .child(
                            rect()
                                .width(Size::fill())
                                .margin((6.0, 0.0, 0.0, 0.0))
                                .horizontal()
                                .content(Content::Flex)
                                .font_size(12.0)
                                .color((200, 200, 200))
                                // Time display
                                .child(rect().width(Size::flex(1.0)).child(format!(
                                    "{} / {}",
                                    format_time(current_secs),
                                    format_time(total_secs_val)
                                )))
                                // Volume label + slider
                                .child(
                                    rect()
                                        .width(Size::px(110.0))
                                        .horizontal()
                                        .cross_align(Alignment::center())
                                        .on_mouse_up(move |e: Event<MouseEventData>| {
                                            e.stop_propagation()
                                        })
                                        .child(
                                            rect()
                                                .width(Size::px(28.0))
                                                .color((180, 180, 180))
                                                .font_size(12.0)
                                                .child("Vol"),
                                        )
                                        // Track (hit area = visual area, rounded)
                                        .child(
                                            rect()
                                                .width(Size::fill())
                                                .height(Size::px(4.0))
                                                .corner_radius(2.0)
                                                .background((80, 80, 80))
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
                                                        .background((255, 255, 255)),
                                                ),
                                        ),
                                ),
                        ),
                )
            }))
            .into_element()
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
