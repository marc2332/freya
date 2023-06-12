use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::NodeView;
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::SendAnyMap;
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::Color;

use crate::{parse_color, CustomAttributeValues};

#[derive(Default, Clone, Debug, Component)]
pub struct Style {
    pub background: Color,
    pub relative_layer: i16,
    pub border: BorderSettings,
    pub shadow: ShadowSettings,
    pub radius: f32,
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
        let mut shadow = ShadowSettings::default();
        let mut border = BorderSettings::default();
        let mut radius = 0.0;
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
                            if let Some(new_shadow) = parse_shadow(attr) {
                                shadow = new_shadow;
                            }
                        }
                    }
                    "radius" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Ok(new_radius) = attr.parse::<f32>() {
                                radius = new_radius * scale_factor;
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
            || (shadow != self.shadow)
            || (border != self.border)
            || (radius != self.radius)
            || (image_data != self.image_data)
            || (svg_data != self.svg_data);

        *self = Self {
            background,
            relative_layer,
            shadow,
            border,
            radius,
            image_data,
            svg_data,
        };
        changed
    }
}

pub fn parse_shadow(value: &str) -> Option<ShadowSettings> {
    let value = value.to_string();
    let mut shadow_values = value.split_ascii_whitespace();
    Some(ShadowSettings {
        x: shadow_values.next()?.parse().ok()?,
        y: shadow_values.next()?.parse().ok()?,
        intensity: shadow_values.next()?.parse().ok()?,
        size: shadow_values.next()?.parse().ok()?,
        color: parse_color(shadow_values.next()?)?,
    })
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
    pub intensity: u8,
    pub size: f32,
    pub color: Color,
}
