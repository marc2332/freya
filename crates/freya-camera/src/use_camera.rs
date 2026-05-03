//! [`use_camera`] hook and the [`UseCamera`] handle.

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
use futures_util::StreamExt;

use crate::{
    camera::{
        CameraConfig,
        CameraError,
        StreamInfo,
    },
    capture::{
        CameraFrame,
        CaptureMessage,
        spawn_capture,
    },
};

/// A handle to a running camera, produced by [`use_camera`] or
/// [`UseCamera::create`].
///
/// The handle is `Copy` and can be passed freely to child components, including
/// [`CameraViewer`](crate::camera_viewer::CameraViewer). The camera is closed
/// automatically when the scope where the handle was created is dropped.
#[derive(Clone, Copy, PartialEq)]
pub struct UseCamera {
    /// The latest frame produced by the camera.
    pub frame: State<Option<ImageHolder>>,
    /// The resolution and frame rate negotiated with the device.
    pub info: State<Option<StreamInfo>>,
    /// The most recent error, if any.
    pub error: State<Option<CameraError>>,
}

impl UseCamera {
    /// Open a camera and start streaming frames into reactive state.
    ///
    /// Must be called from within a Freya render context (typically through
    /// [`use_camera`]). The camera is closed when the surrounding scope is
    /// dropped.
    pub fn create(config: CameraConfig) -> Self {
        let mut frame: State<Option<ImageHolder>> = State::create(None);
        let mut info: State<Option<StreamInfo>> = State::create(None);
        let mut error: State<Option<CameraError>> = State::create(None);

        let mut receiver = spawn_capture(config);

        spawn(async move {
            while let Some(message) = receiver.next().await {
                match message {
                    CaptureMessage::Started(new_info) => {
                        *info.write() = Some(new_info);
                    }
                    CaptureMessage::Frame(camera_frame) => match build_holder(camera_frame) {
                        Ok(holder) => *frame.write() = Some(holder),
                        Err(err) => {
                            tracing::warn!("freya-camera: {err}");
                            *error.write() = Some(err);
                        }
                    },
                    CaptureMessage::Error(err) => {
                        tracing::warn!("freya-camera: {err}");
                        *error.write() = Some(err);
                    }
                }
            }
        });

        Self { frame, info, error }
    }
}

/// Open a camera and return a [`UseCamera`] handle.
///
/// The `init` closure is invoked once on mount to produce the [`CameraConfig`].
/// To switch to a different camera, recreate the component (for example by
/// changing its key) so the hook runs again.
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
pub fn use_camera(init: impl FnOnce() -> CameraConfig) -> UseCamera {
    use_hook(|| UseCamera::create(init()))
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
