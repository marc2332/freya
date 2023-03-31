use accesskit::NodeId as NodeIdKit;
use dioxus_core::ElementId;
use dioxus_native_core::node::NodeType;
use dioxus_native_core::tree::TreeView;
use dioxus_native_core::NodeId;
use freya_common::NodeArea;
use freya_dom::{DioxusNode, FreyaDOM};
use rustc_hash::FxHashMap;

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: FxHashMap<i16, FxHashMap<NodeId, RenderData>>,
}

/// Collection of info about a specific Node to render
#[derive(Clone, Debug)]
pub struct RenderData {
    pub node_area: NodeArea,
    pub element_id: Option<ElementId>,
    pub node_id: NodeId,
    pub children: Option<Vec<NodeId>>,
}

impl RenderData {
    #[inline(always)]
    pub fn get_area(&self) -> &NodeArea {
        &self.node_area
    }

    #[inline(always)]
    pub fn get_element_id(&self) -> &Option<ElementId> {
        &self.element_id
    }

    #[inline(always)]
    pub fn get_id(&self) -> &NodeId {
        &self.node_id
    }

    #[inline(always)]
    pub fn get_children(&self) -> &Option<Vec<NodeId>> {
        &self.children
    }

    #[inline(always)]
    pub fn get_node<'a>(&'a self, rdom: &'a FreyaDOM) -> Option<&DioxusNode> {
        rdom.dom().get(self.node_id)
    }

    pub fn get_text(&self, rdom: &FreyaDOM) -> Option<String> {
        let first_child = *self.children.clone()?.get(0)?;
        let first_child_node: &DioxusNode = rdom.dom().get(first_child)?;
        if let NodeType::Text { text } = &first_child_node.node_data.node_type {
            Some(text.to_owned())
        } else {
            None
        }
    }

    pub fn get_accessibility_children(&self, rdom: &FreyaDOM) -> Option<Vec<NodeIdKit>> {
        self.children.as_ref().map(|children| {
            children
                .iter()
                .filter_map(|child| {
                    let node = rdom.dom().get(*child);
                    if let Some(node) = &node {
                        node.state.accessibility.focus_id
                    } else {
                        None
                    }
                })
                .collect::<Vec<NodeIdKit>>()
        })
    }
}

impl Layers {
    /// Given the height in the DOM of the Node, it's inherited layer from it's parent
    /// and the defined layer via the `layer` attribute,
    /// calculate it's corresponding layer and it's relative layer for it's children to inherit
    pub fn calculate_layer(
        &mut self,
        relative_layer: i16,
        height: i16,
        inherited_relative_layer: i16,
    ) -> (i16, i16) {
        let element_layer = -relative_layer + height - inherited_relative_layer;
        (element_layer, relative_layer + inherited_relative_layer)
    }

    /// Insert a Node into a layer
    pub fn add_element(
        &mut self,
        node: &DioxusNode,
        node_children: Option<Vec<NodeId>>,
        node_area: &NodeArea,
        node_layer: i16,
    ) {
        let layer = self
            .layers
            .entry(node_layer)
            .or_insert_with(FxHashMap::default);

        layer.insert(
            node.node_data.node_id,
            RenderData {
                element_id: node.node_data.element_id,
                node_id: node.node_data.node_id,
                node_area: *node_area,
                children: node_children,
            },
        );
    }
}
