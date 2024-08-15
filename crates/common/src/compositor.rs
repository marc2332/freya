use std::ops::{
    Deref,
    DerefMut,
};

use freya_native_core::NodeId;
use itertools::sorted;
use rustc_hash::FxHashSet;
use torin::prelude::{
    Area,
    Torin,
};

use crate::Layers;

#[derive(Clone, Default)]
pub struct CompositorDirtyNodes(FxHashSet<NodeId>);

impl Deref for CompositorDirtyNodes {
    type Target = FxHashSet<NodeId>;

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
    /// Mark a certain node as invalidated.
    pub fn invalidate(&mut self, node_id: NodeId) {
        self.0.insert(node_id);
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

impl Compositor {
    /// Run the compositor to obtain the rendering layers and the dirty area.
    pub fn run<'a>(
        &mut self,
        dirty_nodes: &mut CompositorDirtyNodes,
        dirty_area: &mut CompositorDirtyArea,
        layers: &'a Layers,
        dirty_layers: &'a mut Layers,
        layout: &Torin<NodeId>,
    ) -> &'a Layers {
        if self.full_render {
            dirty_nodes.clear();
            dirty_area.take();
            self.full_render = false;
            return layers;
        }

        let mut run_check = |layer: i16, nodes: &[NodeId]| {
            for node_id in nodes {
                let Some(area) = layout
                    .get(*node_id)
                    .map(|layout_node| layout_node.visible_area())
                else {
                    continue;
                };
                let is_invalidated = dirty_nodes.contains(node_id);
                let is_area_invalidated = dirty_area.intersects(&area);

                if is_invalidated || is_area_invalidated {
                    // Save this node to the layer it corresponds for rendering later
                    dirty_layers.insert_node_in_layer(*node_id, layer);

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

        dirty_nodes.drain();

        dirty_layers
    }

    pub fn reset(&mut self) {
        self.full_render = true;
    }
}
