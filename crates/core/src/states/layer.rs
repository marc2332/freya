use std::sync::{
    Arc,
    Mutex,
};

use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    NodeId,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{
    custom_attributes::CustomAttributeValues,
    layers::Layers,
    parsing::{
        Parse,
        ParseAttribute,
        ParseError,
    },
    values::LayerMode,
};

#[derive(Default, PartialEq, Clone, Debug, Component)]
pub struct LayerState {
    pub layer: i16,
    pub layer_for_children: i16,
}

impl ParseAttribute for LayerState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        #[allow(clippy::single_match)]
        match attr.attribute {
            AttributeName::Layer => {
                let layer = LayerMode::parse(attr.value.as_text().ok_or(ParseError)?)?;
                match layer {
                    LayerMode::Relative(relative_layer) => {
                        self.layer -= relative_layer;
                        self.layer_for_children += relative_layer;
                    }
                    LayerMode::Overlay => {
                        self.layer = i16::MAX / 2;
                        self.layer_for_children = i16::MIN / 2;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
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

        let root_id = context.get::<NodeId>().unwrap();
        let layers = context.get::<Arc<Mutex<Layers>>>().unwrap();
        let inherited_layer = parent.map(|(p,)| p.layer_for_children).unwrap_or(0i16);

        let mut layer_state = LayerState {
            layer: node_view.height() as i16 - inherited_layer,
            layer_for_children: inherited_layer,
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                layer_state.parse_safe(attr);
            }
        }

        let changed = &layer_state != self;

        let is_orphan = node_view.height() == 0 && node_view.node_id() != *root_id;

        if changed && !is_orphan {
            layers
                .lock()
                .unwrap()
                .insert_node_in_layer(node_view.node_id(), layer_state.layer);
        }

        *self = layer_state;
        changed
    }
}
