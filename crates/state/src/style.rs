use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;
use torin::scaled::Scaled;

use crate::{
    parsing::ExtSplit, AttributesBytes, Border, BorderAlignment, CornerRadius,
    CustomAttributeValues, Fill, OverflowMode, Parse, Shadow,
};

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub struct Style {
    pub background: Fill,
    pub border: Border,
    pub shadows: Vec<Shadow>,
    pub corner_radius: CornerRadius,
    pub image_data: Option<AttributesBytes>,
    pub svg_data: Option<AttributesBytes>,
    pub overflow: OverflowMode,
    pub opacity: Option<f32>,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for Style {
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
        context: &SendAnyMap,
    ) -> bool {
        let mut style = Style::default();
        let scale_factor = context.get::<f32>().unwrap();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute {
                    AttributeName::Background => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(background) = Fill::parse(value) {
                                style.background = background;
                            }
                        }
                    }
                    AttributeName::Border => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut border) = Border::parse(value) {
                                border.alignment = style.border.alignment;
                                border.scale(*scale_factor);

                                style.border = border;
                            }
                        }
                    }
                    AttributeName::BorderAlign => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(alignment) = BorderAlignment::parse(value) {
                                style.border.alignment = alignment;
                            }
                        }
                    }
                    AttributeName::Shadow => {
                        if let Some(value) = attr.value.as_text() {
                            style.shadows = value
                                .split_excluding_group(',', '(', ')')
                                .map(|chunk| {
                                    let mut shadow = Shadow::parse(chunk).unwrap_or_default();
                                    shadow.scale(*scale_factor);
                                    shadow
                                })
                                .collect();
                        }
                    }
                    AttributeName::CornerRadius => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut radius) = CornerRadius::parse(value) {
                                radius.scale(*scale_factor);
                                radius.smoothing = style.corner_radius.smoothing;
                                style.corner_radius = radius;
                            }
                        }
                    }
                    AttributeName::CornerSmoothing => {
                        if let Some(value) = attr.value.as_text() {
                            if value.ends_with('%') {
                                if let Ok(smoothing) = value.replacen('%', "", 1).parse::<f32>() {
                                    style.corner_radius.smoothing =
                                        (smoothing / 100.0).clamp(0.0, 1.0);
                                }
                            }
                        }
                    }
                    AttributeName::ImageData => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) =
                            attr.value
                        {
                            style.image_data = Some(bytes.clone());
                        }
                    }
                    AttributeName::SvgData => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) =
                            attr.value
                        {
                            style.svg_data = Some(bytes.clone());
                        }
                    }
                    AttributeName::SvgContent => {
                        let text = attr.value.as_text();
                        style.svg_data =
                            text.map(|v| AttributesBytes::Dynamic(v.as_bytes().to_vec().into()));
                    }
                    AttributeName::Overflow => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(overflow) = OverflowMode::parse(value) {
                                style.overflow = overflow;
                            }
                        }
                    }
                    AttributeName::Opacity => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(opacity) = value.parse::<f32>() {
                                style.opacity = Some(opacity);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let changed = &style != self;

        *self = style;
        changed
    }
}
