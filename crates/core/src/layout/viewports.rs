use dioxus_native_core::node::ElementNode;
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::{node::NodeType, NodeId};
use torin::prelude::Area;

use crate::layout::*;
use crate::prelude::{
    does_element_have_children_with_intrinsic_layout, does_element_have_intrinsic_layout,
};
use freya_dom::prelude::FreyaDOM;

use freya_node_state::{OverflowMode, Style};
use rustc_hash::FxHashMap;

/// Viewports of all elegible DOM elements.
#[derive(Default)]
pub struct Viewports {
    viewports: FxHashMap<NodeId, (Option<Area>, Vec<NodeId>)>,
}

impl Viewports {
    // Calculate all the applicable viewports for the given nodes
    pub fn new(layers: &Layers, fdom: &FreyaDOM) -> Self {
        let mut viewports = FxHashMap::default();
        let layout = fdom.layout();
        let rdom = fdom.rdom();

        for (_, layer) in layers.layers() {
            for node_id in layer {
                let node = rdom.get(*node_id);
                let node_areas = layout.get(*node_id);

                if let Some((node, node_areas)) = node.zip(node_areas) {
                    let node_type = &*node.node_type();

                    if let NodeType::Element(ElementNode { tag, .. }) = node_type {
                        // No need to consider text spans
                        if !does_element_have_intrinsic_layout(tag) {
                            continue;
                        }

                        let style = node.get::<Style>().unwrap();

                        // Clip any overflow from it's children
                        if style.overflow == OverflowMode::Clip {
                            let viewport = viewports
                                .entry(*node_id)
                                .or_insert_with(|| (None, Vec::new()));
                            viewport.0 = Some(node_areas.visible_area());
                        }

                        // Pass viewports to the children
                        if let Some((_, mut inherited_viewports)) = viewports.get(node_id).cloned()
                        {
                            // Only pass the inherited viewports if they are not empty
                            // or this same element has a clipped overflow
                            if !inherited_viewports.is_empty()
                                || style.overflow == OverflowMode::Clip
                            {
                                // Add itself
                                inherited_viewports.push(*node_id);

                                if does_element_have_children_with_intrinsic_layout(tag) {
                                    for child in node.children() {
                                        if let NodeType::Element(ElementNode { .. }) =
                                            &*child.node_type()
                                        {
                                            viewports
                                                .entry(child.id())
                                                .or_insert_with(|| (None, Vec::new()))
                                                .1 = inherited_viewports.clone();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Self { viewports }
    }

    pub fn get(&self, node_id: &NodeId) -> Option<&(Option<Area>, Vec<NodeId>)> {
        self.viewports.get(node_id)
    }

    pub fn size(&self) -> usize {
        self.viewports.len()
    }
}
