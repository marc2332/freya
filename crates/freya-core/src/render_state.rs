use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use torin::prelude::Area;

use crate::{
    damage::Damage,
    node_id::NodeId,
    render_commands::NodeRecording,
};

/// Cached recordings and pending repaint regions.
#[derive(Default)]
pub struct RenderState {
    /// Recorded render commands per node.
    pub cache: FxHashMap<NodeId, NodeRecording>,

    /// Nodes whose recording must be refreshed in the next frame.
    pub dirty: FxHashSet<NodeId>,

    /// Nodes refreshed by area invalidation without damaging their entire bounds.
    pub area_invalidated: FxHashSet<NodeId>,

    /// Nodes whose rendering can change without prop changes.
    pub volatile: FxHashSet<NodeId>,

    /// Screen regions that must repaint in the next frame.
    pub damage: Damage,

    /// The damage consumed by the last rendered frame.
    pub last_damage: Damage,
}

impl RenderState {
    pub fn invalidate_area(&mut self, area: Area) {
        if area.width() <= 0.0 || area.height() <= 0.0 {
            return;
        }
        self.damage.push(area);
        self.area_invalidated
            .extend(self.cache.iter().filter_map(|(node_id, recording)| {
                recording
                    .bounds
                    .is_none_or(|bounds| bounds.intersects(&area))
                    .then_some(*node_id)
            }));
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        self.dirty.remove(&node_id);
        self.area_invalidated.remove(&node_id);
        self.volatile.remove(&node_id);
        if let Some(recording) = self.cache.remove(&node_id) {
            self.damage.push_bounds(recording.bounds);
        }
    }

    pub fn finish_render(&mut self) {
        self.dirty.clear();
        self.area_invalidated.clear();
        self.last_damage = self.damage.take();
    }
}
