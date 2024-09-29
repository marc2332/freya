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
        State,
    },
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{
    AttributesBytes,
    Border,
    CornerRadius,
    CustomAttributeValues,
    Fill,
    Lexer,
    OverflowMode,
    Parse,
    ParseAttribute,
    Parser,
    Shadow,
    Token,
};

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub struct StyleState {
    pub background: Fill,
    pub borders: Vec<Border>,
    pub shadows: Vec<Shadow>,
    pub corner_radius: CornerRadius,
    pub image_data: Option<AttributesBytes>,
    pub svg_data: Option<AttributesBytes>,
    pub overflow: OverflowMode,
}

impl ParseAttribute for StyleState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), crate::ParseError> {
        match attr.attribute {
            AttributeName::Background => {
                if let Some(value) = attr.value.as_text() {
                    if value == "none" {
                        return Ok(());
                    }

                    self.background = Fill::parse(value)?;
                }
            }
            AttributeName::Border => {
                if let Some(value) = attr.value.as_text() {
                    self.borders = Border::parse_with_separator(value, &Token::Comma)?;
                }
            }
            AttributeName::Shadow => {
                if let Some(value) = attr.value.as_text() {
                    self.shadows = Shadow::parse_with_separator(value, &Token::Comma)?;
                }
            }
            AttributeName::CornerRadius => {
                if let Some(value) = attr.value.as_text() {
                    let mut radius = CornerRadius::parse(value)?;

                    radius.smoothing = self.corner_radius.smoothing;

                    self.corner_radius = radius;
                }
            }
            AttributeName::CornerSmoothing => {
                if let Some(value) = attr.value.as_text() {
                    let mut parser = Parser::new(Lexer::parse(value));

                    let smoothing = parser.consume_map(Token::try_as_f32)?;

                    parser.consume(&Token::Percent)?;

                    self.corner_radius.smoothing = (smoothing / 100.0).clamp(0.0, 1.0);
                }
            }
            AttributeName::ImageData => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) = attr.value
                {
                    self.image_data = Some(bytes.clone());
                }
            }
            AttributeName::SvgData => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) = attr.value
                {
                    self.svg_data = Some(bytes.clone());
                }
            }
            AttributeName::SvgContent => {
                let text = attr.value.as_text();
                self.svg_data =
                    text.map(|v| AttributesBytes::Dynamic(v.as_bytes().to_vec().into()));
            }
            AttributeName::Overflow => {
                if let Some(value) = attr.value.as_text() {
                    self.overflow = OverflowMode::parse(value)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for StyleState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Background,
            AttributeName::Layer,
            AttributeName::Border,
            AttributeName::Shadow,
            AttributeName::CornerRadius,
            AttributeName::CornerSmoothing,
            AttributeName::ImageData,
            AttributeName::SvgData,
            AttributeName::SvgContent,
            AttributeName::Overflow,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();
        let mut style = StyleState::default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                style.parse_safe(attr)
            }
        }

        let changed = &style != self;

        if changed {
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id())
        }

        *self = style;
        changed
    }
}
