use std::cell::RefCell;

use freya_core::data_transfer::{
    DataFormat,
    DataValue,
    TransferData,
};

use crate::clipboard::{
    ClipboardError,
    ClipboardFuture,
    ClipboardProvider,
};

/// An isolated clipboard for headless renderers.
pub struct MemoryClipboard {
    contents: RefCell<TransferData>,
}

impl Default for MemoryClipboard {
    fn default() -> Self {
        Self {
            contents: RefCell::new(TransferData::text(String::new())),
        }
    }
}

impl ClipboardProvider for MemoryClipboard {
    fn get(&self, format: DataFormat) -> ClipboardFuture<DataValue> {
        let result = self
            .contents
            .borrow()
            .formats
            .iter()
            .find(|(available, _)| *available == format)
            .map(|(_, value)| value.clone())
            .ok_or(ClipboardError::UnsupportedFormat)
            .and_then(|value| match (format, value) {
                (
                    DataFormat::Text | DataFormat::Html | DataFormat::Rtf,
                    DataValue::Bytes(bytes),
                ) => String::from_utf8(bytes)
                    .map(DataValue::Text)
                    .map_err(|_| ClipboardError::FailedToRead),
                (DataFormat::Image | DataFormat::Audio, DataValue::Text(text)) => {
                    Ok(DataValue::Bytes(text.into_bytes()))
                }
                (_, value) => Ok(value),
            });
        Box::pin(async move { result })
    }

    fn set(&self, data: TransferData) -> ClipboardFuture<()> {
        let valid_formats = !data.formats.is_empty()
            && data
                .formats
                .iter()
                .all(|(format, value)| match (format, value) {
                    (DataFormat::Files, DataValue::Files(paths)) => {
                        paths.iter().all(|path| path.is_absolute())
                    }
                    (DataFormat::Files, _) | (_, DataValue::Files(_)) => false,
                    _ => true,
                });
        if !valid_formats {
            return Box::pin(async { Err(ClipboardError::FailedToSet) });
        }
        *self.contents.borrow_mut() = data;
        Box::pin(async { Ok(()) })
    }
}
