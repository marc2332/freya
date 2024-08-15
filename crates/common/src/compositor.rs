use std::ops::{
    Deref,
    DerefMut,
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
pub struct CompositorDirtyNodes(FxHashMap<NodeId, DirtyTarget>);

impl Deref for CompositorDirtyNodes {
    type Target = FxHashMap<NodeId, DirtyTarget>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompositorDirtyNodes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CompositorDirtyNodes {
    /// Mark a certain node as invalidated. Uses [DirtyTarget::Itself] by default.
    pub fn invalidate(&mut self, node_id: NodeId) {
        self.0.insert(node_id, DirtyTarget::Itself);
    }

    /// Mark a certain node as invalidated with the given [DirtyTarget].
    pub fn invalidate_with_target(&mut self, node_id: NodeId, target: DirtyTarget) {
        self.0.insert(node_id, target);
    }
}

#[derive(Clone, Default)]
pub struct CompositorDirtyArea(Option<Area>);

impl CompositorDirtyArea {
    /// Take the area, leaving nothing behind.
    pub fn take(&mut self) -> Option<Area> {
        self.0.take()
    }

    /// Unite the area or insert it if none is yet present.
    pub fn unite_or_insert(&mut self, other: &Area) {
        if let Some(dirty_area) = &mut self.0 {
            *dirty_area = dirty_area.union(other);
        } else {
            self.0 = Some(*other);
        }
    }

    /// Round the dirty area to the out bounds to prevent float pixel issues.
    pub fn round_out(&mut self) {
        if let Some(dirty_area) = &mut self.0 {
            *dirty_area = dirty_area.round_out();
        }
    }

    pub fn intersects(&self, other: &Area) -> bool {
        self.0
            .map(|dirty_area| dirty_area.intersects(other))
            .unwrap_or_default()
    }
}

impl Deref for CompositorDirtyArea {
    type Target = Option<Area>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub struct Compositor {
    full_render: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DirtyTarget {
    Itself,
    ItselfAndNested,
}

impl Compositor {
    /// Run the compositor to obtain the rendering layers and the dirty area.
    pub fn run<'a>(
        &mut self,
        dirty_nodes: &mut CompositorDirtyNodes,
        dirty_area: &mut CompositorDirtyArea,
        get_affected: impl Fn(NodeId, bool) -> Vec<NodeId>,
        get_area: impl Fn(NodeId) -> Option<Area>,
        layers: &'a Layers,
        rendering_layers: &'a mut Layers,
    ) -> &'a Layers {
        if self.full_render {
            dirty_nodes.clear();
            dirty_area.take();
            return layers;
        }

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

        let mut run_check = |layer: i16, nodes: &[NodeId]| {
            for node_id in nodes {
                let Some(area) = get_area(*node_id) else {
                    continue;
                };
                let is_invalidated = invalidated_nodes.contains(node_id);
                let is_area_invalidated = dirty_area.intersects(&area);

                if is_invalidated || is_area_invalidated {
                    // Save this node to the layer it corresponds for rendering later
                    rendering_layers.insert_node_in_layer(*node_id, layer);

                    // Expand the dirty area with only nodes who have actually changed
                    if is_invalidated {
                        dirty_area.unite_or_insert(&area);
                    }
                }
            }
        };

        // From bottom to top
        for (layer, nodes) in sorted(layers.iter()) {
            run_check(*layer, nodes);
        }

        // From top to bottom
        for (layer, nodes) in sorted(layers.iter()).rev() {
            run_check(*layer, nodes);
        }

        self.full_render = false;

        rendering_layers
    }

    pub fn reset(&mut self) {
        self.full_render = true;
    }
}
