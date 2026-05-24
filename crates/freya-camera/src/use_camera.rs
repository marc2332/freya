//! [`use_camera`] hook and the [`Camera`] handle.

use std::{
    cell::RefCell,
    rc::Rc,
};

use freya_core::{
    elements::image::ImageHolder,
    prelude::*,
};
use freya_engine::prelude::{
    AlphaType,
    ColorType,
    Data,
    ISize,
    ImageInfo,
    raster_from_data,
};

use crate::{
    camera::{
        CameraConfig,
        CameraError,
        StreamInfo,
    },
    capture::{
        CameraFrame,
        CaptureHandle,
        CaptureState,
        spawn_capture,
    },
};

/// Handle to a running camera. Closed when its owning scope is dropped.
#[derive(Clone, Copy, PartialEq)]
pub struct Camera {
    /// The latest frame produced by the camera.
    pub frame: State<Option<ImageHolder>>,
    /// The resolution and frame rate negotiated with the device.
    pub info: State<Option<StreamInfo>>,
    /// The most recent error, if any.
    pub error: State<Option<CameraError>>,
}

impl Camera {
    /// Open a camera and start streaming frames into reactive state.
    pub fn create(config: CameraConfig) -> Self {
        let mut frame: State<Option<ImageHolder>> = State::create(None);
        let mut info: State<Option<StreamInfo>> = State::create(None);
        let mut error: State<Option<CameraError>> = State::create(None);

        let CaptureHandle { state, wake } = spawn_capture(config);

        spawn(async move {
            loop {
                wake.notified().await;

                let CaptureState {
                    frame: latest_frame,
                    info: latest_info,
                    error: latest_error,
                } = std::mem::take(&mut *state.lock().unwrap());

                if let Some(stream_info) = latest_info {
                    *info.write() = Some(stream_info);
                }
                if let Some(capture_error) = latest_error {
                    tracing::warn!("freya-camera: {capture_error}");
                    *error.write() = Some(capture_error);
                }
                if let Some(camera_frame) = latest_frame {
                    match build_holder(camera_frame) {
                        Ok(holder) => *frame.write() = Some(holder),
                        Err(build_error) => {
                            tracing::warn!("freya-camera: {build_error}");
                            *error.write() = Some(build_error);
                        }
                    }
                }
            }
        });

        Self { frame, info, error }
    }
}

/// Open a camera and return a [`Camera`] handle. `init` runs once on mount.
///
/// # Example
///
/// ```rust, no_run
/// use freya::{
///     camera::*,
///     prelude::*,
/// };
///
/// fn app() -> impl IntoElement {
///     let camera = use_camera(CameraConfig::default);
///
///     rect().center().expanded().child(CameraViewer::new(camera))
/// }
/// ```
pub fn use_camera(init: impl FnOnce() -> CameraConfig) -> Camera {
    use_hook(|| Camera::create(init()))
}

/// Build an [`ImageHolder`] from a raw `RGBA8` camera frame.
fn build_holder(frame: CameraFrame) -> Result<ImageHolder, CameraError> {
    let CameraFrame {
        width,
        height,
        data,
    } = frame;

    let info = ImageInfo::new(
        ISize::new(width as i32, height as i32),
        ColorType::RGBA8888,
        AlphaType::Opaque,
        None,
    );
    let row_bytes = (width as usize) * 4;
    // Safety: `data` outlives the SkImage via `ImageHolder.bytes` below.
    let sk_data = unsafe { Data::new_bytes(&data) };
    let image = raster_from_data(&info, sk_data, row_bytes)
        .ok_or_else(|| CameraError::GeneralError("failed to create raster image".to_string()))?;

    Ok(ImageHolder {
        image: Rc::new(RefCell::new(image)),
        bytes: data,
    })
}
