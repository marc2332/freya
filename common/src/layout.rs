use crate::Area;
use std::{
    ops::Div,
    sync::{Arc, Mutex},
};

/// Layout info of a certain Node, used by `use_node`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeReferenceLayout {
    pub area: Area,
    pub inner: Area,
}

impl NodeReferenceLayout {
    pub fn div(&mut self, rhs: f32) {
        self.area = self.area.div(rhs);
        self.inner = self.inner.div(rhs);
    }
}
pub type LayoutNotifier = Arc<Mutex<bool>>;
