use std::path::PathBuf;

/// Source for a [`VideoPlayer`].
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// # use std::path::PathBuf;
/// // From a local file
/// let source: VideoSource = PathBuf::from("./my_video.mp4").into();
///
/// // From a network stream (requires the `streaming` feature)
/// #[cfg(feature = "streaming")]
/// let stream: VideoSource = VideoSource::url("https://example.com/stream.m3u8");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum VideoSource {
    /// Load a video from a local file path.
    Path(PathBuf),
    /// Load a video from a network URL (HTTP, HLS/m3u8, RTMP, etc.).
    /// Requires FFmpeg to be compiled with the corresponding protocol support.
    #[cfg(feature = "streaming")]
    Url {
        url: String,
        /// Extra HTTP headers forwarded to FFmpeg (e.g. `Origin`, `Referer`).
        headers: Vec<(String, String)>,
    },
}

impl VideoSource {
    /// Create a [`VideoSource`] from a network URL.
    ///
    /// Supported protocols depend on the FFmpeg build (HTTP, HTTPS, HLS, RTMP, etc.).
    ///
    /// Requires the `streaming` feature.
    #[cfg(feature = "streaming")]
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url {
            url: url.into(),
            headers: Vec::new(),
        }
    }

    /// Append an HTTP header that will be sent with every request for this source.
    ///
    /// Useful for CDNs that require specific `Origin` or `Referer` values.
    /// Can be chained multiple times.
    ///
    /// Requires the `streaming` feature.
    #[cfg(feature = "streaming")]
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        if let Self::Url { ref mut headers, .. } = self {
            headers.push((name.into(), value.into()));
        }
        self
    }

    /// Returns the FFmpeg input string and any extra HTTP headers to forward.
    pub(crate) fn into_ffmpeg_input(self) -> FfmpegInput {
        match self {
            VideoSource::Path(p) => FfmpegInput {
                url: p.to_string_lossy().into_owned(),
                extra_headers: Vec::new(),
            },
            #[cfg(feature = "streaming")]
            VideoSource::Url { url, headers } => FfmpegInput {
                url,
                extra_headers: headers,
            },
        }
    }
}

/// Parsed input ready to be passed to FFmpeg.
pub(crate) struct FfmpegInput {
    pub url: String,
    pub extra_headers: Vec<(String, String)>,
}

impl From<PathBuf> for VideoSource {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl From<&str> for VideoSource {
    fn from(s: &str) -> Self {
        Self::Path(PathBuf::from(s))
    }
}

impl From<String> for VideoSource {
    fn from(s: String) -> Self {
        Self::Path(PathBuf::from(s))
    }
}

#[derive(PartialEq)]
pub(crate) enum VideoStatus {
    Loading,
    Playing,
    Paused,
    Finished,
    Error(String),
}
