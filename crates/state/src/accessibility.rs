use std::sync::{
    Arc,
    Mutex,
};

use accesskit::{
    NodeId as AccessibilityId,
    Role,
};
use freya_common::{
    AccessibilityDirtyNodes,
    AccessibilityGenerator,
};
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
    Focusable,
    Parse,
    ParseAttribute,
    ParseError,
};

#[derive(Clone, Debug, PartialEq, Eq, Default, Component)]
pub struct AccessibilityNodeState {
    pub closest_accessibility_node_id: Option<NodeId>,
    pub descencent_accessibility_ids: Vec<AccessibilityId>,
    pub node_id: NodeId,
    pub a11y_id: Option<AccessibilityId>,
    pub a11y_role: Option<Role>,
    pub a11y_alt: Option<String>,
    pub a11y_name: Option<String>,
    pub a11y_auto_focus: bool,
    pub a11y_focusable: Focusable,
}

impl ParseAttribute for AccessibilityNodeState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), crate::ParseError> {
        match attr.attribute {
            AttributeName::A11YId => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::AccessibilityId(id)) =
                    attr.value
                {
                    self.a11y_id = Some(*id);

                    // Enable focus on nodes that pass a custom a11y id
                    if self.a11y_focusable.is_unknown() {
                        self.a11y_focusable = Focusable::Enabled;
                    }
                }
            }
            AttributeName::A11YRole => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.a11y_role = Some(
                        serde_json::from_str::<Role>(&format!("\"{attr}\""))
                            .map_err(|_| ParseError)?,
                    )
                }
            }
            AttributeName::A11YAlt => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.a11y_alt = Some(attr.to_owned())
                }
            }
            AttributeName::A11YName => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.a11y_name = Some(attr.to_owned())
                }
            }
            AttributeName::A11YAutoFocus => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.a11y_auto_focus = attr.parse().unwrap_or_default()
                }
            }
            AttributeName::A11YFocusable => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.a11y_focusable = Focusable::parse(attr)?;
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
            AttributeName::A11YId,
            AttributeName::A11YRole,
            AttributeName::A11YAlt,
            AttributeName::A11YName,
            AttributeName::A11YAutoFocus,
            AttributeName::A11YFocusable,
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
        let accessibility_dirty_nodes = context
            .get::<Arc<Mutex<AccessibilityDirtyNodes>>>()
            .unwrap();
        let accessibility_generator = context.get::<Arc<AccessibilityGenerator>>().unwrap();
        let mut accessibility = AccessibilityNodeState {
            node_id: node_view.node_id(),
            a11y_id: self.a11y_id,
            ..Default::default()
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                accessibility.parse_safe(attr);
            }
        }

        for (child,) in children {
            if let Some(child_id) = child.a11y_id {
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
                .a11y_id
                .map(|_| parent.0.node_id)
                .or(parent.0.closest_accessibility_node_id);
        }

        let changed = &accessibility != self;
        let had_id = self.a11y_id.is_some();

        *self = accessibility;

        if changed {
            // Assign an accessibility ID if none was passed but the node has a role
            if self.a11y_id.is_none() && self.a11y_role.is_some() {
                let id = AccessibilityId(accessibility_generator.new_id());
                #[cfg(debug_assertions)]
                tracing::info!("Assigned {id:?} to {:?}", node_view.node_id());

                self.a11y_id = Some(id);
            }

            let was_just_created = !had_id && self.a11y_id.is_some();

            // Add or update this node if it is the Root or if it has an accessibility ID
            if self.a11y_id.is_some() || node_view.node_id() == *root_id {
                accessibility_dirty_nodes
                    .lock()
                    .unwrap()
                    .add_or_update(node_view.node_id())
            }

            if was_just_created && self.a11y_auto_focus {
                #[cfg(debug_assertions)]
                tracing::info!("Requested auto focus for {:?}", self.a11y_id.unwrap());

                accessibility_dirty_nodes
                    .lock()
                    .unwrap()
                    .request_focus(node_view.node_id())
            }
        }

        changed
    }
}
