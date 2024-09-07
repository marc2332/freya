use std::sync::{
    Arc,
    Mutex,
};

use accesskit::{
    NodeId as AccessibilityId,
    Role,
};
use freya_common::DirtyAccessibilityTree;
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
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
    CustomAttributeValues,
    ParseAttribute,
    ParseError,
};

#[derive(Clone, Debug, PartialEq, Eq, Default, Component)]
pub struct AccessibilityNodeState {
    pub closest_accessibility_node_id: Option<NodeId>,
    pub node_id: NodeId,
    pub accessibility_id: Option<AccessibilityId>,
    pub descencent_accessibility_ids: Vec<AccessibilityId>,
    pub role: Option<Role>,
    pub alt: Option<String>,
    pub name: Option<String>,
    pub focusable: bool,
}

impl ParseAttribute for AccessibilityNodeState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), crate::ParseError> {
        match attr.attribute {
            AttributeName::FocusId => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::AccessibilityId(id)) =
                    attr.value
                {
                    self.accessibility_id = Some(*id);
                }
            }
            AttributeName::Role => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.role = Some(
                        serde_json::from_str::<Role>(&format!("\"{attr}\""))
                            .map_err(|_| ParseError)?,
                    )
                }
            }
            AttributeName::Alt => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.alt = Some(attr.to_owned())
                }
            }
            AttributeName::Name => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.name = Some(attr.to_owned())
                }
            }
            AttributeName::Focusable => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.focusable = attr.parse().unwrap_or_default()
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for AccessibilityNodeState {
    type ParentDependencies = (Self,);

    type ChildDependencies = (Self,);

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::FocusId,
            AttributeName::Role,
            AttributeName::Alt,
            AttributeName::Name,
            AttributeName::Focusable,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let root_id = context.get::<NodeId>().unwrap();
        let dirty_accessibility_tree = context.get::<Arc<Mutex<DirtyAccessibilityTree>>>().unwrap();
        let mut accessibility = AccessibilityNodeState {
            node_id: node_view.node_id(),
            ..Default::default()
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                accessibility.parse_safe(attr);
            }
        }

        for (child,) in children {
            if let Some(child_id) = child.accessibility_id {
                // Mark this child as descendent if it has an ID
                accessibility.descencent_accessibility_ids.push(child_id)
            } else {
                // If it doesn't have an ID then use its descencent accessibility IDs
                accessibility
                    .descencent_accessibility_ids
                    .extend(child.descencent_accessibility_ids.iter());
            }
        }

        if let Some(parent) = parent {
            // Mark the parent accessibility ID as the closest to this node or
            // fallback to its closest ID.
            accessibility.closest_accessibility_node_id = parent
                .0
                .accessibility_id
                .map(|_| parent.0.node_id)
                .or(parent.0.closest_accessibility_node_id);
        }

        let changed = &accessibility != self;

        if changed {
            // Add or update this node if it is the Root or if it has an accessibility ID
            if accessibility.accessibility_id.is_some() || node_view.node_id() == *root_id {
                dirty_accessibility_tree
                    .lock()
                    .unwrap()
                    .add_or_update(node_view.node_id())
            }
        }

        *self = accessibility;
        changed
    }
}
