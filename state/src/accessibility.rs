use accesskit::{NodeId as AccessibilityId, Role};
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;

use crate::CustomAttributeValues;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessibilitySettings {
    pub focus_id: Option<AccessibilityId>,
    pub role: Option<Role>,
}

impl ParentDepState<CustomAttributeValues> for AccessibilitySettings {
    type Ctx = ();
    type DepState = (Self,);

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "focus_id", "role"
        ])));

    fn reduce(
        &mut self,
        node: NodeView<CustomAttributeValues>,
        _parent: Option<(&Self,)>,
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut focus_id = None;
        let mut role = None;

        if let Some(attributes) = node.attributes() {
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
                    _ => {}
                }
            }
        }
        let changed = false;
        *self = Self { focus_id, role };
        changed
    }
}
