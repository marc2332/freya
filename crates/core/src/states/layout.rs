use std::sync::{
    Arc,
    Mutex,
};

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
use torin::{
    prelude::*,
    wrap_content::WrapContent,
};

use crate::{
    custom_attributes::{
        CustomAttributeValues,
        NodeReference,
    },
    dom::CompositorDirtyNodes,
    parsing::{
        Parse,
        ParseAttribute,
        ParseError,
    },
};

#[derive(Default, Clone, Debug, Component, PartialEq)]
pub struct LayoutState {
    pub width: Size,
    pub height: Size,
    pub minimum_width: Size,
    pub minimum_height: Size,
    pub maximum_height: Size,
    pub maximum_width: Size,
    pub visible_width: VisibleSize,
    pub visible_height: VisibleSize,
    pub padding: Gaps,
    pub margin: Gaps,
    pub direction: Direction,
    pub offset_y: Length,
    pub offset_x: Length,
    pub main_alignment: Alignment,
    pub cross_alignment: Alignment,
    pub position: Position,
    pub content: Content,
    pub wrap_content: WrapContent,
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
                self.width = Size::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::Height => {
                self.height = Size::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::MinHeight => {
                self.minimum_height = Size::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::MinWidth => {
                self.minimum_width = Size::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::MaxHeight => {
                self.maximum_height = Size::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::MaxWidth => {
                self.maximum_width = Size::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::VisibleWidth => {
                self.visible_width = VisibleSize::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::VisibleHeight => {
                self.visible_height = VisibleSize::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::Padding => {
                self.padding = Gaps::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::Margin => {
                self.margin = Gaps::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::Direction => {
                self.direction = match attr.value.as_text().ok_or(ParseError)? {
                    "horizontal" => Direction::Horizontal,
                    _ => Direction::Vertical,
                };
            }
            AttributeName::OffsetY => {
                self.offset_y = Length::new(
                    attr.value
                        .as_text()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );
            }
            AttributeName::OffsetX => {
                self.offset_x = Length::new(
                    attr.value
                        .as_text()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );
            }
            AttributeName::MainAlign => {
                self.main_alignment = Alignment::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::CrossAlign => {
                self.cross_alignment = Alignment::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::Position => {
                self.position
                    .swap_for(Position::parse(attr.value.as_text().ok_or(ParseError)?)?);
            }
            AttributeName::PositionTop => {
                self.position.set_top(
                    attr.value
                        .as_text()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );
            }
            AttributeName::PositionRight => {
                self.position.set_right(
                    attr.value
                        .as_text()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );
            }
            AttributeName::PositionBottom => {
                self.position.set_bottom(
                    attr.value
                        .as_text()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );
            }
            AttributeName::PositionLeft => {
                self.position.set_left(
                    attr.value
                        .as_text()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );
            }
            AttributeName::Content => {
                self.content = Content::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::WrapContent => {
                self.wrap_content = WrapContent::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::Reference => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::Reference(reference)) =
                    attr.value
                {
                    self.node_ref = Some(reference.clone());
                }
            }
            AttributeName::Spacing => {
                self.spacing = Length::new(
                    attr.value
                        .as_text()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                );
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
            AttributeName::VisibleWidth,
            AttributeName::VisibleHeight,
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
            AttributeName::WrapContent,
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
