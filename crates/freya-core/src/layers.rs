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
    /// Relative layer to the parent's layer. `0` (the default) keeps the normal stacking order.
    /// Negative values move behind siblings and positive values in front.
    Relative(i16),
    /// Adds a big layer jump relative to the parents layer.
    /// You may stack multiple overlays on top of each other.
    Overlay,
    /// Paint on a specific, numbered overlay level, regardless of the parent's layer.
    /// There are up to 16 levels you can use, anything above will be capped at 16.
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

/// Nodes in each layer follow the current tree order.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Layers(FxHashMap<i16, Vec<NodeId>>);

impl Layers {
    /// Remove the node and discard empty layers.
    pub fn remove_node_from_layer(&mut self, node_id: &NodeId, layer_number: i16) {
        if let Some(entries) = self.0.get_mut(&layer_number) {
            entries.retain(|entry| entry != node_id);

            if entries.is_empty() {
                self.0.remove(&layer_number);
            }
        }
    }

    /// Reinsert affected nodes without changing the relative order of other entries.
    pub fn update_order(
        &mut self,
        nodes: &FxHashMap<NodeId, i16>,
        parents: &FxHashMap<NodeId, NodeId>,
        children: &FxHashMap<NodeId, Vec<NodeId>>,
    ) {
        let affected_layers: FxHashSet<_> = nodes.values().copied().collect();
        for layer in affected_layers {
            if let Some(entries) = self.0.get_mut(&layer) {
                entries.retain(|node_id| !nodes.contains_key(node_id));
            }
        }

        let mut sibling_positions = FxHashMap::default();
        for (&node_id, &layer) in nodes {
            let position = Self::tree_position(node_id, parents, children, &mut sibling_positions);
            let entries = self.0.entry(layer).or_default();
            let index = entries.partition_point(|other| {
                Self::tree_position(*other, parents, children, &mut sibling_positions) < position
            });
            entries.insert(index, node_id);
        }
    }

    fn tree_position(
        mut node_id: NodeId,
        parents: &FxHashMap<NodeId, NodeId>,
        children: &FxHashMap<NodeId, Vec<NodeId>>,
        sibling_positions: &mut FxHashMap<NodeId, usize>,
    ) -> Vec<usize> {
        let mut position = Vec::new();
        while let Some(parent) = parents.get(&node_id) {
            if !sibling_positions.contains_key(&node_id) {
                sibling_positions.extend(
                    children[parent]
                        .iter()
                        .enumerate()
                        .map(|(index, child)| (*child, index)),
                );
            }
            position.push(sibling_positions[&node_id]);
            node_id = *parent;
        }
        position.reverse();
        position
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
