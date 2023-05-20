use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::NodeView;
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::SendAnyMap;
use dioxus_native_core_macro::partial_derive_state;

use crate::{CanvasReference, CursorReference, CustomAttributeValues, ImageReference};

#[derive(Default, Clone, Debug, Component)]
pub struct References {
    pub image_ref: Option<ImageReference>,
    pub cursor_ref: Option<CursorReference>,
    pub canvas_ref: Option<CanvasReference>,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for References {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            "cursor_reference",
            "image_reference",
            "canvas_reference",
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut cursor_ref = if let Some(parent) = parent {
            parent.0.cursor_ref.clone()
        } else {
            None
        };
        let mut image_ref = None;
        let mut canvas_ref = None;

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
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
                    "canvas_reference" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Canvas(
                            new_canvas,
                        )) = attr.value
                        {
                            canvas_ref = Some(new_canvas.clone());
                        }
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = cursor_ref != self.cursor_ref
            || image_ref != self.image_ref
            || canvas_ref != self.canvas_ref;
        *self = Self {
            cursor_ref,
            image_ref,
            canvas_ref,
        };
        changed
    }
}
