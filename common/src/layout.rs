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

impl NodeReferenceLayout {
    pub fn div(&mut self, div_n: f32) {
        self.x /= div_n;
        self.y /= div_n;
        self.width /= div_n;
        self.height /= div_n;
        self.inner_width /= div_n;
        self.inner_height /= div_n;
    }
}
pub type LayoutNotifier = Arc<Mutex<bool>>;
