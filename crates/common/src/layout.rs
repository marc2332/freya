use freya_engine::prelude::Paragraph;
use std::ops::Div;
use torin::geometry::{Area, Size2D};

/// Layout info of a certain Node, used by `use_node`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeReferenceLayout {
    pub area: Area,
    pub inner: Size2D,
}

impl NodeReferenceLayout {
    pub fn div(&mut self, rhs: f32) {
        self.area = self.area.div(rhs);
        self.inner = self.inner.div(rhs);
    }
}

/// Messages emitted from the layout library to the Nodes. Used in `use_editable`.
#[derive(Debug)]
pub enum CursorLayoutResponse {
    CursorPosition { position: usize, id: usize },
    TextSelection { from: usize, to: usize, id: usize },
}

pub struct CachedParagraph(pub Paragraph);

/// # Safety
/// Skia `Paragraph` are neither Sync or Send, but in order to store them in the Associated
/// data of the Nodes in Torin (which will be used across threads when making the attributes diffing),
/// we must manually mark the Paragraph as Send and Sync, this is fine because `Paragraph`s will only be accessed and modified
/// In the main thread when measuring the layout and painting.
unsafe impl Send for CachedParagraph {}
unsafe impl Sync for CachedParagraph {}
