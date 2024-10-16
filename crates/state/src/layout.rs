use std::sync::{
    Arc,
    Mutex,
};

use freya_common::CompositorDirtyNodes;
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        OwnedAttributeView,
        State,
    },
    NodeId,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;
use torin::prelude::*;

use crate::{
    CustomAttributeValues,
    NodeReference,
    Parse,
    ParseAttribute,
    ParseError,
};

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
    pub spacing: Length,
}

impl ParseAttribute for LayoutState {
    fn parse_attribute(
        &mut self,
        attr: OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        match attr.attribute {
            AttributeName::Width => {
                if let Some(value) = attr.value.as_text() {
                    self.width = Size::parse(value)?;
                }
            }
            AttributeName::Height => {
                if let Some(value) = attr.value.as_text() {
                    self.height = Size::parse(value)?;
                }
            }
            AttributeName::MinHeight => {
                if let Some(value) = attr.value.as_text() {
                    self.minimum_height = Size::parse(value)?;
                }
            }
            AttributeName::MinWidth => {
                if let Some(value) = attr.value.as_text() {
                    self.minimum_width = Size::parse(value)?;
                }
            }
            AttributeName::MaxHeight => {
                if let Some(value) = attr.value.as_text() {
                    self.maximum_height = Size::parse(value)?;
                }
            }
            AttributeName::MaxWidth => {
                if let Some(value) = attr.value.as_text() {
                    self.maximum_width = Size::parse(value)?;
                }
            }
            AttributeName::Padding => {
                if let Some(value) = attr.value.as_text() {
                    self.padding = Gaps::parse(value)?;
                }
            }
            AttributeName::Margin => {
                if let Some(value) = attr.value.as_text() {
                    self.margin = Gaps::parse(value)?;
                }
            }
            AttributeName::Direction => {
                if let Some(value) = attr.value.as_text() {
                    self.direction = match value {
                        "horizontal" => DirectionMode::Horizontal,
                        "vertical" => DirectionMode::Vertical,
                        value => {
                            return Err(ParseError::invalid_ident(
                                value,
                                &["horizontal", "vertical"],
                            ))
                        }
                    }
                }
            }
            AttributeName::OffsetY => {
                if let Some(value) = attr.value.as_text() {
                    self.offset_y = Length::new(
                        value
                            .parse::<f32>()
                            .map_err(|err| ParseError(err.to_string()))?,
                    );
                }
            }
            AttributeName::OffsetX => {
                if let Some(value) = attr.value.as_text() {
                    self.offset_x = Length::new(
                        value
                            .parse::<f32>()
                            .map_err(|err| ParseError(err.to_string()))?,
                    );
                }
            }
            AttributeName::MainAlign => {
                if let Some(value) = attr.value.as_text() {
                    self.main_alignment = Alignment::parse(value)?;
                }
            }
            AttributeName::CrossAlign => {
                if let Some(value) = attr.value.as_text() {
                    self.cross_alignment = Alignment::parse(value)?;
                }
            }
            AttributeName::Position => {
                if let Some(value) = attr.value.as_text() {
                    if self.position.is_empty() {
                        self.position = Position::parse(value)?;
                    }
                }
            }
            AttributeName::PositionTop => {
                if let Some(value) = attr.value.as_text() {
                    self.position.set_top(
                        value
                            .parse::<f32>()
                            .map_err(|err| ParseError(err.to_string()))?,
                    );
                }
            }
            AttributeName::PositionRight => {
                if let Some(value) = attr.value.as_text() {
                    self.position.set_right(
                        value
                            .parse::<f32>()
                            .map_err(|err| ParseError(err.to_string()))?,
                    );
                }
            }
            AttributeName::PositionBottom => {
                if let Some(value) = attr.value.as_text() {
                    self.position.set_bottom(
                        value
                            .parse::<f32>()
                            .map_err(|err| ParseError(err.to_string()))?,
                    );
                }
            }
            AttributeName::PositionLeft => {
                if let Some(value) = attr.value.as_text() {
                    self.position.set_left(
                        value
                            .parse::<f32>()
                            .map_err(|err| ParseError(err.to_string()))?,
                    );
                }
            }
            AttributeName::Content => {
                if let Some(value) = attr.value.as_text() {
                    self.content = Content::parse(value)?;
                }
            }
            AttributeName::Reference => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::Reference(reference)) =
                    attr.value
                {
                    self.node_ref = Some(reference.clone());
                }
            }
            AttributeName::Spacing => {
                if let Some(value) = attr.value.as_text() {
                    self.spacing = Length::new(
                        value
                            .parse::<f32>()
                            .map_err(|err| ParseError(err.to_string()))?,
                    );
                }
            }
            _ => {}
        }
        Ok(())
    }
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
            AttributeName::Spacing,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let root_id = context.get::<NodeId>().unwrap();
        let torin_layout = context.get::<Arc<Mutex<Torin<NodeId>>>>().unwrap();
        let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();

        let mut layout = LayoutState {
            node_id: node_view.node_id(),
            ..Default::default()
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                layout.parse_safe(attr);
            }
        }

        let changed = layout != *self;

        let is_orphan = node_view.height() == 0 && node_view.node_id() != *root_id;

        if changed && !is_orphan {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id());
        }

        *self = layout;
        changed
    }
}
