use std::{
    self,
    ops::{
        Deref,
        DerefMut,
    },
};

use freya_native_core::NodeId;
use rustc_hash::FxHashMap;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Layers(FxHashMap<i16, Vec<NodeId>>);

impl Layers {
    /// Insert the given [NodeId] in the given layer. Will create an entry for the layer if missing.
    pub fn insert_node_in_layer(&mut self, node_id: NodeId, layer_n: i16) {
        let layer = self.0.entry(layer_n).or_default();
        if layer.contains(&node_id) {
            return;
        }
        layer.push(node_id);
    }

    /// Remove the [NodeId] from the given layer. Will remove the entry of the layer if it becomes empty.
    pub fn remove_node_from_layer(&mut self, node_id: NodeId, layer_n: i16) {
        let layer = self.0.get_mut(&layer_n).unwrap();
        layer.retain(|id| *id != node_id);

        if layer.is_empty() {
            self.0.remove(&layer_n);
        }
    }
}

impl Deref for Layers {
    type Target = FxHashMap<i16, Vec<NodeId>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Layers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
