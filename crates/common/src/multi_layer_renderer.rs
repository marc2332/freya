use std::sync::{
    Arc,
    Mutex,
};

use freya_native_core::NodeId;
use itertools::sorted;
use rustc_hash::FxHashSet;
use torin::prelude::{
    Area,
    AreaModel,
};

use crate::Layers;

#[derive(Clone, Default)]
pub struct MultiLayerRenderer {
    invalidated_nodes: Arc<Mutex<FxHashSet<NodeId>>>,
}

impl MultiLayerRenderer {
    pub fn invalidate(&self, node_id: NodeId) {
        self.invalidated_nodes.lock().unwrap().insert(node_id);
    }

    pub fn run(
        &self,
        get_children: impl Fn(NodeId) -> Vec<NodeId>,
        layers: &Layers,
        get_area: impl Fn(NodeId) -> Option<Area>,
    ) -> Layers {
        let mut invalidated_nodes = self.invalidated_nodes.lock().unwrap();
        let mut invalidated_nodes = invalidated_nodes.drain().collect::<Vec<NodeId>>();

        let mut tmp_invalidated = invalidated_nodes.clone();

        // 1. What nodes have changed? Affected children
        while !tmp_invalidated.is_empty() {
            let node_id = tmp_invalidated.pop().unwrap();

            let children = get_children(node_id);
            invalidated_nodes.extend(children.clone());
            tmp_invalidated.extend(children);
        }

        // 2. What nodes are affected by rerendering (layering)
        let rendering_layers = Layers::default();

        // from bottom to top
        for (layer, nodes) in sorted(layers.layers().iter()) {
            for node_id in nodes {
                let Some(area) = get_area(*node_id) else {
                    continue;
                };
                let is_invalidated = invalidated_nodes.contains(node_id);
                let is_area_invalidated = is_invalidated
                    || invalidated_nodes.iter().any(|invalid_node| {
                        let invalid_area = get_area(*invalid_node).unwrap();
                        invalid_area.touches_or_contains(&area) // TODO: Instead of iterating over all the elements, just accumulate their areas into 1.
                    });

                if is_area_invalidated {
                    rendering_layers.insert_node_in_layer(*node_id, *layer);
                }
            }
        }

        // 3. Render those affected nodes
        rendering_layers
    }
}
