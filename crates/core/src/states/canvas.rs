use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::{
        NodeType,
        OwnedAttributeValue,
    },
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    tags::TagName,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::custom_attributes::{
    CanvasReference,
    CustomAttributeValues,
};

#[derive(Default, PartialEq, Clone, Debug, Component)]
pub struct CanvasState {
    pub canvas_ref: Option<CanvasReference>,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for CanvasState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::CanvasReference,
        ]))
        .with_tag();

    fn allow_node(node_type: &NodeType<CustomAttributeValues>) -> bool {
        node_type.tag() == Some(&TagName::Rect)
    }

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut canvas = CanvasState::default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                #[allow(clippy::single_match)]
                match attr.attribute {
                    AttributeName::CanvasReference => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Canvas(
                            new_canvas,
                        )) = attr.value
                        {
                            canvas.canvas_ref = Some(new_canvas.clone());
                        }
                    }
                    _ => {}
                }
            }
        }

        let changed = &canvas != self;

        *self = canvas;
        changed
    }
}
