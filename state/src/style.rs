use dioxus_native_core::{
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;
// use skia_safe::Color;
use torin::{radius::Radius, scaled::Scaled};

use crate::{
    Border, BorderAlignment, CustomAttributeValues, ExtSplit, Fill, OverflowMode, Parse, Shadow,
};

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub struct Style {
    pub background: Fill,
    pub relative_layer: i16,
    pub border: Border,
    pub shadows: Vec<Shadow>,
    pub radius: Radius,
    pub image_data: Option<Vec<u8>>,
    pub svg_data: Option<Vec<u8>>,
    pub overflow: OverflowMode,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for Style {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            "background",
            "layer",
            "border",
            "border_align",
            "shadow",
            "radius",
            "image_data",
            "svg_data",
            "svg_content",
            "overflow",
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
                match attr.attribute.name.as_str() {
                    "background" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(background) = Fill::parse(value) {
                                style.background = background;
                            }
                        }
                    }
                    "layer" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(relative_layer) = value.parse::<i16>() {
                                style.relative_layer = relative_layer;
                            }
                        }
                    }
                    "border" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut border) = Border::parse(value) {
                                border.alignment = style.border.alignment;
                                border.scale(*scale_factor);

                                style.border = border;
                            }
                        }
                    }
                    "border_align" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(alignment) = BorderAlignment::parse(value) {
                                style.border.alignment = alignment;
                            }
                        }
                    }
                    "shadow" => {
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
                    "radius" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut radius) = Radius::parse(value) {
                                radius.scale(*scale_factor);
                                style.radius = radius;
                            }
                        }
                    }
                    "image_data" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) =
                            attr.value
                        {
                            style.image_data = Some(bytes.clone());
                        }
                    }
                    "svg_data" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) =
                            attr.value
                        {
                            style.svg_data = Some(bytes.clone());
                        }
                    }
                    "svg_content" => {
                        let text = attr.value.as_text();
                        style.svg_data = text.map(|v| v.as_bytes().to_owned());
                    }
                    "overflow" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(overflow) = OverflowMode::parse(value) {
                                style.overflow = overflow;
                            }
                        }
                    }
                    _ => {
                        panic!("Unsupported attribute <{}>, this should not be happening, please report it.", attr.attribute.name);
                    }
                }
            }
        }

        let changed = &style != self;

        *self = style;
        changed
    }
}
