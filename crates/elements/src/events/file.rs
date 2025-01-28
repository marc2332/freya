use std::path::PathBuf;

use crate::definitions::ErasedEventData;

/// Data of a Keyboard event.
#[derive(Debug, Clone, PartialEq)]
pub struct FileData {
    pub file_path: Option<PathBuf>,
}

impl From<&ErasedEventData> for FileData {
    fn from(val: &ErasedEventData) -> Self {
        val.downcast::<FileData>().cloned().unwrap()
    }
}
