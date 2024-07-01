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
    parsing::ExtSplit,
    AttributesBytes,
    Border,
    BorderAlignment,
    CornerRadius,
    CustomAttributeValues,
    Fill,
    OverflowMode,
    Parse,
    ParseAttribute,
    ParseError,
    Shadow,
};

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub struct StyleState {
    pub background: Fill,
    pub border: Border,
    pub shadows: Vec<Shadow>,
    pub corner_radius: CornerRadius,
    pub image_data: Option<AttributesBytes>,
    pub svg_data: Option<AttributesBytes>,
    pub overflow: OverflowMode,
    pub opacity: Option<f32>,
}

impl ParseAttribute for StyleState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), crate::ParseError> {
        match attr.attribute {
            AttributeName::Background => {
                if let Some(value) = attr.value.as_text() {
                    self.background = Fill::parse(value)?;
                }
            }
            AttributeName::Border => {
                if let Some(value) = attr.value.as_text() {
                    let mut border = Border::parse(value)?;
                    border.alignment = self.border.alignment;
                    self.border = border;
                }
            }
            AttributeName::BorderAlign => {
                if let Some(value) = attr.value.as_text() {
                    self.border.alignment = BorderAlignment::parse(value)?;
                }
            }
            AttributeName::Shadow => {
                if let Some(value) = attr.value.as_text() {
                    self.shadows = value
                        .split_excluding_group(',', '(', ')')
                        .map(|chunk| Shadow::parse(chunk).unwrap_or_default())
                        .collect();
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
                    if value.ends_with('%') {
                        let smoothing = value
                            .replacen('%', "", 1)
                            .parse::<f32>()
                            .map_err(|_| ParseError)?;
                        self.corner_radius.smoothing = (smoothing / 100.0).clamp(0.0, 1.0);
                    }
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
            AttributeName::Opacity => {
                if let Some(value) = attr.value.as_text() {
                    self.opacity = Some(value.parse::<f32>().map_err(|_| ParseError)?);
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for StyleState {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Background,
            AttributeName::Layer,
            AttributeName::Border,
            AttributeName::BorderAlign,
            AttributeName::Shadow,
            AttributeName::CornerRadius,
            AttributeName::CornerSmoothing,
            AttributeName::ImageData,
            AttributeName::SvgData,
            AttributeName::SvgContent,
            AttributeName::Overflow,
            AttributeName::Opacity,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut style = StyleState::default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                style.parse_safe(attr)
            }
        }

        let changed = &style != self;

        *self = style;
        changed
    }
}
