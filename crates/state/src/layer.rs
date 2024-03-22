use std::sync::{Arc, Mutex};

use dioxus_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    NodeId, SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;
use rustc_hash::FxHashMap;

use crate::CustomAttributeValues;

#[derive(Default, PartialEq, Clone, Debug, Component)]
pub struct LayerState {
    pub provided_layer: i16,
    pub layer: i16,
    pub children_element_layer: i16,
    pub node_id: NodeId,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for LayerState {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[AttributeName::Layer]))
        .with_tag();

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let layers = context
            .get::<Arc<Mutex<FxHashMap<i16, Vec<NodeId>>>>>()
            .unwrap();
        let mut layer_state = LayerState::default();
        let inherited_relative_layer = parent.map(|(p,)| p.children_element_layer).unwrap_or(0i16);

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                #[allow(clippy::single_match)]
                match attr.attribute {
                    AttributeName::Layer => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(relative_layer) = value.parse::<i16>() {
                                layer_state.provided_layer = relative_layer;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let element_layer =
            -layer_state.provided_layer + node_view.height() as i16 - inherited_relative_layer;
        let children_element_layer = layer_state.provided_layer + inherited_relative_layer;

        layer_state.layer = element_layer;
        layer_state.children_element_layer = children_element_layer;

        let changed = &layer_state != self;

        if changed {
            let mut layers = layers.lock().unwrap();
            let layer = layers.entry(element_layer).or_default();

            layer.push(node_view.node_id());
        }

        *self = layer_state;
        changed
    }
}
