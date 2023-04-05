use std::sync::{Arc, Mutex};

/// Layout info of a certain Node, used by `use_node`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeReferenceLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub inner_height: f32,
    pub inner_width: f32,
}

pub type LayoutNotifier = Arc<Mutex<bool>>;

/// Messages emitted from the layour to the Nodes. Used in `use_editable`.
#[derive(Debug)]
pub enum CursorLayoutResponse {
    CursorPosition { position: usize, id: usize },
    TextSelection { from: usize, to: usize, id: usize },
}
