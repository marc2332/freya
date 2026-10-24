use freya_engine::prelude::Recording;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use torin::prelude::Area;

use crate::{
    damage::Damage,
    node_id::NodeId,
};

/// Cached recordings and pending repaint regions.
#[derive(Default)]
pub struct PaintState {
    /// Recorded painting commands per node.
    pub cache: FxHashMap<NodeId, Recording>,

    /// Changed nodes whose recordings and full painted bounds must be refreshed.
    pub dirty_changed: FxHashSet<NodeId>,

    /// Nodes refreshed by area invalidation without damaging their entire bounds.
    pub dirty_invalidated: FxHashSet<NodeId>,

    /// Nodes refreshed and damaged on every requested frame.
    pub dirty_always: FxHashSet<NodeId>,

    /// Screen regions that must repaint in the next frame.
    pub damage: Damage,

    /// The damage consumed by the last painted frame.
    pub last_damage: Damage,
}

impl PaintState {
    /// Mark an area for repainting and refresh recordings that intersect it.
    pub fn invalidate_area(&mut self, area: Area) {
        if !cfg!(feature = "incremental-painting") {
            return;
        }
        if area.width() <= 0.0 || area.height() <= 0.0 {
            return;
        }
        self.damage.push(area);

        // Refresh intersecting recordings, including unbounded ones, without expanding damage to their full bounds.
        self.dirty_invalidated
            .extend(self.cache.iter().filter_map(|(node_id, recording)| {
                recording
                    .bounds
                    .is_none_or(|bounds| bounds.intersects(&area))
                    .then_some(*node_id)
            }));
    }

    /// Remove a node's recording and repaint the area it previously covered.
    pub fn remove_node(&mut self, node_id: NodeId) {
        if !cfg!(feature = "incremental-painting") {
            return;
        }
        self.dirty_changed.remove(&node_id);
        self.dirty_invalidated.remove(&node_id);
        self.dirty_always.remove(&node_id);
        if let Some(recording) = self.cache.remove(&node_id) {
            self.damage.push_bounds(recording.bounds);
        }
    }

    /// Clear pending invalidations and retain the damage from the completed frame.
    pub fn finish_paint(&mut self) {
        self.dirty_changed.clear();
        self.dirty_invalidated.clear();
        self.last_damage = self.damage.take();
    }
}
