use std::{
    path::PathBuf,
    sync::{
        Arc,
        Mutex,
    },
    time::Duration,
};

use dioxus::prelude::*;
use ffmpeg::{
    decoder::video::Video as DecoderVideo,
    format::{
        input,
        Pixel,
    },
    media::Type,
    software::scaling::{
        context::Context,
        flag::Flags,
    },
    util::frame::video::Video as FrameVideo,
};
use ffmpeg_the_third::{
    self as ffmpeg,
    format::context::Input,
};
use freya_elements::{
    self as dioxus_elements,
    MouseEvent,
};
use freya_engine::prelude::{
    AlphaType,
    Bitmap,
    ClipOp,
    Color,
    ColorType,
    ImageInfo,
    Paint,
    Rect,
};
use freya_hooks::{
    use_canvas,
    use_focus,
    use_node_signal,
    use_platform,
};
use tokio::sync::oneshot;

use crate::Button;

enum VideoTask {
    Init {
        path: PathBuf,
        data: oneshot::Sender<Data>,
    },
    Play,
    PlayFrom {
        frame: usize,
        ts: i64,
    },
}

#[derive(Clone)]
struct Data {
    pub elapsed_duration: Duration,
    pub duration: Duration,
    pub size: (u32, u32),
    pub frames: i64,
    pub framerate: f64,
}

struct VideoFrame {
    frame: FrameVideo,
    timestamp: i64,
}

#[component]
pub fn Video(
    path: ReadOnlySignal<PathBuf>,
    /// Information about the gif.
    alt: Option<String>,
    /// Aspect ratio of the gif.
    aspect_ratio: Option<String>,
    /// Cover of the gif.
    cover: Option<String>,
    /// GIF sampling algorithm.
    sampling: Option<String>,
    /// Width of the gif container. Default to `auto`.
    #[props(default = "auto".into())]
    width: String,
    /// Height of the gif container. Default to `auto`.
    #[props(default = "auto".into())]
    height: String,
    /// Min width of the gif container.
    min_width: Option<String>,
    /// Min height of the gif container.
    min_height: Option<String>,
) -> Element {
    let platform = use_platform();
    let (reference, size_signal) = use_node_signal();
    let focus = use_focus();

    let (tasks, frames, frame, data) = use_hook(move || {
        let path = path();

        let frame = Arc::new(Mutex::new(0));
        let frames = Arc::new(Mutex::new(Vec::<VideoFrame>::new()));

        let (tx, rx) = oneshot::channel();
        let tasks = Arc::new(Mutex::new(vec![VideoTask::Init { path, data: tx }]));

        tokio::task::spawn_blocking({
            let frames = frames.clone();
            let frame = frame.clone();
            let tasks = tasks.clone();
            move || {
                enum PlayState {
                    Unloaded,
                    Loaded {
                        input: Input,
                        context: Context,
                        video: DecoderVideo,
                        video_stream_index: usize,
                    },
                }

                let mut state = PlayState::Unloaded;
                loop {
                    while let Some(task) = tasks.lock().unwrap().pop() {
                        match (task, &mut state) {
                            (VideoTask::Init { path, data }, PlayState::Unloaded) => {
                                let input = input(path).expect("failed to create input");
                                let (video_stream_index, context) = {
                                    let stream = input
                                        .streams()
                                        .best(Type::Video)
                                        .ok_or(ffmpeg::Error::StreamNotFound)
                                        .unwrap();
                                    (
                                        stream.index(),
                                        ffmpeg::codec::context::Context::from_parameters(
                                            stream.parameters(),
                                        )
                                        .unwrap(),
                                    )
                                };
                                let video = context.decoder().video().unwrap();

                                let context = Context::get(
                                    video.format(),
                                    video.width(),
                                    video.height(),
                                    Pixel::RGB24,
                                    video.width(),
                                    video.height(),
                                    Flags::BILINEAR,
                                )
                                .unwrap();
                                let stream = input.stream(video_stream_index).unwrap();
                                let duration = Duration::from_millis(
                                    (stream.duration().max(1) as f64
                                        * f64::from(stream.time_base())
                                        * 1000.) as u64,
                                );
                                let frames = if stream.frames() == 0 {
                                    duration.as_millis() as i64
                                        / f64::from(stream.avg_frame_rate()) as i64
                                } else {
                                    stream.frames()
                                };

                                data.send(Data {
                                    elapsed_duration: Duration::from_secs(0),
                                    duration,
                                    size: (
                                        stream.parameters().width(),
                                        stream.parameters().height(),
                                    ),
                                    frames,
                                    framerate: stream.avg_frame_rate().into(),
                                })
                                .ok();
                                state = PlayState::Loaded {
                                    input,
                                    context,
                                    video,
                                    video_stream_index,
                                };
                            }
                            (VideoTask::Play, PlayState::Loaded { .. }) => {}
                            (
                                VideoTask::PlayFrom {
                                    ts,
                                    frame: from_frame,
                                },
                                PlayState::Loaded { input, .. },
                            ) => {
                                *frame.lock().unwrap() = from_frame;
                                // *last_frame = ts;
                                let timestamp = ts * ffmpeg::ffi::AV_TIME_BASE as i64;

                                input.seek(timestamp, ..timestamp + 1).unwrap();
                            }
                            _ => {}
                        };
                    }

                    if let PlayState::Loaded {
                        ref mut input,
                        context,
                        video,
                        video_stream_index,
                    } = &mut state
                    {
                        let mut packet = ffmpeg::Packet::empty();
                        if packet.read(input).is_err() {
                            continue;
                        };

                        if packet.stream() == *video_stream_index {
                            video.send_packet(&packet).unwrap();

                            let stream = input.stream(packet.stream()).unwrap();
                            let mut decoded = FrameVideo::empty();
                            while video.receive_frame(&mut decoded).is_ok() {
                                let mut new_frame = FrameVideo::empty();
                                context.run(&decoded, &mut new_frame).unwrap();

                                // ???
                                let pts = new_frame
                                    .timestamp()
                                    .or_else(|| new_frame.pts())
                                    .unwrap_or(packet.pts().unwrap_or(0)); // fallback

                                let seconds = pts as f64 * f64::from(stream.time_base()); // seconds
                                let millis = (seconds * 1000.0).round() as i64;

                                let mut frames = frames.lock().unwrap();
                                let exists = frames.iter().any(|f| f.timestamp == millis);
                                let closest_frame_position = frames
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(i, frame)| {
                                        if frame.timestamp < millis {
                                            Some(i)
                                        } else {
                                            None
                                        }
                                    })
                                    .last();

                                if !exists && millis != i64::MIN {
                                    if let Some(i) = closest_frame_position {
                                        frames.insert(
                                            i,
                                            VideoFrame {
                                                frame: new_frame,
                                                timestamp: millis,
                                            },
                                        );
                                    } else {
                                        frames.push(VideoFrame {
                                            frame: new_frame,
                                            timestamp: millis,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        let mut data = Signal::new(rx.blocking_recv().unwrap());

        spawn({
            let frame = frame.clone();
            let frames = frames.clone();
            async move {
                let ms_per_frame = (1000. / data.read().framerate) as u64;
                let mut interval = tokio::time::interval(Duration::from_millis(ms_per_frame));
                loop {
                    interval.tick().await;
                    platform.request_animation_frame();
                    platform.invalidate_drawing_area(size_signal.peek().area);

                    let frames = frames.lock().unwrap();
                    let mut frame = frame.lock().unwrap();
                    if frames.len() > *frame {
                        // Move 1 frame forward
                        *frame += 1;

                        // Sync elapsed duration
                        data.write().elapsed_duration =
                            Duration::from_millis(*frame as u64 * ms_per_frame);
                    }
                }
            }
        });

        (tasks, frames, frame, data)
    });

    let canvas = use_canvas(move || {
        let size = data.peek().size;
        to_owned![frames, frame];
        move |ctx| {
            let area = ctx.area;
            let rect = Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());
            ctx.canvas.save();
            ctx.canvas.clip_rect(rect, ClipOp::Intersect, true);

            let mut paint = Paint::default();
            paint.set_color(Color::BLACK);
            ctx.canvas.draw_rect(rect, &paint);

            let location = (
                area.center().x as i32 - size.0 as i32 / 2,
                area.center().y as i32 - size.1 as i32 / 2,
            );

            let mut paint = Paint::default();
            paint.set_anti_alias(false);

            let frames = frames.lock().unwrap();
            let Some(frame) = frames.get(*frame.lock().unwrap()) else {
                return;
            };
            fn convert_rgb_to_rgba(rgb_frame: &FrameVideo) -> Vec<u8> {
                let width = rgb_frame.width() as usize;
                let height = rgb_frame.height() as usize;
                let stride = rgb_frame.stride(0);
                let data = rgb_frame.data(0);

                let mut rgba_pixels = Vec::with_capacity(width * height * 4);

                for y in 0..height {
                    let row_start = y * stride;
                    let row = &data[row_start..row_start + width * 3];

                    for x in 0..width {
                        let i = x * 3;
                        rgba_pixels.push(row[i]); // R
                        rgba_pixels.push(row[i + 1]); // G
                        rgba_pixels.push(row[i + 2]); // B
                        rgba_pixels.push(255); // A
                    }
                }

                rgba_pixels
            }

            fn rgba_pixels_to_bitmap(
                rgba_pixels: &mut [u8],
                width: usize,
                height: usize,
            ) -> Bitmap {
                let mut bitmap = Bitmap::new();
                let image_info = ImageInfo::new(
                    (width as i32, height as i32),
                    ColorType::RGBA8888,
                    AlphaType::Unpremul,
                    None,
                );
                let _ = bitmap.set_info(&image_info, width * 4); // row bytes = width * 4 bytes per pixel
                let ptr = rgba_pixels.as_mut_ptr() as *mut std::ffi::c_void;
                unsafe {
                    bitmap.install_pixels(&image_info, ptr, width * 4);
                }
                bitmap
            }

            let mut bytes = convert_rgb_to_rgba(&frame.frame);
            let bitmap = rgba_pixels_to_bitmap(
                &mut bytes,
                frame.frame.width() as usize,
                frame.frame.height() as usize,
            );

            let _ = ctx.canvas.write_pixels_from_bitmap(&bitmap, location);
            ctx.canvas.restore();
        }
    });

    let mut clicking = use_signal(|| false);

    let onplay = {
        let tasks = tasks.clone();
        move |_| {
            tasks.lock().unwrap().push(VideoTask::Play);
        }
    };

    let onglobalclick = move |_| clicking.set(false);

    let onclick = {
        let tasks = tasks.clone();
        move |e: MouseEvent| {
            clicking.set(false);
            let per_x = e.element_coordinates.x as f32 / size_signal.read().area.width() * 100.;
            let selected_frame = data.read().frames as f32 / 100. * per_x;
            let selected_ts = data.read().duration.as_millis() as f32 / 100. * per_x;
            tasks.lock().unwrap().push(VideoTask::PlayFrom {
                ts: selected_ts as i64,
                frame: selected_frame as usize,
            });
        }
    };

    let onmousedown = move |_| {
        clicking.set(true);
    };

    let onmousemove = move |e: MouseEvent| {
        if clicking() {
            let per_x = e.element_coordinates.x as f32 / size_signal.read().area.width() * 100.;
            let selected_frame = data.read().frames as f32 / 100. * per_x;
            let selected_ts = data.read().duration.as_millis() as f32 / 100. * per_x;
            tasks.lock().unwrap().push(VideoTask::PlayFrom {
                ts: selected_ts as i64,
                frame: selected_frame as usize,
            });
        }
    };

    let a11y_id = focus.attribute();

    let elapsed_duration = data.read().elapsed_duration.as_secs();
    let total_duration = data.read().duration.as_secs();

    let progress = elapsed_duration as f32 / total_duration as f32 * 100.;

    rsx!(
        rect {
            reference,
            canvas_reference: canvas.attribute(),
            width,
            height,
            min_width,
            min_height,
            a11y_id,
            a11y_role: "image",
            a11y_name: alt,
            rect {
                position: "absolute",
                position_bottom: "0",
                height: "70",
                layer: "-999",
                background: "rgb(170, 170, 170)",
                rect {
                    height: "20",
                    width: "fill",
                    onglobalclick,
                    onclick,
                    onmousedown,
                    onmousemove,
                    rect {
                        corner_radius: "16",
                        width: "{progress}%",
                        height: "fill",
                        background: "rgb(103, 80, 164)"
                    }
                }
                Button {
                    onpress: onplay,
                    label {
                        "Play"
                    }
                }
                label {
                    color: "white",
                    "{elapsed_duration}:{total_duration}"
                }
            }
        }
    )
}
