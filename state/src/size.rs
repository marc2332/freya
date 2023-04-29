use std::sync::{Arc, Mutex};

use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node_ref::NodeView;
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::{NodeId, SendAnyMap};
use dioxus_native_core_macro::partial_derive_state;
use torin::{Direction, DynamicCalculation, EmbeddedData, Length, Node, Paddings, Torin};

use crate::CustomAttributeValues;

#[derive(Default, Clone, Debug, Component)]
pub struct Size {
    pub width: torin::Size,
    pub height: torin::Size,
    pub min_height: torin::Size,
    pub min_width: torin::Size,
    pub max_height: torin::Size,
    pub max_width: torin::Size,
    pub padding: torin::Paddings,
    pub direction: torin::Direction,
    pub node_id: NodeId,
    pub scroll_y: f32,
    pub scroll_x: f32,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for Size {
    type ParentDependencies = (Self,);

    type ChildDependencies = (Self,);

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
        ]))
        .with_tag()
        .with_text();

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let scale_factor = context.get::<f32>().unwrap();
        let torin_layout = context
            .get::<Arc<Mutex<Torin<NodeId, EmbeddedData>>>>()
            .unwrap();

        let mut width = torin::Size::default();
        let mut height = torin::Size::default();
        let mut min_height = torin::Size::default();
        let mut min_width = torin::Size::default();
        let mut max_height = torin::Size::default();
        let mut max_width = torin::Size::default();
        let mut padding = torin::Paddings::default();
        let mut scroll_y = 0.0;
        let mut scroll_x = 0.0;

        let mut direction = if let Some("label") = node_view.tag() {
            Direction::Horizontal
        } else if let Some("paragraph") = node_view.tag() {
            Direction::Horizontal
        } else if let Some("text") = node_view.tag() {
            Direction::Horizontal
        } else if node_view.text().is_some() {
            Direction::Horizontal
        } else {
            Direction::Vertical
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
                                min_height = new_min_height;
                            }
                        }
                    }
                    "min_width" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_min_width) = parse_size(attr, *scale_factor) {
                                min_width = new_min_width;
                            }
                        }
                    }
                    "max_height" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_max_height) = parse_size(attr, *scale_factor) {
                                max_height = new_max_height;
                            }
                        }
                    }
                    "max_width" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_max_width) = parse_size(attr, *scale_factor) {
                                max_width = new_max_width;
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
                                Direction::Horizontal
                            } else if attr == "both" {
                                Direction::Vertical
                            } else {
                                Direction::Vertical
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
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = (width != self.width)
            || (height != self.height)
            || (min_height != self.min_height)
            || (min_width != self.min_width)
            || (max_height != self.max_height)
            || (max_width != self.max_width)
            || (padding != self.padding)
            || (node_view.node_id() != self.node_id)
            || (direction != self.direction)
            || (scroll_x != self.scroll_x)
            || (scroll_y != self.scroll_y);

        if changed {
            let node = Node {
                width: width.clone(),
                height: height.clone(),
                direction: direction.clone(),
                padding,
                display: torin::Display::Normal,
                scroll_x: Length::new(scroll_x),
                scroll_y: Length::new(scroll_y),
            };

            if torin_layout.lock().unwrap().has(node_view.node_id()) {
                torin_layout
                    .lock()
                    .unwrap()
                    .set_node(node_view.node_id(), node)
            } else if let Some((parent_id,)) = parent {
                torin_layout.lock().unwrap().insert(
                    node_view.node_id(),
                    parent_id.node_id,
                    node,
                    EmbeddedData::default(),
                    children
                        .iter()
                        .map(|(c,)| c.node_id)
                        .collect::<Vec<NodeId>>(),
                )
            } else {
                torin_layout.lock().unwrap().add(
                    node_view.node_id(),
                    node,
                    EmbeddedData::default(),
                    None,
                    children
                        .iter()
                        .map(|(c,)| c.node_id)
                        .collect::<Vec<NodeId>>(),
                );
            }
        }

        *self = Self {
            width,
            height,
            min_height,
            min_width,
            max_height,
            max_width,
            padding,
            direction,
            node_id: node_view.node_id(),
            scroll_x,
            scroll_y,
        };
        changed
    }
}

pub fn parse_padding(padding: &str, scale_factor: f32) -> Option<Paddings> {
    let mut padding_config = (
        Length::new(0.0),
        Length::new(0.0),
        Length::new(0.0),
        Length::new(0.0),
    );
    let mut paddings = padding.split_ascii_whitespace();

    match paddings.clone().count() {
        // Same in each directions
        1 => {
            padding_config.0 = Length::new(paddings.next()?.parse::<f32>().ok()? * scale_factor);
            padding_config.1 = padding_config.0;
            padding_config.2 = padding_config.0;
            padding_config.3 = padding_config.0;
        }
        // By vertical and horizontal
        2 => {
            // Vertical
            padding_config.0 = Length::new(paddings.next()?.parse::<f32>().ok()? * scale_factor);
            padding_config.2 = padding_config.0;

            // Horizontal
            padding_config.1 = Length::new(paddings.next()?.parse::<f32>().ok()? * scale_factor);
            padding_config.3 = padding_config.1;
        }
        // Each directions
        4 => {
            padding_config.0 = Length::new(paddings.next()?.parse::<f32>().ok()? * scale_factor);
            padding_config.1 = Length::new(paddings.next()?.parse::<f32>().ok()? * scale_factor);
            padding_config.2 = Length::new(paddings.next()?.parse::<f32>().ok()? * scale_factor);
            padding_config.3 = Length::new(paddings.next()?.parse::<f32>().ok()? * scale_factor);
        }
        _ => {}
    }

    Some(padding_config)
}

pub fn parse_size(size: &str, scale_factor: f32) -> Option<torin::Size> {
    if size == "stretch" {
        Some(torin::Size::Percentage(Length::new(100.0)))
    } else if size == "auto" {
        Some(torin::Size::Inner)
    } else if size.contains("calc") {
        Some(torin::Size::DynamicCalculations(parse_calc(
            size,
            scale_factor,
        )?))
    } else if size.contains('%') {
        Some(torin::Size::Percentage(Length::new(
            size.replace('%', "").parse().ok()?,
        )))
    } else {
        Some(torin::Size::Pixels(Length::new(
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
