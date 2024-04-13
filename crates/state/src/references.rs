use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{CanvasReference, CustomAttributeValues, ImageReference};

#[derive(Default, PartialEq, Clone, Debug, Component)]
pub struct References {
    pub image_ref: Option<ImageReference>,
    pub canvas_ref: Option<CanvasReference>,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for References {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::ImageReference,
            AttributeName::CanvasReference,
        ]))
        .with_tag();

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut references = References::default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute {
                    AttributeName::ImageReference => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::ImageReference(
                            reference,
                        )) = attr.value
                        {
                            references.image_ref = Some(reference.clone());
                        }
                    }
                    AttributeName::CanvasReference => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Canvas(
                            new_canvas,
                        )) = attr.value
                        {
                            references.canvas_ref = Some(new_canvas.clone());
                        }
                    }
                    _ => {}
                }
            }
        }

        let changed = &references != self;

        *self = references;
        changed
    }
}
