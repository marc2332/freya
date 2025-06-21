use freya_native_core::{
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        AttributeName,
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
        ParseAttribute,
        ParseError,
    },
};

#[derive(Default, Clone, Debug, Component, PartialEq)]
pub struct ScrollableState {
    pub node_id: NodeId,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub scrollables: Vec<NodeId>,
}

impl ParseAttribute for ScrollableState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        #[allow(clippy::single_match)]
        match attr.attribute {
            AttributeName::A11yRole => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                if value == "scroll-view" {
                    self.scrollables.push(self.node_id);
                }
            }
            AttributeName::A11yScrollX => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                self.scroll_x = value.parse::<f32>().unwrap();
            }
            AttributeName::A11yScrollY => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                self.scroll_y = value.parse::<f32>().unwrap();
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for ScrollableState {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::A11yRole,
            AttributeName::A11yScrollX,
            AttributeName::A11yScrollY,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let inherited_scrollable = parent.map(|(p,)| p.clone()).unwrap_or_default();

        let mut scrollable_state = ScrollableState {
            node_id: node_view.node_id(),
            scroll_x: 0.,
            scroll_y: 0.,
            ..inherited_scrollable
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                scrollable_state.parse_safe(attr);
            }
        }

        let changed = scrollable_state != *self;

        *self = scrollable_state;
        changed
    }
}
