use std::fmt::Display;

use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::NodeDepState;
use dioxus_native_core_macro::sorted_str_slice;
use skia_safe::Color;

use crate::parse_color;

#[derive(Default, Clone, Debug)]
pub struct Style {
    pub background: Color,
    pub relative_layer: i16,
    pub shadow: ShadowSettings,
    pub radius: f32,
    pub image_data: Option<Vec<u8>>,
    pub svg_data: Option<Vec<u8>>,
    pub display: DisplayMode,
}

impl NodeDepState<()> for Style {
    type Ctx = ();

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "background",
            "layer",
            "shadow",
            "radius",
            "image_data",
            "svg_data",
            "svg_content",
            "display",
        ])));

    fn reduce<'a>(&mut self, node: NodeView, _sibling: (), _ctx: &Self::Ctx) -> bool {
        let mut background = Color::TRANSPARENT;
        let mut relative_layer = 0;
        let mut shadow = ShadowSettings::default();
        let mut radius = 0.0;
        let mut image_data = None;
        let mut svg_data = None;
        let mut display = DisplayMode::Normal;

        for attr in node.attributes() {
            match attr.name {
                "display" => display = parse_display(&attr.value.to_string()),
                "background" => {
                    let new_back = parse_color(&attr.value.to_string());
                    if let Some(new_back) = new_back {
                        background = new_back;
                    }
                }
                "layer" => {
                    let new_relative_layer: Option<i16> = attr.value.to_string().parse().ok();
                    if let Some(new_relative_layer) = new_relative_layer {
                        relative_layer = new_relative_layer;
                    }
                }
                "shadow" => {
                    let new_shadow = parse_shadow(&attr.value.to_string());

                    if let Some(new_shadow) = new_shadow {
                        shadow = new_shadow;
                    }
                }
                "radius" => {
                    let new_radius: Option<f32> = attr.value.to_string().parse().ok();

                    if let Some(new_radius) = new_radius {
                        radius = new_radius;
                    }
                }
                "image_data" => {
                    let bytes = attr.value.as_bytes();
                    image_data = bytes.map(|v| v.to_vec());
                }
                "svg_data" => {
                    let bytes = attr.value.as_bytes();
                    svg_data = bytes.map(|v| v.to_vec());
                }
                "svg_content" => {
                    let text = attr.value.as_text();
                    svg_data = text.map(|v| v.as_bytes().to_vec());
                }
                _ => {
                    println!("Unsupported attribute <{}>", attr.name);
                }
            }
        }

        let changed = (background != self.background)
            || (relative_layer != self.relative_layer)
            || (shadow != self.shadow)
            || (radius != self.radius)
            || (image_data != self.image_data);

        *self = Self {
            background,
            relative_layer,
            shadow,
            radius,
            image_data,
            svg_data,
            display,
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

pub fn parse_display(value: &str) -> DisplayMode {
    match value {
        "center" => DisplayMode::Center,
        _ => DisplayMode::Normal,
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum DisplayMode {
    #[default]
    Normal,
    Center,
}

impl Display for DisplayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayMode::Normal => f.write_str("normal"),
            DisplayMode::Center => f.write_str("center"),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ShadowSettings {
    pub x: f32,
    pub y: f32,
    pub intensity: u8,
    pub size: f32,
    pub color: Color,
}
