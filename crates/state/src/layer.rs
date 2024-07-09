use freya_common::Layers;
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::CustomAttributeValues;

#[derive(Default, PartialEq, Clone, Debug, Component)]
pub struct LayerState {
    pub layer: i16,
    pub layer_for_children: i16,
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
        if !node_view.node_type().is_visible_element() {
            return false;
        }

        let layers = context.get::<Layers>().unwrap();
        let inherited_layer = parent.map(|(p,)| p.layer_for_children).unwrap_or(0i16);

        let mut provided_layer = 0;

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                #[allow(clippy::single_match)]
                match attr.attribute {
                    AttributeName::Layer => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(relative_layer) = value.parse::<i16>() {
                                provided_layer = relative_layer;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let layer_state = LayerState {
            layer: -provided_layer + node_view.height() as i16 - inherited_layer,
            layer_for_children: provided_layer + inherited_layer,
        };

        let changed = &layer_state != self;

        if changed {
            layers.insert_node_in_layer(node_view.node_id(), layer_state.layer);
        }

        *self = layer_state;
        changed
    }
}
