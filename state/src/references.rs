use dioxus_native_core::{
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;

use crate::{CanvasReference, CursorReference, CustomAttributeValues, ImageReference};

#[derive(Default, PartialEq, Clone, Debug, Component)]
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
        let mut references = References::default();

        references.cursor_ref = if let Some(parent) = parent {
            parent.0.cursor_ref.clone()
        } else {
            None
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "cursor_reference" => {
                        if let OwnedAttributeValue::Custom(
                            CustomAttributeValues::CursorReference(reference),
                        ) = attr.value
                        {
                            references.cursor_ref = Some(reference.clone());
                        }
                    }
                    "image_reference" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::ImageReference(
                            reference,
                        )) = attr.value
                        {
                            references.image_ref = Some(reference.clone());
                        }
                    }
                    "canvas_reference" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Canvas(
                            new_canvas,
                        )) = attr.value
                        {
                            references.canvas_ref = Some(new_canvas.clone());
                        }
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = &references != self;
        *self = references;
        changed
    }
}
