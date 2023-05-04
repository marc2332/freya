use std::sync::{Arc, Mutex};

use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::NodeView;
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::{NodeId, SendAnyMap};
use dioxus_native_core_macro::partial_derive_state;
use freya_common::NodeReferenceLayout;
use tokio::sync::mpsc::UnboundedSender;
use torin::*;

use crate::CustomAttributeValues;

#[derive(Default, Clone, Debug, Component)]
pub struct SizeState {
    pub width: Size,
    pub height: Size,
    pub minimum_width: Size,
    pub minimum_height: Size,
    pub maximum_height: Size,
    pub maximum_width: Size,
    pub padding: Paddings,
    pub direction: DirectionMode,
    pub node_id: NodeId,
    pub scroll_y: f32,
    pub scroll_x: f32,
    pub display: DisplayMode,
    pub node_ref: Option<UnboundedSender<NodeReferenceLayout>>,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for SizeState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[
            "width",
            "height",
            "min_height",
            "min_width",
            "max_height",
            "max_width",
            "padding",
            "direction",
            "scroll_y",
            "scroll_x",
            "display",
            "reference",
        ]))
        .with_tag()
        .with_text();

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let torin_layout = context.get::<Arc<Mutex<Torin<NodeId>>>>().unwrap();
        let scale_factor = context.get::<f32>().unwrap();

        let mut width = Size::default();
        let mut height = Size::default();
        let mut minimum_height = Size::default();
        let mut minimum_width = Size::default();
        let mut maximum_height = Size::default();
        let mut maximum_width = Size::default();
        let mut padding = Paddings::default();
        let mut scroll_y = 0.0;
        let mut scroll_x = 0.0;
        let mut display = DisplayMode::Normal;
        let mut node_ref = None;

        let mut direction = if let Some("label") = node_view.tag() {
            DirectionMode::Horizontal
        } else if let Some("paragraph") = node_view.tag() {
            DirectionMode::Horizontal
        } else if let Some("text") = node_view.tag() {
            DirectionMode::Horizontal
        } else if node_view.text().is_some() {
            DirectionMode::Horizontal
        } else {
            DirectionMode::Vertical
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "width" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_width) = parse_size(attr, *scale_factor) {
                                width = new_width;
                            }
                        }
                    }
                    "height" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_height) = parse_size(attr, *scale_factor) {
                                height = new_height;
                            }
                        }
                    }
                    "min_height" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_min_height) = parse_size(attr, *scale_factor) {
                                minimum_height = new_min_height;
                            }
                        }
                    }
                    "min_width" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_min_width) = parse_size(attr, *scale_factor) {
                                minimum_width = new_min_width;
                            }
                        }
                    }
                    "max_height" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_max_height) = parse_size(attr, *scale_factor) {
                                maximum_height = new_max_height;
                            }
                        }
                    }
                    "max_width" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_max_width) = parse_size(attr, *scale_factor) {
                                maximum_width = new_max_width;
                            }
                        }
                    }
                    "padding" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(paddings) = parse_padding(attr, *scale_factor) {
                                padding = paddings;
                            }
                        }
                    }
                    "direction" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            direction = if attr == "horizontal" {
                                DirectionMode::Horizontal
                            } else if attr == "both" {
                                DirectionMode::Both
                            } else {
                                DirectionMode::Vertical
                            };
                        }
                    }
                    "scroll_y" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            let scroll: f32 = attr.parse().unwrap();
                            scroll_y = scroll * scale_factor;
                        }
                    }
                    "scroll_x" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            let scroll: f32 = attr.parse().unwrap();
                            scroll_x = scroll * scale_factor;
                        }
                    }
                    "display" => {
                        if let Some(new_display) = attr.value.as_text() {
                            display = parse_display(new_display)
                        }
                    }
                    "reference" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Reference(
                            reference,
                        )) = attr.value
                        {
                            node_ref = Some(reference.0.clone());
                        }
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = (width != self.width)
            || (height != self.height)
            || (minimum_width != self.minimum_width)
            || (minimum_height != self.minimum_height)
            || (maximum_width != self.maximum_width)
            || (maximum_height != self.maximum_height)
            || (padding != self.padding)
            || (node_view.node_id() != self.node_id)
            || (direction != self.direction)
            || (scroll_x != self.scroll_x)
            || (scroll_y != self.scroll_y)
            || (display != self.display);

        if changed {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        *self = Self {
            width,
            height,
            minimum_height,
            minimum_width,
            maximum_height,
            maximum_width,
            padding,
            direction,
            node_id: node_view.node_id(),
            scroll_x,
            scroll_y,
            display,
            node_ref,
        };
        changed
    }
}

pub fn parse_display(value: &str) -> DisplayMode {
    match value {
        "center" => DisplayMode::Center,
        _ => DisplayMode::Normal,
    }
}

pub fn parse_padding(padding: &str, scale_factor: f32) -> Option<Paddings> {
    let mut padding_config = Paddings::default();
    let mut paddings = padding.split_ascii_whitespace();

    match paddings.clone().count() {
        // Same in each directions
        1 => {
            padding_config.fill_all(paddings.next()?.parse::<f32>().ok()? * scale_factor);
        }
        // By vertical and horizontal
        2 => {
            // Vertical
            padding_config.fill_vertical(paddings.next()?.parse::<f32>().ok()? * scale_factor);

            // Horizontal
            padding_config.fill_horizontal(paddings.next()?.parse::<f32>().ok()? * scale_factor)
        }
        // Each directions
        4 => {
            padding_config = Paddings::new(
                paddings.next()?.parse::<f32>().ok()? * scale_factor,
                paddings.next()?.parse::<f32>().ok()? * scale_factor,
                paddings.next()?.parse::<f32>().ok()? * scale_factor,
                paddings.next()?.parse::<f32>().ok()? * scale_factor,
            );
        }
        _ => {}
    }

    Some(padding_config)
}

pub fn parse_size(size: &str, scale_factor: f32) -> Option<Size> {
    if size == "auto" {
        Some(Size::Inner)
    } else if size.contains("calc") {
        Some(Size::DynamicCalculations(parse_calc(size, scale_factor)?))
    } else if size.contains('%') {
        Some(Size::Percentage(Length::new(
            size.replace('%', "").parse().ok()?,
        )))
    } else {
        Some(Size::Pixels(Length::new(
            (size.parse::<f32>().ok()?) * scale_factor,
        )))
    }
}
pub fn parse_calc(mut size: &str, scale_factor: f32) -> Option<Vec<DynamicCalculation>> {
    let mut calcs = Vec::new();

    size = size.strip_prefix("calc(")?;
    size = size.strip_suffix(')')?;

    let vals = size.split_whitespace();

    for val in vals {
        if val.contains('%') {
            calcs.push(DynamicCalculation::Percentage(
                val.replace('%', "").parse().ok()?,
            ));
        } else if val == "+" {
            calcs.push(DynamicCalculation::Add);
        } else if val == "-" {
            calcs.push(DynamicCalculation::Sub);
        } else if val == "/" {
            calcs.push(DynamicCalculation::Div);
        } else if val == "*" {
            calcs.push(DynamicCalculation::Mul);
        } else {
            calcs.push(DynamicCalculation::Pixels(
                val.parse::<f32>().ok()? * scale_factor,
            ));
        }
    }

    Some(calcs)
}
