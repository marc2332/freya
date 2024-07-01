use accesskit::{
    NodeId as AccessibilityId,
    Role,
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
    pub accessibility_id: Option<AccessibilityId>,
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
    type ParentDependencies = ();

    type ChildDependencies = ();

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
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut accessibility = AccessibilityNodeState::default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                accessibility.parse_safe(attr);
            }
        }
        let changed = &accessibility != self;

        *self = accessibility;
        changed
    }
}
