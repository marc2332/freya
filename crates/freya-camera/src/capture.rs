//! Background capture thread that drives [nokhwa] and publishes decoded frames.

use std::sync::{
    Arc,
    Mutex,
    Weak,
};

use blocking::unblock;
use bytes::Bytes;
use freya_core::notify::ArcNotify;
use nokhwa::{
    Camera,
    pixel_format::RgbAFormat,
    utils::{
        CameraFormat as NokhwaCameraFormat,
        FrameFormat,
        RequestedFormat,
        RequestedFormatType,
        Resolution,
    },
};

use crate::camera::{
    CameraConfig,
    CameraError,
    CameraFormat,
    StreamInfo,
};

/// A single decoded camera frame in `RGBA8` layout.
#[derive(Clone)]
pub struct CameraFrame {
    pub width: u32,
    pub height: u32,
    pub data: Bytes,
}

/// Latest values published by the capture thread. New entries overwrite older ones.
#[derive(Default)]
pub struct CaptureState {
    pub frame: Option<CameraFrame>,
    pub info: Option<StreamInfo>,
    pub error: Option<CameraError>,
}

/// Handle returned by [`spawn_capture`].
pub struct CaptureHandle {
    pub state: Arc<Mutex<CaptureState>>,
    pub wake: ArcNotify,
}

/// Producer side of the capture channel.
struct CaptureProducer {
    state: Weak<Mutex<CaptureState>>,
    wake: ArcNotify,
}

impl CaptureProducer {
    /// Apply `update` to the slot and wake the consumer. Returns `false` if the consumer is gone.
    fn publish(&self, update: impl FnOnce(&mut CaptureState)) -> bool {
        let Some(slot) = self.state.upgrade() else {
            return false;
        };
        update(&mut slot.lock().unwrap());
        self.wake.notify();
        true
    }
}

/// Spawn the capture thread.
pub fn spawn_capture(config: CameraConfig) -> CaptureHandle {
    let handle = CaptureHandle {
        state: Arc::new(Mutex::new(CaptureState::default())),
        wake: ArcNotify::new(),
    };

    let producer = CaptureProducer {
        state: Arc::downgrade(&handle.state),
        wake: handle.wake.clone(),
    };

    unblock(move || {
        if let Err(err) = run_capture(config, &producer) {
            producer.publish(|slot| slot.error = Some(err));
        }
    })
    .detach();

    handle
}

fn run_capture(config: CameraConfig, producer: &CaptureProducer) -> Result<(), CameraError> {
    let requested = RequestedFormat::new::<RgbAFormat>(config.format.into());
    let mut camera = Camera::new(config.device, requested)?;
    camera.open_stream()?;

    let resolution = camera.resolution();
    let info = StreamInfo {
        width: resolution.width(),
        height: resolution.height(),
        frame_rate: camera.frame_rate(),
    };

    if !producer.publish(|slot| slot.info = Some(info)) {
        return Ok(());
    }

    loop {
        let buffer = camera.frame()?;
        let decoded = buffer.decode_image::<RgbAFormat>()?;
        let new_frame = CameraFrame {
            width: decoded.width(),
            height: decoded.height(),
            data: Bytes::from(decoded.into_raw()),
        };

        if !producer.publish(|slot| slot.frame = Some(new_frame)) {
            break;
        }
    }

    Ok(())
}

impl From<CameraFormat> for RequestedFormatType {
    fn from(format: CameraFormat) -> Self {
        match format {
            CameraFormat::HighestFrameRate => Self::AbsoluteHighestFrameRate,
            CameraFormat::HighestResolution => Self::AbsoluteHighestResolution,
            CameraFormat::Resolution { width, height } => {
                Self::HighestResolution(Resolution::new(width, height))
            }
            CameraFormat::Exact {
                width,
                height,
                frame_rate,
            } => Self::Closest(NokhwaCameraFormat::new(
                Resolution::new(width, height),
                FrameFormat::MJPEG,
                frame_rate,
            )),
        }
    }
}
