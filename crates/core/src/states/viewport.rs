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
use torin::{
    prelude::Area,
    torin::Torin,
};

use crate::{
    custom_attributes::CustomAttributeValues,
    parsing::{
        Parse,
        ParseAttribute,
        ParseError,
    },
    values::{
        LayerMode,
        OverflowMode,
    },
};

#[derive(Default, PartialEq, Clone, Debug, Component)]
pub struct ViewportState {
    pub viewports: Vec<NodeId>,
    pub node_id: NodeId,
    pub overflow: OverflowMode,
}

impl ViewportState {
    pub fn is_visible(&self, layout: &Torin<NodeId>, area: &Area) -> bool {
        // Skip elements that are completely out of any their parent's viewport
        for viewport_id in &self.viewports {
            let viewport = layout.get(*viewport_id).unwrap().visible_area();
            if !viewport.intersects(area) {
                return false;
            }
        }
        true
    }
}

impl ParseAttribute for ViewportState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        match attr.attribute {
            AttributeName::Overflow => {
                self.overflow = OverflowMode::parse(attr.value.as_text().ok_or(ParseError)?)
                    .map_err(|_| ParseError)?;
            }
            AttributeName::Layer => {
                let layer = LayerMode::parse(attr.value.as_text().ok_or(ParseError)?)?;
                if layer == LayerMode::Overlay {
                    self.viewports.clear();
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
        .with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Overflow,
            AttributeName::Layer,
        ]))
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

        if let Some((parent,)) = parent {
            viewports_state.viewports.extend(parent.viewports.clone());
            if parent.overflow == OverflowMode::Clip {
                viewports_state.viewports.push(parent.node_id);
            }
        }

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                viewports_state.parse_safe(attr)
            }
        }

        let changed = &viewports_state != self;
        *self = viewports_state;
        changed
    }
}
