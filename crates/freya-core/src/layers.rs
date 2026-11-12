use std::ops::{
    Deref,
    DerefMut,
};

use rustc_hash::{
    FxHashMap,
    FxHashSet,
};

use crate::node_id::NodeId;

/// Controls the painting order of an element and the base layer of its descendants.
///
/// Higher layers paint on top of lower layers, regardless of their position in the tree.
/// Elements on the same layer have no guaranteed painting order.
///
/// The default is [`Layer::Relative(0)`]. An `i16` converts into a
/// [`Layer::Relative`] offset, so `.layer(1)` is equivalent to
/// `.layer(Layer::Relative(1))`.
///
/// Overlay variants also escape clipping inherited from ancestors. Use them for
/// content such as menus, tooltips and dialogs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layer {
    /// Offsets the normal layer, which is one above the parent's layer.
    ///
    /// `0` preserves this default. Among siblings, negative offsets paint below
    /// the default layer and positive offsets paint above it. `-1` places an
    /// element on its parent's layer, not below it. Descendants use the resulting
    /// layer as their base.
    Relative(i16),
    /// Raises the element by `i16::MAX / 16` layers relative to its parent.
    ///
    /// Nested overlays each add this jump.
    /// Also clears inherited clipping, allowing the element and its descendants to
    /// paint outside ancestor bounds.
    Overlay,
    /// Sets an absolute overlay level, independent of the parent's layer.
    ///
    /// Levels `1` through `16` are spaced by `i16::MAX / 16`.
    /// A nested overlay level can therefore paint below its parent.
    /// Like [`Layer::Overlay`], this clears inherited clipping.
    OverlayLevel(u8),
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
