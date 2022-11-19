use std::sync::{Arc, Mutex};

use dioxus::prelude::UseRef;
use dioxus_core::AttributeValue;
use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use freya_common::NodeReferenceLayout;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Default, Clone, Debug)]
pub struct References {
    pub node_ref: Option<UnboundedSender<NodeReferenceLayout>>,
    pub cursor_ref: Option<CursorReference>,
}

impl ParentDepState for References {
    type Ctx = ();
    type DepState = Self;

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "reference",
            "cursor_reference"
        ])));

    fn reduce<'a>(
        &mut self,
        node: NodeView,
        parent: Option<&'a Self::DepState>,
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut node_ref = None;
        let mut cursor_ref = if let Some(parent) = parent {
            parent.cursor_ref.clone()
        } else {
            None
        };

        for a in node.attributes() {
            match a.name {
                "reference" => {
                    if let AttributeValue::Any(v) = a.value {
                        let r: &UseRef<UnboundedSender<NodeReferenceLayout>> =
                            v.value.downcast_ref().unwrap();
                        node_ref = Some(r.read().clone())
                    }
                }
                "cursor_reference" => {
                    if let AttributeValue::Any(v) = a.value {
                        let r: &UseRef<CursorReference> = v.value.downcast_ref().unwrap();
                        cursor_ref = Some(r.read().clone())
                    }
                }
                _ => {
                    println!("Unsupported attribute <{}>", a.name);
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

impl PartialEq for CursorReference {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
