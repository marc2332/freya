use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::NodeView;
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::SendAnyMap;
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::Color;
use torin::radius::Radius;

use crate::{parse_color, CustomAttributeValues};

#[derive(Default, Clone, Debug, Component)]
pub struct Style {
    pub background: Color,
    pub relative_layer: i16,
    pub border: BorderSettings,
    pub shadows: Vec<ShadowSettings>,
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
        let mut shadows: Vec<ShadowSettings> = vec![];
        let mut border = BorderSettings::default();
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
                            if let Some(new_border) =
                                parse_border(attr, border.alignment, *scale_factor)
                            {
                                border = new_border;
                            }
                        }
                    }
                    "border_align" => {
                        if let Some(attr) = attr.value.as_text() {
                            border.alignment = parse_border_align(attr);
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
                                .map(|chunk| parse_shadow(chunk, *scale_factor).unwrap_or_default())
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

pub fn parse_radius(value: &str, scale_factor: f32) -> Option<Radius> {
    let mut radius_config = Radius::default();
    let mut radius = value.split_ascii_whitespace();

    match radius.clone().count() {
        // Same in all corners
        1 => {
            radius_config.fill_all(radius.next()?.parse::<f32>().ok()? * scale_factor);
        }
        // By Top and Bottom
        2 => {
            // Top
            radius_config.fill_top(radius.next()?.parse::<f32>().ok()? * scale_factor);

            // Bottom
            radius_config.fill_bottom(radius.next()?.parse::<f32>().ok()? * scale_factor)
        }
        // Each corner
        4 => {
            radius_config = Radius::new(
                radius.next()?.parse::<f32>().ok()? * scale_factor,
                radius.next()?.parse::<f32>().ok()? * scale_factor,
                radius.next()?.parse::<f32>().ok()? * scale_factor,
                radius.next()?.parse::<f32>().ok()? * scale_factor,
            );
        }
        _ => {}
    }

    Some(radius_config)
}

pub fn parse_shadow(value: &str, scale_factor: f32) -> Option<ShadowSettings> {
    let value = value.to_string();
    let mut shadow_values = value.split_ascii_whitespace();

    let mut shadow = ShadowSettings::default();

    let first = shadow_values.next()?;
    if first == "inset" {
        shadow.inset = true;
        shadow.x = shadow_values.next()?.parse().ok()?;
    } else {
        shadow.x = first.parse().ok()?;
    }

    shadow.x *= scale_factor;
    shadow.y = shadow_values.next()?.parse::<f32>().ok()? * scale_factor;
    shadow.blur = shadow_values.next()?.parse::<f32>().ok()? * scale_factor;

    let spread_or_color = shadow_values.next()?;
    let mut color_string = String::new();
    if spread_or_color.parse::<f32>().is_ok() {
        shadow.spread = spread_or_color.parse::<f32>().ok()? * scale_factor;
    } else {
        color_string.push_str(spread_or_color);
    }
    color_string.push_str(shadow_values.collect::<Vec<&str>>().join(" ").as_str());

    shadow.color = parse_color(color_string.as_str())?;

    Some(shadow)
}

pub fn parse_border_align(value: &str) -> BorderAlignment {
    let mut border_align_value = value.split_ascii_whitespace();

    match border_align_value.next() {
        Some("inner") => BorderAlignment::Inner,
        Some("outer") => BorderAlignment::Outer,
        Some("center") => BorderAlignment::Center,
        _ => BorderAlignment::Inner,
    }
}

pub fn parse_border(
    border_value: &str,
    alignment: BorderAlignment,
    scale_factor: f32,
) -> Option<BorderSettings> {
    let mut border_values = border_value.split_ascii_whitespace();

    Some(BorderSettings {
        width: border_values.next()?.parse::<f32>().ok()? * scale_factor,
        style: match border_values.next()? {
            "solid" => BorderStyle::Solid,
            _ => BorderStyle::None,
        },
        color: parse_color(&border_values.collect::<Vec<&str>>().join(" "))?,
        alignment,
    })
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderAlignment {
    #[default]
    Inner,
    Outer,
    Center,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct BorderSettings {
    pub color: Color,
    pub style: BorderStyle,
    pub width: f32,
    pub alignment: BorderAlignment,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ShadowSettings {
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
    pub inset: bool,
}
