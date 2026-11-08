use std::{
    fmt,
    future::Future,
    path::PathBuf,
    sync::{
        Arc,
        Mutex,
        atomic::{
            AtomicBool,
            Ordering,
        },
    },
};

use futures_channel::oneshot;
use futures_util::{
    FutureExt,
    future::{
        BoxFuture,
        Shared,
    },
};
use rustc_hash::FxHashMap;
use torin::prelude::{
    Area,
    Size2D,
    Vector2D,
};

use crate::{
    element::Element,
    platform::PlatformWindow,
    prelude::current_scope_id,
    scope_id::ScopeId,
    user_event::{
        SingleThreadErasedEvent,
        UserEvent,
    },
};

/// What the drop target does with the dragged data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropAction {
    Copy,
    Move,
    Link,
}

/// A type of dragged data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DragDataType {
    Files,
    Text,
    Html,
    Rtf,
    Image,
    Audio,
}

/// Data sent during a native drag.
#[derive(Clone, Debug, PartialEq)]
pub enum DragValue {
    Files(Vec<PathBuf>),
    Text(String),
    Bytes(Vec<u8>),
}

/// Data available to other apps during a native drag.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DragData {
    pub formats: Vec<(DragDataType, DragValue)>,
}

impl DragData {
    pub fn files(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        Self::default().with(
            DragDataType::Files,
            DragValue::Files(paths.into_iter().collect()),
        )
    }

    pub fn text(text: impl Into<String>) -> Self {
        Self::default().with(DragDataType::Text, DragValue::Text(text.into()))
    }

    /// Adds or replaces the data for a format.
    pub fn with(mut self, format: DragDataType, value: DragValue) -> Self {
        self.formats.retain(|(existing, _)| *existing != format);
        self.formats.push((format, value));
        self
    }
}

/// An error when starting a native drag or reading its data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragError {
    Unsupported,
    Expired,
    Unavailable,
    InvalidData(String),
    Platform(String),
}

impl fmt::Display for DragError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported => {
                formatter.write_str("Native dragging is not supported on this platform")
            }
            Self::Expired => formatter.write_str("The drag data is no longer available"),
            Self::Unavailable => formatter.write_str("The requested data format is not available"),
            Self::InvalidData(message) | Self::Platform(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for DragError {}

type DragRead = Shared<BoxFuture<'static, Result<DragValue, DragError>>>;

/// A read request waiting for the platform to handle it.
pub struct DragReadRequest {
    pub format: DragDataType,
    pub response: oneshot::Sender<Result<DragValue, DragError>>,
}

#[derive(Default)]
struct IncomingReads {
    cached: FxHashMap<DragDataType, DragRead>,
    pending: Vec<DragReadRequest>,
}

struct IncomingData {
    formats: Vec<DragDataType>,
    reads: Mutex<IncomingReads>,
    expired: AtomicBool,
}

/// Incoming drag data, read only when requested.
#[derive(Clone)]
pub struct DragDataHandle(Arc<IncomingData>);

impl PartialEq for DragDataHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for DragDataHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DragDataHandle")
            .field("formats", &self.0.formats)
            .field("expired", &self.0.expired.load(Ordering::Relaxed))
            .finish()
    }
}

impl DragDataHandle {
    pub fn new(formats: Vec<DragDataType>) -> Self {
        Self(Arc::new(IncomingData {
            formats,
            reads: Mutex::default(),
            expired: AtomicBool::new(false),
        }))
    }

    /// Takes the read requests waiting for the platform to handle them.
    pub fn take_read_requests(&self) -> Result<Vec<DragReadRequest>, DragError> {
        let mut reads = self.0.reads.lock().map_err(|_| DragError::Expired)?;
        Ok(std::mem::take(&mut reads.pending))
    }

    pub fn types(&self) -> &[DragDataType] {
        &self.0.formats
    }

    pub fn has(&self, format: DragDataType) -> bool {
        self.types().contains(&format)
    }

    pub fn has_files(&self) -> bool {
        self.has(DragDataType::Files)
    }

    pub fn has_text(&self) -> bool {
        self.has(DragDataType::Text)
    }

    /// Stops new reads and cancels queued requests.
    /// Started reads and saved results stay available.
    pub fn expire(&self) {
        self.0.expired.store(true, Ordering::Relaxed);
        if let Ok(requests) = self.take_read_requests() {
            for request in requests {
                let _ = request.response.send(Err(DragError::Expired));
            }
        }
    }

    /// Reads a format once and reuses the result for later reads.
    pub async fn read(&self, format: DragDataType) -> Result<DragValue, DragError> {
        let read = {
            let mut reads = self.0.reads.lock().map_err(|_| DragError::Expired)?;

            if let Some(read) = reads.cached.get(&format) {
                read.clone()
            } else {
                if self.0.expired.load(Ordering::Relaxed) {
                    return Err(DragError::Expired);
                }
                if !self.has(format) {
                    return Err(DragError::Unavailable);
                }

                let (sender, receiver) = oneshot::channel();
                let read = async move { receiver.await.map_err(|_| DragError::Expired)? }
                    .boxed()
                    .shared();
                reads.cached.insert(format, read.clone());
                reads.pending.push(DragReadRequest {
                    format,
                    response: sender,
                });
                read
            }
        };

        read.await
    }

    pub async fn read_files(&self) -> Result<Vec<PathBuf>, DragError> {
        match self.read(DragDataType::Files).await? {
            DragValue::Files(paths) => Ok(paths),
            _ => Err(DragError::InvalidData("Expected file paths".into())),
        }
    }

    pub async fn read_text(&self) -> Result<String, DragError> {
        match self.read(DragDataType::Text).await? {
            DragValue::Text(text) => Ok(text),
            _ => Err(DragError::InvalidData("Expected text".into())),
        }
    }

    pub async fn read_bytes(&self, format: DragDataType) -> Result<Vec<u8>, DragError> {
        match self.read(format).await? {
            DragValue::Bytes(bytes) => Ok(bytes),
            DragValue::Text(text) => Ok(text.into_bytes()),
            _ => Err(DragError::InvalidData("Expected bytes".into())),
        }
    }
}

/// How a native drag ended. Moving data does not delete the original.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeDragResult {
    Dropped(Option<DropAction>),
    Cancelled,
}

/// A preview drawn once by Freya and shown by the operating system.
#[derive(Clone, PartialEq)]
pub enum NativeDragPreview {
    Snapshot {
        area: Area,
        offset: Vector2D,
    },
    Element {
        element: Element,
        size: Size2D,
        offset: Vector2D,
    },
}

pub struct NativeDragCommand {
    pub data: DragData,
    pub actions: Vec<DropAction>,
    pub preview: Option<NativeDragPreview>,
    pub scope_id: ScopeId,
    pub response: oneshot::Sender<Result<NativeDragResult, DragError>>,
}

pub trait NativeDragPlatformWindow {
    /// Starts a native drag while the pointer is held down and waits until it ends.
    fn start_drag(
        &self,
        data: DragData,
        actions: Vec<DropAction>,
        preview: Option<NativeDragPreview>,
    ) -> impl Future<Output = Result<NativeDragResult, DragError>>;
}

impl NativeDragPlatformWindow for PlatformWindow {
    async fn start_drag(
        &self,
        data: DragData,
        actions: Vec<DropAction>,
        preview: Option<NativeDragPreview>,
    ) -> Result<NativeDragResult, DragError> {
        let (response, receiver) = oneshot::channel();
        let command = NativeDragCommand {
            data,
            actions,
            preview,
            scope_id: current_scope_id(),
            response,
        };
        self.send(UserEvent::Erased(SingleThreadErasedEvent(Box::new(
            command,
        ))));

        receiver.await.map_err(|_| DragError::Expired)?
    }
}
