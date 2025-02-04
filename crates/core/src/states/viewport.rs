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
    parsing::{
        Parse,
        ParseAttribute,
        ParseError,
    },
    values::OverflowMode,
};

#[derive(Default, PartialEq, Clone, Debug, Component)]
pub struct ViewportState {
    pub viewports: Vec<NodeId>,
    pub node_id: NodeId,
    pub overflow: OverflowMode,
}

impl ParseAttribute for ViewportState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        #[allow(clippy::single_match)]
        match attr.attribute {
            AttributeName::Overflow => {
                if let Some(value) = attr.value.as_text() {
                    self.overflow = OverflowMode::parse(value).map_err(|_| ParseError)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for ViewportState {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[AttributeName::Overflow]))
        .with_tag();

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        if !node_view.node_type().is_visible_element() {
            return false;
        }

        let mut viewports_state = ViewportState {
            node_id: node_view.node_id(),
            ..Default::default()
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                viewports_state.parse_safe(attr)
            }
        }

        if let Some((parent,)) = parent {
            viewports_state.viewports.extend(parent.viewports.clone());
            if parent.overflow == OverflowMode::Clip {
                viewports_state.viewports.push(parent.node_id);
            }
        }

        let changed = &viewports_state != self;
        *self = viewports_state;
        changed
    }
}
