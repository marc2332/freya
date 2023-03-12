use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use dioxus_core::{AttributeValue, ScopeState};
use dioxus_hooks::{to_owned, use_effect, use_state, UseState};
use freya_common::EventMessage;
use freya_node_state::{CustomAttributeValues, ImageReference};
use nokhwa::{pixel_format::RgbFormat, utils::RequestedFormat, Camera, NokhwaError};
use tokio::time::sleep;
use winit::event_loop::EventLoopProxy;

pub use nokhwa::utils::{CameraIndex, RequestedFormatType, Resolution};

/// Configuration for a camera
pub struct CameraSettings {
    frame_rate: u32,
    camera_index: CameraIndex,
    resolution: Option<Resolution>,
    camera_format: RequestedFormatType,
}

impl CameraSettings {
    /// Specify a frame rate
    pub fn with_frame_rate(mut self, frame_rate: u32) -> Self {
        self.frame_rate = frame_rate;
        self
    }

    /// Specify a camera index   
    pub fn with_camera_index(mut self, camera_index: CameraIndex) -> Self {
        self.camera_index = camera_index;
        self
    }

    /// Specify a resolution
    pub fn with_resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = Some(resolution);
        self
    }

    /// Specify a camera format
    pub fn with_camera_format(mut self, camera_format: RequestedFormatType) -> Self {
        self.camera_format = camera_format;
        self
    }
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            frame_rate: 60,
            camera_index: CameraIndex::Index(0),
            resolution: None,
            camera_format: RequestedFormatType::AbsoluteHighestFrameRate,
        }
    }
}

/// Connect to a given camera and render its frames into an image element
pub fn use_camera(
    cx: &ScopeState,
    camera_settings: CameraSettings,
) -> (AttributeValue, &UseState<Option<NokhwaError>>) {
    let camera_error = use_state(cx, || None);
    let image_reference = cx.use_hook(|| Arc::new(Mutex::new(None)));

    let image_reference_attr = cx.any_value(CustomAttributeValues::ImageReference(ImageReference(
        image_reference.clone(),
    )));

    use_effect(cx, (), move |_| {
        to_owned![image_reference, camera_error];
        let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
        async move {
            let handle_error = |e: NokhwaError| {
                camera_error.set(Some(e));
            };

            let requested = RequestedFormat::new::<RgbFormat>(camera_settings.camera_format);
            let camera = Camera::new(camera_settings.camera_index, requested);

            if let Ok(mut camera) = camera {
                // Set the custom resolution if specified
                if let Some(resolution) = camera_settings.resolution {
                    camera
                        .set_resolution(resolution)
                        .unwrap_or_else(handle_error);
                }

                let frame_rate = camera_settings.frame_rate;
                let fps = 1000 / frame_rate;

                loop {
                    sleep(Duration::from_millis(fps as u64)).await;

                    // Capture the next frame
                    let frame = camera.frame();

                    if let Ok(frame) = frame {
                        let bts = frame.buffer_bytes();
                        // Send the frame to the renderer via the image reference
                        image_reference.lock().unwrap().replace(bts);

                        // Request the renderer to relayout
                        if let Some(event_loop_proxy) = &event_loop_proxy {
                            event_loop_proxy
                                .send_event(EventMessage::RequestRerender)
                                .unwrap();
                        }
                    } else if let Err(err) = frame {
                        handle_error(err);
                    }
                }
            } else if let Err(err) = camera {
                handle_error(err);
            }
        }
    });

    (image_reference_attr, camera_error)
}
