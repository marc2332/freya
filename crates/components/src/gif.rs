use std::{
    io::Cursor,
    sync::{
        Arc,
        Mutex,
    },
    time::Duration,
};

use bytes::Bytes;
use dioxus::prelude::*;
use freya_elements::{
    self as dioxus_elements,
};
use freya_engine::prelude::{
    raster_from_data,
    raster_n32_premul,
    AlphaType,
    Color,
    ColorType,
    Data,
    ISize,
    ImageInfo,
    Paint,
    Rect,
    SamplingOptions,
    Surface,
};
use freya_hooks::{
    use_canvas,
    use_focus,
    use_node_signal,
    use_platform,
};
use gif::DisposalMethod;
pub use reqwest::Url;
use tokio::time::{
    sleep,
    Instant,
};

/// Wrapper over Skia's Surface.
///
/// # Safety
/// Marking this as Send is fine because even though it will go through a shipyard's thread, it will not be accessed at any time.
/// In fact, this surface is only ever used in the main thread.
struct SharedSurface(Surface);

/// # Safety
/// Marking this as Send is fine because even though it will go through a shipyard's thread, it will not be accessed at any time.
/// In fact, this surface is only ever used in the main thread.
unsafe impl Send for SharedSurface {}

enum GifTask {
    Wait(Duration),

    Repeat,
}

#[component]
pub fn Gif(
    data: ReadOnlySignal<Bytes>,
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

    let (tx, target_time, reset) = use_hook(|| {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<GifTask>(256);

        let target_time = Arc::new(Mutex::new(Instant::now()));

        let mut reset = Signal::new(());

        spawn(async move {
            while let Some(task) = rx.recv().await {
                match task {
                    GifTask::Repeat => {
                        reset.set(());

                        platform.request_animation_frame();
                        platform.invalidate_drawing_area(size_signal.peek().area);
                    }

                    GifTask::Wait(duration) => {
                        spawn(async move {
                            sleep(duration).await;

                            platform.request_animation_frame();
                            platform.invalidate_drawing_area(size_signal.peek().area);
                        });
                    }
                }
            }
        });

        (tx, target_time, reset)
    });

    let canvas = use_canvas(move || {
        // Reset signal used to forcefully recreate the GIF.
        reset();

        let mut decoder_options = gif::DecodeOptions::new();
        decoder_options.set_color_output(gif::ColorOutput::RGBA);
        let cursor = Cursor::new(data.read().clone());
        let mut decoder = decoder_options.read_info(cursor.clone()).unwrap();

        // Shared surface where the GIF is rendered
        // Why not simply render directly to the canvas instead of using this and then rendering this to the canvas?
        // The main canvas can get cleared at any moment but we can't recreate a GIF frame from scratch, therefore
        // this surface acts as a custom cache for rendering the gif.
        let gif_surface = Arc::new(Mutex::new(SharedSurface(
            raster_n32_premul((decoder.width() as i32, decoder.height() as i32)).unwrap(),
        )));

        to_owned![tx, target_time];
        move |ctx| {
            let area = ctx.area;
            let rect = Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());

            let mut paint = Paint::default();
            paint.set_anti_alias(false);

            // Skip render if the target time has not been reached
            // In this case, only the gif surface will be rendered again.
            // This can happen when resizing the window.
            if Instant::now() < *target_time.lock().unwrap() {
                ctx.canvas.draw_image_rect_with_sampling_options(
                    gif_surface.lock().unwrap().0.image_snapshot(),
                    None,
                    rect,
                    SamplingOptions::default(),
                    &paint,
                );
                return;
            }

            match decoder.read_next_frame() {
                // Render new frame
                Ok(Some(frame)) => {
                    // Clear the background if requested
                    if frame.dispose == DisposalMethod::Background {
                        let rect = Rect::from_xywh(
                            frame.left as f32,
                            frame.top as f32,
                            frame.width as f32,
                            frame.height as f32,
                        );
                        ctx.canvas.save();
                        ctx.canvas.clip_rect(rect, None, false);
                        ctx.canvas.clear(Color::TRANSPARENT);
                        ctx.canvas.restore();
                    }

                    let row_bytes = (frame.width * 4) as usize;
                    let gif = raster_from_data(
                        &ImageInfo::new(
                            ISize::new(frame.width as i32, frame.height as i32),
                            ColorType::RGBA8888,
                            AlphaType::Unpremul,
                            None,
                        ),
                        unsafe { Data::new_bytes(&frame.buffer) },
                        row_bytes,
                    )
                    .unwrap();

                    // Render the gif into the GIF surface
                    gif_surface.lock().unwrap().0.canvas().draw_image(
                        &gif,
                        (frame.left as f32, frame.top as f32),
                        None,
                    );

                    // And then just draw the gif surface into the main canvas
                    ctx.canvas.draw_image_rect_with_sampling_options(
                        gif_surface.lock().unwrap().0.image_snapshot(),
                        None,
                        rect,
                        SamplingOptions::default(),
                        &paint,
                    );

                    // Set the delay ms as new time target and register an async task to rerender
                    let duration = Duration::from_millis(frame.delay as u64 * 10);
                    tx.blocking_send(GifTask::Wait(duration)).unwrap();
                    *target_time.lock().unwrap() = Instant::now() + duration;
                }

                // No more frames
                Ok(None) => {
                    // Just draw the gif surface into the main canvas
                    ctx.canvas.draw_image_rect_with_sampling_options(
                        gif_surface.lock().unwrap().0.image_snapshot(),
                        None,
                        rect,
                        SamplingOptions::default(),
                        &paint,
                    );

                    tx.blocking_send(GifTask::Repeat).unwrap();
                    *target_time.lock().unwrap() = Instant::now();
                }
                // TODO: Something went wrong
                Err(_e) => {}
            }
        }
    });

    let a11y_id = focus.attribute();

    rsx!(rect {
        reference,
        canvas_reference: canvas.attribute(),
        width,
        height,
        min_width,
        min_height,
        a11y_id,
        a11y_role: "image",
        a11y_name: alt,
    })
}
