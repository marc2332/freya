use std::sync::{
    Arc,
    Mutex,
};

use bytes::Bytes;
use dioxus_core::{
    prelude::spawn,
    use_hook,
    AttributeValue,
};
use dioxus_hooks::{
    to_owned,
    use_effect,
    use_reactive,
    use_signal,
};
use dioxus_signals::{
    ReadOnlySignal,
    Readable,
    Signal,
    Writable,
};
use freya_core::custom_attributes::{
    CustomAttributeValues,
    ImageReference,
    NodeReference,
};
pub use nokhwa::utils::{
    CameraIndex,
    RequestedFormatType,
    Resolution,
};
use nokhwa::{
    pixel_format::RgbFormat,
    utils::RequestedFormat,
    Camera,
    NokhwaError,
};

use crate::{
    use_node_with_reference,
    use_platform,
};

/// Configuration for a camera
#[derive(Clone, PartialEq, Debug)]
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

#[derive(Debug, Clone)]
pub struct UseCamera {
    error: Signal<Option<NokhwaError>>,
    node_reference: NodeReference,
    image: Arc<Mutex<Option<Bytes>>>,
}

impl UseCamera {
    /// Get a [AttributeValue] for the `reference` attribute.
    pub fn attribute(&self) -> AttributeValue {
        AttributeValue::any_value(CustomAttributeValues::Reference(
            self.node_reference.clone(),
        ))
    }

    /// Get a [AttributeValue] for the `image_reference` attribute.
    pub fn image_attribute(&self) -> AttributeValue {
        AttributeValue::any_value(CustomAttributeValues::ImageReference(ImageReference(
            self.image.clone(),
        )))
    }

    /// Get a [ReadOnlySignal] of the error.
    pub fn error(&self) -> ReadOnlySignal<Option<NokhwaError>> {
        self.error.into()
    }
}

/// Connect to a given camera and render its frames into an image element
pub fn use_camera(camera_settings: CameraSettings) -> UseCamera {
    let platform = use_platform();
    let mut error = use_signal(|| None);
    let image = use_hook(|| Arc::new(Mutex::new(None)));
    let (node_reference, size) = use_node_with_reference();

    let camera = UseCamera {
        error,
        image: image.clone(),
        node_reference,
    };

    use_effect(use_reactive!(|camera_settings| {
        to_owned![image];
        spawn(async move {
            let requested = RequestedFormat::new::<RgbFormat>(camera_settings.camera_format);
            let camera = Camera::new(camera_settings.camera_index, requested);

            if let Ok(mut camera) = camera {
                // Set the custom resolution if specified
                if let Some(resolution) = camera_settings.resolution {
                    if let Err(err) = camera.set_resolution(resolution) {
                        error.set(Some(err));
                    }
                }

                let mut ticker = platform.new_ticker();

                loop {
                    // Wait for the event loop to tick
                    ticker.tick().await;

                    // Capture the next frame
                    let frame = camera.frame();

                    if let Ok(frame) = frame {
                        let new_frame = frame.buffer_bytes();

                        // Replace the old frame with the new
                        image.lock().unwrap().replace(new_frame);

                        // Request a rerender
                        platform.invalidate_drawing_area(size.peek().area);
                        platform.request_animation_frame();
                    } else if let Err(err) = frame {
                        error.set(Some(err));
                        break;
                    }
                }
            } else if let Err(err) = camera {
                error.set(Some(err));
            }
        });
    }));

    camera
}
