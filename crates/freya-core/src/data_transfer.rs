use std::path::PathBuf;

/// A clipboard or native drag data format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataFormat {
    Files,
    Text,
    Html,
    Rtf,
    /// PNG-encoded image data.
    Image,
    /// WAV-encoded audio data.
    Audio,
}

impl DataFormat {
    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Files => "text/uri-list",
            Self::Text => "text/plain",
            Self::Html => "text/html",
            Self::Rtf => "application/rtf",
            Self::Image => "image/png",
            Self::Audio => "audio/wav",
        }
    }
}

/// A payload transferred through the clipboard or native drag-and-drop.
/// Text formats are read as strings, image and audio formats as bytes.
#[derive(Clone, Debug, PartialEq)]
pub enum DataValue {
    Files(Vec<PathBuf>),
    Text(String),
    Bytes(Vec<u8>),
}

/// Data offered in one or more formats during a transfer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransferData {
    pub formats: Vec<(DataFormat, DataValue)>,
}

impl TransferData {
    pub fn files(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        Self::default().with(
            DataFormat::Files,
            DataValue::Files(paths.into_iter().collect()),
        )
    }

    pub fn text(text: impl Into<String>) -> Self {
        Self::default().with(DataFormat::Text, DataValue::Text(text.into()))
    }

    /// Adds or replaces the data for a format.
    pub fn with(mut self, format: DataFormat, value: DataValue) -> Self {
        self.formats.retain(|(existing, _)| *existing != format);
        self.formats.push((format, value));
        self
    }
}
