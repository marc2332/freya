use dioxus_native_core::{
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::Color;
use torin::radius::Radius;

use crate::{Parse, Border, BorderAlignment, CustomAttributeValues, Shadow};

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub struct Style {
    pub background: Color,
    pub relative_layer: i16,
    pub border: Border,
    pub shadows: Vec<Shadow>,
    pub radius: Radius,
    pub image_data: Option<Vec<u8>>,
    pub svg_data: Option<Vec<u8>>,
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
                            if let Ok(background) = Color::parse(value, None) {
                                style.background = background;
                            } else {
                                println!("Failed to parse color: {}", value);
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
                            if let Ok(mut border) = Border::parse(value, Some(*scale_factor)) {
                                border.alignment = style.border.alignment;
                                style.border = border;
                            }
                        }
                    }
                    "border_align" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(alignment) = BorderAlignment::parse(value, None) {
                                style.border.alignment = alignment;
                            }
                        }
                    }
                    "shadow" => {
                        if let Some(attr) = attr.value.as_text() {
                            let mut chunks = Vec::new();
                            let mut current = String::new();
                            let mut in_parenthesis = false;

                            for character in attr.chars() {
                                if character == '(' {
                                    in_parenthesis = true;
                                } else if character == ')' {
                                    in_parenthesis = false;
                                }

                                if character == ',' && !in_parenthesis {
                                    chunks.push(std::mem::take(&mut current));
                                } else {
                                    current.push(character);
                                }
                            }

                            if !current.is_empty() {
                                chunks.push(current);
                            }

                            shadows = chunks
                                .iter()
                                .map(|chunk| parse_shadow(chunk, *scale_factor).unwrap_or_default())
                                .collect();
                        }
                    }
                    "shadow" => {
                        if let Some(value) = attr.value.as_text() {
                            let mut chunks = Vec::new();
                            let mut current = String::new();
                            let mut in_parenthesis = false;

                            for character in value.chars() {
                                if character == '(' {
                                    in_parenthesis = true;
                                } else if character == ')' {
                                    in_parenthesis = false;
                                }

                                if character == ',' && !in_parenthesis {
                                    chunks.push(std::mem::take(&mut current));
                                } else {
                                    current.push(character);
                                }
                            }

                            if current.len() > 0 {
                                chunks.push(current);
                            }

                            style.shadows = chunks
                                .iter()
                                .map(|chunk| {
                                    Shadow::parse(chunk, Some(*scale_factor)).unwrap_or_default()
                                })
                                .collect();
                        }
                    }
                    "radius" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(radius) = Radius::parse(value, Some(*scale_factor)) {
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
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = &style != self;

        *self = style;
        changed
    }
}
