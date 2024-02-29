use std::path::PathBuf;

use crate::definitions::PlatformEventData;

/// Data of a Keyboard event.
#[derive(Debug, Clone, PartialEq)]
pub struct FileData {
    pub file_path: Option<PathBuf>,
}

impl From<&PlatformEventData> for FileData {
    fn from(val: &PlatformEventData) -> Self {
        val.downcast::<FileData>().cloned().unwrap()
    }
}
