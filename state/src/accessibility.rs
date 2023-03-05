use accesskit::NodeId as NodeIdKit;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;

use crate::CustomAttributeValues;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccessibilitySettings {
    pub accessibility_id: Option<NodeIdKit>,
}

impl ParentDepState<CustomAttributeValues> for AccessibilitySettings {
    type Ctx = ();
    type DepState = (Self,);

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "accessibility_id",
        ])));

    fn reduce(
        &mut self,
        node: NodeView<CustomAttributeValues>,
        _parent: Option<(&Self,)>,
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut accessibility_id = None;

        if let Some(attributes) = node.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "accessibility_id" => {
                        if let OwnedAttributeValue::Custom(
                            CustomAttributeValues::AccessibilityId(id),
                        ) = attr.value
                        {
                            accessibility_id = Some(*id);
                        }
                    }
                    _ => {}
                }
            }
        }
        let changed = false;
        *self = Self { accessibility_id };
        changed
    }
}
