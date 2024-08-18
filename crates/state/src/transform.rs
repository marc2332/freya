use std::sync::{
    Arc,
    Mutex,
};

use freya_common::CompositorDirtyNodes;
use freya_native_core::{
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        AttributeName,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{
    CustomAttributeValues,
    ParseAttribute,
    ParseError,
};

#[derive(Default, Clone, Debug, Component, PartialEq)]
pub struct TransformState {
    pub rotate_degs: Option<f32>,
}

impl ParseAttribute for TransformState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), crate::ParseError> {
        #[allow(clippy::single_match)]
        match attr.attribute {
            AttributeName::Rotate => {
                if let Some(value) = attr.value.as_text() {
                    if value.ends_with("deg") {
                        self.rotate_degs = Some(
                            value
                                .replacen("deg", "", 1)
                                .parse::<f32>()
                                .map_err(|_| ParseError)?,
                        )
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for TransformState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[AttributeName::Rotate]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let mut transform_state = TransformState::default();
        let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                transform_state.parse_safe(attr);
            }
        }

        let changed = transform_state != *self;

        if changed {
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id());
        }

        *self = transform_state;
        changed
    }
}
