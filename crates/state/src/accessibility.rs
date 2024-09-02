use accesskit::NodeId as AccessibilityId;
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{
    AccessibilityOptions, CustomAttributeValues,
};

#[derive(Clone, Debug, PartialEq, Component)]
pub struct AccessibilityState {
    pub id: AccessibilityId,
    pub options: AccessibilityOptions,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for Option<AccessibilityState> {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Accessibility,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        
        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute {
                    AttributeName::ImageReference => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Accessibility(
                            id,
                            options
                        )) = attr.value
                        {
                            let accessibility = Some(AccessibilityState {
                                id: *id,
                                options: options.clone(),
                            });

                            let changed = &accessibility != self;
                            *self = accessibility;
                    
                            return changed;
                        }
                    }
                    _ => {}
                }
            }
        }

        let changed = &None != self;
        *self = None;

        changed
    }
}
