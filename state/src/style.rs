use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::NodeView;
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::SendAnyMap;
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::Color;
use torin::radius::Radius;

use crate::{Border, BorderAlignment, CustomAttributeValues, Shadow};

#[derive(Default, Clone, Debug, Component)]
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
        let scale_factor = context.get::<f32>().unwrap();

        let mut background = Color::TRANSPARENT;
        let mut relative_layer = 0;
        let mut shadows: Vec<Shadow> = vec![];
        let mut border = Border::default();
        let mut radius = Radius::default();
        let mut image_data = None;
        let mut svg_data = None;

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "background" => {
                        if let Some(attr) = attr.value.as_text() {
                            let new_back = parse_color(attr);
                            if let Some(new_back) = new_back {
                                background = new_back;
                            }
                        }
                    }
                    "layer" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Ok(new_relative_layer) = attr.parse::<i16>() {
                                relative_layer = new_relative_layer;
                            }
                        }
                    }
                    "border" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Ok(mut new_border) = attr.parse::<Border>() {
                                new_border.width *= scale_factor;

                                border = new_border;
                            }
                        }
                    }
                    "border_align" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Ok(new_border_alignment) = attr.parse::<BorderAlignment>() {
                                border.alignment = new_border_alignment;
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

                            if current.len() > 0 {
                                chunks.push(current);
                            }

                            shadows = chunks
                                .iter()
                                .map(|chunk| {
                                    let mut shadow = chunk.parse::<Shadow>().unwrap_or_default();

                                    shadow.x *= scale_factor;
                                    shadow.y *= scale_factor;
                                    shadow.spread *= scale_factor;
                                    shadow.blur *= scale_factor;

                                    shadow
                                })
                                .collect();
                        }
                    }
                    "radius" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Some(new_radius) = parse_radius(attr, *scale_factor) {
                                radius = new_radius;
                            }
                        }
                    }
                    "image_data" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) =
                            attr.value
                        {
                            image_data = Some(bytes.clone());
                        }
                    }
                    "svg_data" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) =
                            attr.value
                        {
                            svg_data = Some(bytes.clone());
                        }
                    }
                    "svg_content" => {
                        let text = attr.value.as_text();
                        svg_data = text.map(|v| v.as_bytes().to_owned());
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = (background != self.background)
            || (relative_layer != self.relative_layer)
            || (shadows != self.shadows)
            || (border != self.border)
            || (radius != self.radius)
            || (image_data != self.image_data)
            || (svg_data != self.svg_data);

        *self = Self {
            background,
            relative_layer,
            shadows,
            border,
            radius,
            image_data,
            svg_data,
        };
        changed
    }
}