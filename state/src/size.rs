use std::fmt::Display;

use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use freya_common::LayoutNotifier;

use crate::CustomAttributeValues;

#[derive(Default, Clone, Debug)]
pub struct Size {
    pub width: SizeMode,
    pub height: SizeMode,
    pub min_height: SizeMode,
    pub min_width: SizeMode,
    pub max_height: SizeMode,
    pub max_width: SizeMode,
    pub padding: (f32, f32, f32, f32),
    pub direction: DirectionMode,
}

impl ParentDepState<CustomAttributeValues> for Size {
    type Ctx = LayoutNotifier;
    type DepState = (Self,);

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "width",
            "height",
            "min_height",
            "min_width",
            "max_height",
            "max_width",
            "padding",
            "direction",
        ])))
        .with_text()
        .with_tag();

    fn reduce(
        &mut self,
        node: NodeView<CustomAttributeValues>,
        _parent: Option<(&Self,)>,
        ctx: &Self::Ctx,
    ) -> bool {
        let mut width = SizeMode::default();
        let mut height = SizeMode::default();
        let mut min_height = SizeMode::default();
        let mut min_width = SizeMode::default();
        let mut max_height = SizeMode::default();
        let mut max_width = SizeMode::default();
        let mut padding = (0.0, 0.0, 0.0, 0.0);
        let mut direction = if let Some("label") = node.tag() {
            DirectionMode::Both
        } else if let Some("paragraph") = node.tag() {
            DirectionMode::Both
        } else if let Some("text") = node.tag() {
            DirectionMode::Both
        } else if node.text().is_some() {
            DirectionMode::Both
        } else {
            DirectionMode::Vertical
        };

        if let Some(attributes) = node.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "width" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_width) = parse_size(attr) {
                                width = new_width;
                            }
                        }
                    }
                    "height" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_height) = parse_size(attr) {
                                height = new_height;
                            }
                        }
                    }
                    "min_height" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_min_height) = parse_size(attr) {
                                min_height = new_min_height;
                            }
                        }
                    }
                    "min_width" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_min_width) = parse_size(attr) {
                                min_width = new_min_width;
                            }
                        }
                    }
                    "max_height" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_max_height) = parse_size(attr) {
                                max_height = new_max_height;
                            }
                        }
                    }
                    "max_width" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_max_width) = parse_size(attr) {
                                max_width = new_max_width;
                            }
                        }
                    }
                    "padding" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(paddings) = parse_padding(attr) {
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
            || (direction != self.direction);

        if changed {
            *ctx.lock().unwrap() = true;
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
        };
        changed
    }
}

pub fn parse_padding(padding: &str) -> Option<(f32, f32, f32, f32)> {
    let mut padding_config = (0.0, 0.0, 0.0, 0.0);
    let mut paddings = padding.split_ascii_whitespace();

    match paddings.clone().count() {
        // Same in each directions
        1 => {
            padding_config.0 = paddings.next()?.parse::<f32>().ok()? * 2.0;
            padding_config.1 = padding_config.0;
            padding_config.2 = padding_config.0;
            padding_config.3 = padding_config.0;
        }
        // By vertical and horizontal
        2 => {
            // Vertical
            padding_config.0 = paddings.next()?.parse::<f32>().ok()? * 2.0;
            padding_config.2 = padding_config.0;

            // Horizontal
            padding_config.1 = paddings.next()?.parse::<f32>().ok()? * 2.0;
            padding_config.3 = padding_config.1;
        }
        // Each directions
        4 => {
            padding_config.0 = paddings.next()?.parse::<f32>().ok()? * 2.0;
            padding_config.1 = paddings.next()?.parse::<f32>().ok()? * 2.0;
            padding_config.2 = paddings.next()?.parse::<f32>().ok()? * 2.0;
            padding_config.3 = paddings.next()?.parse::<f32>().ok()? * 2.0;
        }
        _ => {}
    }

    Some(padding_config)
}

pub fn parse_size(size: &str) -> Option<SizeMode> {
    if size == "stretch" {
        Some(SizeMode::Percentage(100.0))
    } else if size == "auto" {
        Some(SizeMode::Auto)
    } else if size.contains("calc") {
        Some(SizeMode::Calculation(parse_calc(size)?))
    } else if size.contains('%') {
        Some(SizeMode::Percentage(size.replace('%', "").parse().ok()?))
    } else if size.contains("calc") {
        Some(SizeMode::Calculation(parse_calc(size)?))
    } else {
        Some(SizeMode::Manual((size.parse::<f32>().ok()?) * 2.0))
    }
}

pub fn parse_calc(mut size: &str) -> Option<Vec<CalcType>> {
    let mut calcs = Vec::new();

    size = size.strip_prefix("calc(")?;
    size = size.strip_suffix(')')?;

    let vals = size.split_whitespace();

    for val in vals {
        if val.contains('%') {
            calcs.push(CalcType::Percentage(val.replace('%', "").parse().ok()?));
        } else if val == "+" {
            calcs.push(CalcType::Add);
        } else if val == "-" {
            calcs.push(CalcType::Sub);
        } else if val == "/" {
            calcs.push(CalcType::Div);
        } else if val == "*" {
            calcs.push(CalcType::Mul);
        } else {
            calcs.push(CalcType::Manual(val.parse::<f32>().ok()? * 2.0));
        }
    }

    Some(calcs)
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DirectionMode {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

impl Display for DirectionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionMode::Vertical => f.write_str("vertical"),
            DirectionMode::Horizontal => f.write_str("horizontal"),
            DirectionMode::Both => f.write_str("both"),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub enum SizeMode {
    #[default]
    Auto,
    Calculation(Vec<CalcType>),
    Percentage(f32),
    Manual(f32),
}

impl Display for SizeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SizeMode::Auto => f.write_str("auto"),
            SizeMode::Manual(s) => f.write_fmt(format_args!("{s}")),
            SizeMode::Calculation(calcs) => f.write_fmt(format_args!(
                "calc({})",
                calcs
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )),
            SizeMode::Percentage(p) => f.write_fmt(format_args!("{p}%")),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CalcType {
    Sub,
    Mul,
    Div,
    Add,
    Percentage(f32),
    Manual(f32),
}

impl Display for CalcType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcType::Sub => f.write_str("-"),
            CalcType::Mul => f.write_str("*"),
            CalcType::Div => f.write_str("/"),
            CalcType::Add => f.write_str("+"),
            CalcType::Percentage(p) => f.write_fmt(format_args!("{p}%")),
            CalcType::Manual(s) => f.write_fmt(format_args!("{s}")),
        }
    }
}
