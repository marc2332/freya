use crate::{Area, Size2D};
use std::ops::Div;

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
