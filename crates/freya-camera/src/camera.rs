//! Camera configuration types.

pub use nokhwa::{
    NokhwaError as CameraError,
    utils::{
        CameraIndex,
        CameraInfo,
    },
};
use nokhwa::{
    query as nokhwa_query,
    utils::ApiBackend,
};

/// Requested capture format.
///
/// The actual format used by the device may differ if the requested one is not
/// supported. Inspect [`StreamInfo`] from the running camera to see what was
/// negotiated.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum CameraFormat {
    /// Highest framerate available, any resolution.
    #[default]
    HighestFrameRate,
    /// Highest resolution available, any framerate.
    HighestResolution,
    /// Highest framerate at the given resolution.
    Resolution { width: u32, height: u32 },
    /// Closest match to the given resolution and framerate.
    Exact {
        width: u32,
        height: u32,
        frame_rate: u32,
    },
}

/// Configuration used to open a camera.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CameraConfig {
    pub device: CameraIndex,
    pub format: CameraFormat,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            device: CameraIndex::Index(0),
            format: CameraFormat::default(),
        }
    }
}

impl CameraConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn device(mut self, device: CameraIndex) -> Self {
        self.device = device;
        self
    }

    pub fn format(mut self, format: CameraFormat) -> Self {
        self.format = format;
        self
    }
}

/// Negotiated information about a running camera.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamInfo {
    pub width: u32,
    pub height: u32,
    pub frame_rate: u32,
}

/// Enumerate the cameras available on the system.
///
/// Uses the platform's native backend (V4L2 on Linux, AVFoundation on macOS,
/// MediaFoundation on Windows). On macOS make sure [`init`](crate::init) has
/// been called first so the OS authorization prompt has a chance to run.
///
/// # Example
///
/// ```rust, no_run
/// use freya::camera::*;
///
/// for device in query().unwrap_or_default() {
///     println!("{}: {}", device.human_name(), device.description());
/// }
/// ```
pub fn query() -> Result<Vec<CameraInfo>, CameraError> {
    nokhwa_query(ApiBackend::Auto)
}
