//! Background capture thread that drives [nokhwa] and forwards decoded frames
//! to the UI thread.

use async_channel::{
    Receiver,
    Sender,
    bounded,
};
use blocking::unblock;
use bytes::Bytes;
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

/// Messages sent from the capture thread to the consumer.
pub enum CaptureMessage {
    Started(StreamInfo),
    Frame(CameraFrame),
    Error(CameraError),
}

const CHANNEL_CAPACITY: usize = 2;

/// Spawn a thread that opens the camera and streams decoded frames back.
///
/// The thread terminates when the returned receiver is dropped.
pub fn spawn_capture(config: CameraConfig) -> Receiver<CaptureMessage> {
    let (tx, rx) = bounded(CHANNEL_CAPACITY);

    unblock(move || {
        if let Err(err) = run_capture(config, &tx) {
            let _ = tx.send_blocking(CaptureMessage::Error(err));
        }
    })
    .detach();

    rx
}

fn run_capture(config: CameraConfig, tx: &Sender<CaptureMessage>) -> Result<(), CameraError> {
    let requested = RequestedFormat::new::<RgbAFormat>(config.format.into());

    let mut camera = Camera::new(config.device, requested)?;
    camera.open_stream()?;

    let resolution = camera.resolution();
    let info = StreamInfo {
        width: resolution.width(),
        height: resolution.height(),
        frame_rate: camera.frame_rate(),
    };

    if tx.send_blocking(CaptureMessage::Started(info)).is_err() {
        return Ok(());
    }

    loop {
        let buffer = camera.frame()?;
        let decoded = buffer.decode_image::<RgbAFormat>()?;

        let width = decoded.width();
        let height = decoded.height();
        let frame = CameraFrame {
            width,
            height,
            data: Bytes::from(decoded.into_raw()),
        };

        if tx.send_blocking(CaptureMessage::Frame(frame)).is_err() {
            // Receiver was dropped, stop capturing.
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
