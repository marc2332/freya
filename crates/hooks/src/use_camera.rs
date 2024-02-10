use std::sync::{Arc, Mutex};

use crate::use_platform;
use dioxus_core::{prelude::spawn, use_hook, AttributeValue};
use dioxus_hooks::use_signal;
use dioxus_signals::{Signal, Writable};
use freya_node_state::{CustomAttributeValues, ImageReference};
pub use nokhwa::utils::{CameraIndex, RequestedFormatType, Resolution};
use nokhwa::{pixel_format::RgbFormat, utils::RequestedFormat, Camera, NokhwaError};

/// Configuration for a camera
pub struct CameraSettings {
    camera_index: CameraIndex,
    resolution: Option<Resolution>,
    camera_format: RequestedFormatType,
}

impl CameraSettings {
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
            camera_index: CameraIndex::Index(0),
            resolution: None,
            camera_format: RequestedFormatType::AbsoluteHighestFrameRate,
        }
    }
}

/// Connect to a given camera and render its frames into an image element
pub fn use_camera(
    camera_settings: CameraSettings,
) -> (AttributeValue, Signal<Option<NokhwaError>>) {
    let platform = use_platform();
    let mut camera_error = use_signal(|| None);
    let image_reference = use_hook(|| Arc::new(Mutex::new(None)));

    let image_reference_attr = AttributeValue::any_value(CustomAttributeValues::ImageReference(
        ImageReference(image_reference.clone()),
    ));

    use_hook(move || {
        spawn(async move {
            let mut handle_error = |e: NokhwaError| {
                camera_error.set(Some(e));
            };

            let requested = RequestedFormat::new::<RgbFormat>(camera_settings.camera_format);
            let camera = Camera::new(camera_settings.camera_index, requested);

            if let Ok(mut camera) = camera {
                // Set the custom resolution if specified
                if let Some(resolution) = camera_settings.resolution {
                    camera
                        .set_resolution(resolution)
                        .unwrap_or_else(&mut handle_error);
                }

                let mut ticker = platform.new_ticker();

                loop {
                    // Wait for the event loop to tick
                    ticker.tick().await;

                    // Capture the next frame
                    let frame = camera.frame();

                    if let Ok(frame) = frame {
                        let bts = frame.buffer_bytes();
                        // Send the frame to the renderer via the image reference
                        image_reference.lock().unwrap().replace(bts);

                        // Request the renderer to rerender
                        platform.request_animation_frame();
                    } else if let Err(err) = frame {
                        handle_error(err);
                        break;
                    }
                }
            } else if let Err(err) = camera {
                handle_error(err);
            }
        });
    });

    (image_reference_attr, camera_error)
}
