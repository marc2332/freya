use std::ops::{
    Deref,
    DerefMut,
};

use rustc_hash::{
    FxHashMap,
    FxHashSet,
};

use crate::node_id::NodeId;

/// The painting layer of an element, controlling what it draws on top of.
///
/// Converts from an `i16`, which becomes a [`Layer::Relative`] offset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layer {
    /// Offset from the parent's layer. `0` (the default) keeps the normal stacking order;
    /// negative values move behind siblings and positive values in front.
    Relative(i16),
    /// Paint far above everything else, for overlays such as popups and tooltips.
    Overlay,
    /// Paint on an overlay layer offset by the given multiplier, to stack overlays among themselves.
    RelativeOverlay(u8),
}

impl Default for Layer {
    fn default() -> Self {
        Layer::Relative(0)
    }
}

impl From<i16> for Layer {
    fn from(value: i16) -> Self {
        Layer::Relative(value)
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Layers(FxHashMap<i16, FxHashSet<NodeId>>);

impl Layers {
    /// Insert the given [NodeId] in the given layer. Will create an entry for the layer if missing.
    pub fn insert_node_in_layer(&mut self, node_id: NodeId, layer_n: i16) {
        let layer = self.0.entry(layer_n).or_default();
        layer.insert(node_id);
    }

    /// Remove the [NodeId] from the given layer. Will remove the entry of the layer if it becomes empty.
    pub fn remove_node_from_layer(&mut self, node_id: &NodeId, layer_n: i16) {
        let layer = self.0.get_mut(&layer_n);
        if let Some(layer) = layer {
            layer.remove(node_id);

            if layer.is_empty() {
                self.0.remove(&layer_n);
            }
        }
    }
}

impl Deref for Layers {
    type Target = FxHashMap<i16, FxHashSet<NodeId>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Layers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
