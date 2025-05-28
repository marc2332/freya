use std::sync::{
    Arc,
    Mutex,
};

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

use crate::{
    custom_attributes::{
        AttributesBytes,
        CustomAttributeValues,
    },
    dom::CompositorDirtyNodes,
    parsing::{
        Parse,
        ParseAttribute,
        ParseError,
    },
    values::SvgPaint,
};

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub struct SvgState {
    pub svg_fill: Option<SvgPaint>,
    pub svg_stroke: Option<SvgPaint>,
    pub svg_data: Option<AttributesBytes>,
}

impl ParseAttribute for SvgState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        match attr.attribute {
            AttributeName::SvgData => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) = attr.value
                {
                    self.svg_data = Some(bytes.clone());
                }
            }
            AttributeName::Fill => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                if value == "none" {
                    return Ok(());
                }
                self.svg_fill = Some(SvgPaint::parse(value)?);
            }
            AttributeName::Stroke => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                if value == "none" {
                    return Ok(());
                }
                self.svg_stroke = Some(SvgPaint::parse(value)?);
            }
            AttributeName::SvgContent => {
                self.svg_data = attr
                    .value
                    .as_text()
                    .map(|v| AttributesBytes::Dynamic(v.as_bytes().to_vec().into()));
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for SvgState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Fill,
            AttributeName::Stroke,
            AttributeName::SvgData,
            AttributeName::SvgContent,
        ]));

    fn allow_node(node_type: &NodeType<CustomAttributeValues>) -> bool {
        node_type.tag() == Some(&TagName::Svg)
    }

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let mut style = SvgState::default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                style.parse_safe(attr)
            }
        }

        let changed = &style != self;
        if changed {
            let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id());
        }

        *self = style;
        changed
    }
}
