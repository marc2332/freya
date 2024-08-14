use std::sync::{
    atomic::{
        AtomicBool,
        Ordering,
    },
    Arc,
    Mutex,
    MutexGuard,
};

use freya_engine::prelude::Canvas;
use freya_native_core::NodeId;
use itertools::sorted;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use torin::prelude::{
    Area,
    AreaModel,
    Size2D,
};

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
        canvas: &Canvas,
        get_affected: impl Fn(NodeId, bool) -> Vec<NodeId>,
        get_area: impl Fn(NodeId) -> Option<Area>,
        layers: &Layers,
    ) -> (Layers, Area) {
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
        let dirty_area = Area::from_size(Size2D::new(999., 999.));

        let full_render = self.full_render.load(Ordering::Relaxed);

        let run_check = |layer: i16, nodes: &[NodeId]| {
            for node_id in nodes {
                let Some(area) = get_area(*node_id) else {
                    continue;
                };
                let is_invalidated = full_render || invalidated_nodes.contains(node_id);
                let is_area_invalidated = is_invalidated
                    || invalidated_nodes.iter().any(|invalid_node| {
                        let Some(invalid_node_area) = get_area(*invalid_node) else {
                            return false;
                        };
                        invalid_node_area.area_intersects(&area)
                    });

                if is_area_invalidated {
                    rendering_layers.insert_node_in_layer(*node_id, layer);
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
