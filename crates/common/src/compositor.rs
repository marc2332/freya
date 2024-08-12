use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
        Mutex,
        MutexGuard,
    },
};

use freya_engine::prelude::{
    Canvas,
    Surface,
};
use freya_native_core::NodeId;
use itertools::sorted;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};

use crate::Layers;

#[derive(PartialEq)]
pub enum LayerState {
    NeedsRender,
    Ignore,
}

pub type RenderingLayers = HashMap<i16, (Surface, LayerState)>;

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
    pub rendering_layers: RefCell<RenderingLayers>,
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
        layers: &Layers,
    ) {
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

        let mut rendering_layers = self.rendering_layers.borrow_mut();

        // From bottom to top
        for (layer, nodes) in sorted(layers.layers().iter()) {
            let rendering_layer = rendering_layers.entry(*layer).or_insert_with(|| {
                // Create an individual surface for every layer
                let mut surface = unsafe { canvas.surface().unwrap() };
                let new_surface = surface
                    .new_surface_with_dimensions(canvas.base_layer_size())
                    .unwrap();

                (new_surface, LayerState::Ignore)
            });

            // Reset the state
            rendering_layer.1 = LayerState::Ignore;

            let dirty_layer = self.full_render.load(Ordering::Relaxed)
                || nodes
                    .iter()
                    .any(|node_id| invalidated_nodes.contains(node_id));

            if dirty_layer {
                // Mark this layer as NeedsRender if any of its nodes has been affected
                rendering_layer.1 = LayerState::NeedsRender;
            }
        }

        self.full_render.store(false, Ordering::Relaxed);
    }

    pub fn reset_invalidated_layers(&self) {
        self.rendering_layers
            .borrow_mut()
            .values_mut()
            .for_each(|(_, state)| *state = LayerState::Ignore);
    }

    pub fn reset(&self) {
        self.rendering_layers.borrow_mut().clear();
        self.full_render.store(true, Ordering::Relaxed)
    }

    pub fn remove_layer(&self, layer_n: i16) {
        let mut layers = self.rendering_layers.borrow_mut();
        layers.remove(&layer_n);
    }
}
