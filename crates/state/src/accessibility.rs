use accesskit::{NodeId as AccessibilityId, Role};
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    tags::TagName,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{AccessibilityOptions, CustomAttributeValues};

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

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[AttributeName::Accessibility]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut accessibility_state: Option<AccessibilityState> = None;
        let mut image_alt: Option<String> = None;

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute {
                    AttributeName::Accessibility => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Accessibility(
                            state,
                        )) = attr.value
                        {
                            accessibility_state = Some(state.clone());
                        } else if let Some(tag) = node_view.tag() {
                            match tag {
                                TagName::Image => {
                                    accessibility_state = Some(AccessibilityState {
                                        options: AccessibilityOptions {
                                            role: Role::Image,
                                            ..Default::default()
                                        },
                                        id: AccessibilityId(todo!()),
                                    });
                                }
                                TagName::Paragraph => {
                                    accessibility_state = Some(AccessibilityState {
                                        options: AccessibilityOptions {
                                            role: Role::Paragraph,
                                            ..Default::default()
                                        },
                                        id: AccessibilityId(todo!()),
                                    });
                                }
                                TagName::Rect => {
                                    accessibility_state = Some(AccessibilityState {
                                        options: AccessibilityOptions {
                                            role: Role::GenericContainer,
                                            ..Default::default()
                                        },
                                        id: AccessibilityId(todo!()),
                                    });
                                }
                                TagName::Svg => {
                                    accessibility_state = Some(AccessibilityState {
                                        options: AccessibilityOptions {
                                            role: Role::GraphicsObject,
                                            ..Default::default()
                                        },
                                        id: AccessibilityId(todo!()),
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                    AttributeName::Alt => {
                        if let Some(alt) = attr.value.as_text() {
                            image_alt = Some(alt.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(alt) = image_alt {
            if let Some(ref mut state) = accessibility_state {
                if state.options.name.is_none() {
                    state.options.name = Some(alt);
                }
            }
        }

        let changed = &accessibility_state != self;
        *self = accessibility_state;

        changed
    }
}
