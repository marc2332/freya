use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::Mutex;

use bytes::Bytes;
use dioxus_core::AnyValue;
use dioxus_core::AttributeValue;
use dioxus_core::Scope;
use dioxus_native_core::node::FromAnyValue;
use freya_common::CursorLayoutResponse;
use freya_common::NodeReferenceLayout;
use tokio::sync::mpsc::UnboundedSender;

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

/// Cursor reference
#[derive(Clone, Debug)]
pub struct CursorReference {
    #[allow(clippy::type_complexity)]
    pub cursor_selections: Arc<Mutex<Option<((usize, usize), (usize, usize))>>>,
    pub cursor_position: Arc<Mutex<Option<(f32, f32)>>>,
    pub agent: UnboundedSender<CursorLayoutResponse>,
    pub id: Arc<Mutex<Option<usize>>>,
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
}

impl Debug for CustomAttributeValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reference(_) => f.debug_tuple("Reference").finish(),
            Self::CursorReference(_) => f.debug_tuple("CursorReference").finish(),
            Self::Bytes(_) => f.debug_tuple("Bytes").finish(),
            Self::ImageReference(_) => f.debug_tuple("ImageReference").finish(),
            Self::TextHighlights(_) => f.debug_tuple("TextHighlights").finish(),
        }
    }
}

impl FromAnyValue for CustomAttributeValues {
    fn from_any_value(b: &dyn AnyValue) -> Self {
        b.as_any()
            .downcast_ref::<CustomAttributeValues>()
            .unwrap()
            .clone()
    }
}

/// Transform some bytes (e.g: raw image, raw svg) into attribute data
pub fn bytes_to_data<'a, T>(cx: Scope<'a, T>, bytes: &[u8]) -> AttributeValue<'a> {
    cx.any_value(CustomAttributeValues::Bytes(bytes.to_vec()))
}
