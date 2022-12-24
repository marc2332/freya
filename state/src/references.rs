use std::sync::{Arc, Mutex};

use dioxus_core::exports::bumpalo::Bump;
use dioxus_core::{IntoAttributeValue, AttributeValue, AnyValueContainer};
use dioxus_hooks::UseRef;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use freya_common::NodeReferenceLayout;
use freya_elements::NodeRefWrapper;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Default, Clone, Debug)]
pub struct References {
    pub node_ref: Option<UnboundedSender<NodeReferenceLayout>>,
    pub cursor_ref: Option<CursorReference>,
}

impl ParentDepState for References {
    type Ctx = ();
    type DepState = (Self,);

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "reference",
            "cursor_reference"
        ])));

    fn reduce<'a>(
        &mut self,
        node: NodeView,
        parent: Option<(&'a Self,)>,
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut node_ref = None;
        let mut cursor_ref = if let Some(parent) = parent {
            parent.0.cursor_ref.clone()
        } else {
            None
        };

        if let Some(attributes) = node.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "reference" => {
                        if let OwnedAttributeValue::Any(v) = attr.value {
                            node_ref = v.downcast_ref::<NodeRefWrapper>().map(|v|v.0.clone());
                        }
                    }
                    "cursor_reference" => {
                        if let OwnedAttributeValue::Any(v) = attr.value {
                            cursor_ref = v.downcast_ref::<&UseRef<CursorReference>>().map(|v|v.read().clone());
                        }
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = false;
        *self = Self {
            node_ref,
            cursor_ref,
        };
        changed
    }
}

#[derive(Clone, Debug)]
pub struct CursorReference {
    pub positions: Arc<Mutex<Option<(f32, f32)>>>,
    pub agent: UnboundedSender<(usize, usize)>,
    pub id: Arc<Mutex<Option<usize>>>,
}

impl<'a> IntoAttributeValue<'a> for CursorReference {
    fn into_value(self, _bump: &'a Bump) -> AttributeValue<'a> {
        AttributeValue::Any(AnyValueContainer(Arc::new(self)))
    }
}

impl PartialEq for CursorReference {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
