use accesskit::{NodeId as AccessibilityId, Role};
use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::{
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;

use crate::CustomAttributeValues;

#[derive(Clone, Debug, PartialEq, Eq, Default, Component)]
pub struct AccessibilitySettings {
    pub focus_id: Option<AccessibilityId>,
    pub role: Option<Role>,
    pub alt: Option<String>,
    pub name: Option<String>,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for AccessibilitySettings {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            "focus_id", "role", "alt", "name",
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut focus_id = None;
        let mut role = None;
        let mut alt = None;
        let mut name = None;

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                #[allow(clippy::single_match)]
                match attr.attribute.name.as_str() {
                    "focus_id" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::FocusId(id)) =
                            attr.value
                        {
                            focus_id = Some(*id);
                        }
                    }
                    "role" => {
                        if let OwnedAttributeValue::Text(attr) = attr.value {
                            if let Ok(new_role) =
                                serde_json::from_str::<Role>(&format!("\"{attr}\""))
                            {
                                role = Some(new_role)
                            }
                        }
                    }
                    "alt" => {
                        if let OwnedAttributeValue::Text(attr) = attr.value {
                            alt = Some(attr.to_owned())
                        }
                    }
                    "name" => {
                        if let OwnedAttributeValue::Text(attr) = attr.value {
                            alt = Some(attr.to_owned())
                        }
                    }
                    _ => {}
                }
            }
        }
        let changed = self.focus_id != focus_id || self.role != role || self.alt != alt;
        *self = Self {
            focus_id,
            role,
            alt,
            name,
        };
        changed
    }
}
