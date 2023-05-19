use std::any::Any;
use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::Mutex;

use bytes::Bytes;
use dioxus_core::AttributeValue;
use dioxus_core::Scope;
use dioxus_native_core::node::FromAnyValue;
use freya_common::CursorLayoutResponse;
use freya_common::NodeReferenceLayout;
use skia_safe::Canvas;
use tokio::sync::mpsc::UnboundedSender;
use torin::geometry::{Area, CursorPoint};
use uuid::Uuid;

/// Image Reference
#[derive(Clone, Debug)]
pub struct ImageReference(pub Arc<Mutex<Option<Bytes>>>);

impl PartialEq for ImageReference {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Display for ImageReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageReference").finish_non_exhaustive()
    }
}

/// Node Reference
#[derive(Clone)]
pub struct NodeReference(pub UnboundedSender<NodeReferenceLayout>);

impl PartialEq for NodeReference {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Display for NodeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeReference").finish_non_exhaustive()
    }
}

type CanvasRunner = dyn Fn(&mut Canvas, Area);

/// Canvas Reference
#[derive(Clone)]
pub struct CanvasReference {
    pub runner: Arc<Box<CanvasRunner>>,
}

impl PartialEq for CanvasReference {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Debug for CanvasReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CanvasReference").finish_non_exhaustive()
    }
}

unsafe impl Sync for CanvasReference {}
unsafe impl Send for CanvasReference {}

/// Cursor reference
#[derive(Clone, Debug)]
pub struct CursorReference {
    pub text_id: Uuid,
    #[allow(clippy::type_complexity)]
    pub cursor_selections: Arc<Mutex<Option<(CursorPoint, CursorPoint)>>>,
    pub cursor_position: Arc<Mutex<Option<CursorPoint>>>,
    pub agent: UnboundedSender<CursorLayoutResponse>,
    pub cursor_id: Arc<Mutex<Option<usize>>>,
}

impl CursorReference {
    pub fn set_cursor_selections(&self, cursor_selections: Option<(CursorPoint, CursorPoint)>) {
        *self.cursor_selections.lock().unwrap() = cursor_selections;
    }

    pub fn set_cursor_position(&self, cursor_position: Option<CursorPoint>) {
        *self.cursor_position.lock().unwrap() = cursor_position;
    }

    pub fn set_id(&self, id: Option<usize>) {
        *self.cursor_id.lock().unwrap() = id;
    }
}

impl PartialEq for CursorReference {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Display for CursorReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CursorReference").finish_non_exhaustive()
    }
}

/// Group all the custom attribute types
#[derive(Clone, PartialEq)]
pub enum CustomAttributeValues {
    Reference(NodeReference),
    CursorReference(CursorReference),
    Bytes(Vec<u8>),
    ImageReference(ImageReference),
    TextHighlights(Vec<(usize, usize)>),
    Canvas(CanvasReference),
}

impl Debug for CustomAttributeValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reference(_) => f.debug_tuple("Reference").finish(),
            Self::CursorReference(_) => f.debug_tuple("CursorReference").finish(),
            Self::Bytes(_) => f.debug_tuple("Bytes").finish(),
            Self::ImageReference(_) => f.debug_tuple("ImageReference").finish(),
            Self::TextHighlights(_) => f.debug_tuple("TextHighlights").finish(),
            Self::Canvas(_) => f.debug_tuple("Canvas").finish(),
        }
    }
}

impl FromAnyValue for CustomAttributeValues {
    fn from_any_value(b: &dyn Any) -> Self {
        b.downcast_ref::<CustomAttributeValues>().unwrap().clone()
    }
}

/// Transform some bytes (e.g: raw image, raw svg) into attribute data
pub fn bytes_to_data<'a, T>(cx: Scope<'a, T>, bytes: &[u8]) -> AttributeValue<'a> {
    cx.any_value(CustomAttributeValues::Bytes(bytes.to_vec()))
}
