use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use freya_common::NodeReferenceLayout;
use tokio::sync::mpsc::UnboundedSender;

use crate::{CursorReference, CustomAttributeValues, ImageReference};

#[derive(Default, Clone, Debug)]
pub struct References {
    pub image_ref: Option<ImageReference>,
    pub node_ref: Option<UnboundedSender<NodeReferenceLayout>>,
    pub cursor_ref: Option<CursorReference>,
}

impl ParentDepState<CustomAttributeValues> for References {
    type Ctx = ();
    type DepState = (Self,);

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "reference",
            "cursor_reference",
            "image_reference"
        ])));

    fn reduce(
        &mut self,
        node: NodeView<CustomAttributeValues>,
        parent: Option<(&Self,)>,
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut node_ref = None;
        let mut cursor_ref = if let Some(parent) = parent {
            parent.0.cursor_ref.clone()
        } else {
            None
        };
        let mut image_ref = None;

        if let Some(attributes) = node.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "reference" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Reference(
                            reference,
                        )) = attr.value
                        {
                            node_ref = Some(reference.0.clone());
                        }
                    }
                    "cursor_reference" => {
                        if let OwnedAttributeValue::Custom(
                            CustomAttributeValues::CursorReference(reference),
                        ) = attr.value
                        {
                            cursor_ref = Some(reference.clone());
                        }
                    }
                    "image_reference" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::ImageReference(
                            reference,
                        )) = attr.value
                        {
                            image_ref = Some(reference.clone());
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
            image_ref,
        };
        changed
    }
}
