use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::NodeDepState;
use dioxus_native_core_macro::sorted_str_slice;

use crate::{CanvasReference, CustomAttributeValues};

#[derive(Default, Clone, Debug)]
pub struct Transform {
    pub rotate_degs: Option<f32>,
    pub canvas: Option<CanvasReference>,
}

impl NodeDepState<CustomAttributeValues> for Transform {
    type DepState = ();
    type Ctx = ();

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "rotate", "canvas",
        ])));

    fn reduce(
        &mut self,
        node: NodeView<CustomAttributeValues>,
        _sibling: (),
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut rotate_degs = None;
        let mut canvas = None;

        if let Some(attributes) = node.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "rotate" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Ok(degs) = attr.parse::<f32>() {
                                rotate_degs = Some(degs)
                            }
                        }
                    }
                    "canvas" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Canvas(
                            new_canvas,
                        )) = attr.value
                        {
                            canvas = Some(new_canvas.clone());
                        }
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = rotate_degs != self.rotate_degs;
        *self = Self {
            rotate_degs,
            canvas,
        };
        changed
    }
}
