use std::sync::{
    atomic::{
        AtomicBool,
        Ordering,
    },
    Arc,
    Mutex,
    MutexGuard,
};

use freya_native_core::NodeId;
use itertools::sorted;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use torin::prelude::Area;

use crate::Layers;

#[derive(Clone, Default)]
pub struct CompositorDirtyNodes(Arc<Mutex<FxHashMap<NodeId, DirtyTarget>>>);

impl CompositorDirtyNodes {
    pub fn invalidate(&self, node_id: NodeId) {
        self.0.lock().unwrap().insert(node_id, DirtyTarget::Itself);
    }

    pub fn invalidate_with_target(&self, node_id: NodeId, target: DirtyTarget) {
        self.0.lock().unwrap().insert(node_id, target);
    }

    pub fn get(&self) -> MutexGuard<FxHashMap<NodeId, DirtyTarget>> {
        self.0.lock().unwrap()
    }
}

#[derive(Default)]
pub struct Compositor {
    full_render: Arc<AtomicBool>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DirtyTarget {
    Itself,
    ItselfAndNested,
}

impl Compositor {
    pub fn run(
        &self,
        dirty_nodes: &CompositorDirtyNodes,
        initial_dirty_rect: Option<Area>,
        get_affected: impl Fn(NodeId, bool) -> Vec<NodeId>,
        get_area: impl Fn(NodeId) -> Option<Area>,
        layers: &Layers,
    ) -> (Layers, Option<Area>) {
        let mut dirty_nodes = dirty_nodes.get();
        let (mut invalidated_nodes, mut dirty_nodes) = {
            (
                FxHashSet::from_iter(dirty_nodes.keys().copied()),
                dirty_nodes.drain().collect::<Vec<(NodeId, DirtyTarget)>>(),
            )
        };

        // Mark children
        while let Some((node_id, target)) = dirty_nodes.pop() {
            // Mark this node as invalidated
            invalidated_nodes.insert(node_id);

            let traverse_children = target == DirtyTarget::ItselfAndNested;
            let affected = get_affected(node_id, traverse_children)
                .into_iter()
                .filter(|id| !invalidated_nodes.contains(id));

            // Continue searching in the affected nodes
            dirty_nodes.extend(
                affected
                    .into_iter()
                    .map(|id| (id, DirtyTarget::ItselfAndNested)),
            );
        }

        let rendering_layers = Layers::default();
        let mut dirty_area: Option<Area> = initial_dirty_rect.map(|area| area.round_out());

        let full_render = self.full_render.load(Ordering::Relaxed);

        let mut run_check = |layer: i16, nodes: &[NodeId]| {
            for node_id in nodes {
                let Some(area) = get_area(*node_id) else {
                    continue;
                };
                let is_invalidated = full_render || invalidated_nodes.contains(node_id);
                let is_area_invalidated = dirty_area
                    .map(|dirty_area| dirty_area.intersects(&area))
                    .unwrap_or_default();

                if is_invalidated || is_area_invalidated {
                    // Save this node to the layer it corresponds for rendering later
                    rendering_layers.insert_node_in_layer(*node_id, layer);

                    // Expand the dirty area with only nodes who have actually changed
                    if is_invalidated {
                        if let Some(dirty_area) = &mut dirty_area {
                            *dirty_area = dirty_area.union(&area);
                        } else {
                            dirty_area = Some(area)
                        }
                    }
                }
            }
        };

        // From bottom to top
        for (layer, nodes) in sorted(layers.layers().iter()) {
            run_check(*layer, nodes);
        }

        // From top to bottom
        for (layer, nodes) in sorted(layers.layers().iter()).rev() {
            run_check(*layer, nodes);
        }

        self.full_render.store(false, Ordering::Relaxed);

        (rendering_layers, dirty_area)
    }

    pub fn reset(&self) {
        self.full_render.store(true, Ordering::Relaxed)
    }
}
