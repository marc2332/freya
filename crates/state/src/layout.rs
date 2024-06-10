use std::sync::{Arc, Mutex};

use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    NodeId, SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;
use torin::prelude::*;

use crate::{CustomAttributeValues, NodeReference, Parse};

#[derive(Default, Clone, Debug, Component, PartialEq)]
pub struct LayoutState {
    pub width: Size,
    pub height: Size,
    pub minimum_width: Size,
    pub minimum_height: Size,
    pub maximum_height: Size,
    pub maximum_width: Size,
    pub padding: Gaps,
    pub margin: Gaps,
    pub direction: DirectionMode,
    pub offset_y: Length,
    pub offset_x: Length,
    pub main_alignment: Alignment,
    pub cross_alignment: Alignment,
    pub position: Position,
    pub content: Content,
    pub node_ref: Option<NodeReference>,
    pub node_id: NodeId,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for LayoutState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Width,
            AttributeName::Height,
            AttributeName::MinWidth,
            AttributeName::MinHeight,
            AttributeName::MaxWidth,
            AttributeName::MaxHeight,
            AttributeName::Padding,
            AttributeName::Direction,
            AttributeName::OffsetX,
            AttributeName::OffsetY,
            AttributeName::MainAlign,
            AttributeName::CrossAlign,
            AttributeName::Reference,
            AttributeName::Margin,
            AttributeName::Position,
            AttributeName::PositionTop,
            AttributeName::PositionRight,
            AttributeName::PositionBottom,
            AttributeName::PositionLeft,
            AttributeName::Content,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let torin_layout = context.get::<Arc<Mutex<Torin<NodeId>>>>().unwrap();

        let mut layout = LayoutState {
            node_id: node_view.node_id(),
            ..Default::default()
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute {
                    AttributeName::Width => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(width) = Size::parse(value) {
                                layout.width = width;
                            }
                        }
                    }
                    AttributeName::Height => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(height) = Size::parse(value) {
                                layout.height = height;
                            }
                        }
                    }
                    AttributeName::MinHeight => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(min_height) = Size::parse(value) {
                                layout.minimum_height = min_height;
                            }
                        }
                    }
                    AttributeName::MinWidth => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(min_width) = Size::parse(value) {
                                layout.minimum_width = min_width;
                            }
                        }
                    }
                    AttributeName::MaxHeight => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(max_height) = Size::parse(value) {
                                layout.maximum_height = max_height;
                            }
                        }
                    }
                    AttributeName::MaxWidth => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(max_width) = Size::parse(value) {
                                layout.maximum_width = max_width;
                            }
                        }
                    }
                    AttributeName::Padding => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(padding) = Gaps::parse(value) {
                                layout.padding = padding;
                            }
                        }
                    }
                    AttributeName::Margin => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(margin) = Gaps::parse(value) {
                                layout.margin = margin;
                            }
                        }
                    }
                    AttributeName::Direction => {
                        if let Some(value) = attr.value.as_text() {
                            layout.direction = match value {
                                "horizontal" => DirectionMode::Horizontal,
                                _ => DirectionMode::Vertical,
                            }
                        }
                    }
                    AttributeName::OffsetY => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(scroll) = value.parse::<f32>() {
                                layout.offset_y = Length::new(scroll);
                            }
                        }
                    }
                    AttributeName::OffsetX => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(scroll) = value.parse::<f32>() {
                                layout.offset_x = Length::new(scroll);
                            }
                        }
                    }
                    AttributeName::MainAlign => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(alignment) = Alignment::parse(value) {
                                layout.main_alignment = alignment;
                            }
                        }
                    }
                    AttributeName::CrossAlign => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(alignment) = Alignment::parse(value) {
                                layout.cross_alignment = alignment;
                            }
                        }
                    }
                    AttributeName::Position => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(position) = Position::parse(value) {
                                if layout.position.is_empty() {
                                    layout.position = position;
                                }
                            }
                        }
                    }
                    AttributeName::PositionTop => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(top) = value.parse::<f32>() {
                                layout.position.set_top(top);
                            }
                        }
                    }
                    AttributeName::PositionRight => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(right) = value.parse::<f32>() {
                                layout.position.set_right(right);
                            }
                        }
                    }
                    AttributeName::PositionBottom => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(bottom) = value.parse::<f32>() {
                                layout.position.set_bottom(bottom);
                            }
                        }
                    }
                    AttributeName::PositionLeft => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(left) = value.parse::<f32>() {
                                layout.position.set_left(left);
                            }
                        }
                    }
                    AttributeName::Content => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(content) = Content::parse(value) {
                                layout.content = content;
                            }
                        }
                    }
                    AttributeName::Reference => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Reference(
                            reference,
                        )) = attr.value
                        {
                            layout.node_ref = Some(reference.clone());
                        }
                    }
                    _ => {}
                }
            }
        }

        let changed = layout != *self;

        if changed {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        *self = layout;
        changed
    }
}
