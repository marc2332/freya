use std::fmt::Display;
use std::sync::{Arc, Mutex};

use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use freya_layout_common::LayoutMemorizer;

#[derive(Default, Clone)]
pub struct Size {
    pub width: SizeMode,
    pub height: SizeMode,
    pub min_height: SizeMode,
    pub min_width: SizeMode,
    pub max_height: SizeMode,
    pub max_width: SizeMode,
    pub padding: (f32, f32, f32, f32),
    pub direction: DirectionMode,
    pub id: usize,
}

impl Size {
    pub fn expanded() -> Self {
        Self {
            width: SizeMode::Percentage(100.0),
            height: SizeMode::Percentage(100.0),
            min_height: SizeMode::Manual(0.0),
            min_width: SizeMode::Manual(0.0),
            max_height: SizeMode::Manual(0.0),
            max_width: SizeMode::Manual(0.0),
            padding: (0.0, 0.0, 0.0, 0.0),
            direction: DirectionMode::Both,
            id: 0,
        }
    }
}

impl ParentDepState for Size {
    type Ctx = Arc<Mutex<LayoutMemorizer>>;
    type DepState = Self;

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

    fn reduce<'a>(
        &mut self,
        node: NodeView,
        _parent: Option<&'a Self::DepState>,
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

        for a in node.attributes() {
            match a.name {
                "width" => {
                    let attr = a.value.to_string();
                    if let Some(new_width) = parse_size(&attr) {
                        width = new_width;
                    }
                }
                "height" => {
                    let attr = a.value.to_string();
                    if let Some(new_height) = parse_size(&attr) {
                        height = new_height;
                    }
                }
                "min_height" => {
                    let attr = a.value.to_string();
                    if let Some(new_min_height) = parse_size(&attr) {
                        min_height = new_min_height;
                    }
                }
                "min_width" => {
                    let attr = a.value.to_string();
                    if let Some(new_min_width) = parse_size(&attr) {
                        min_width = new_min_width;
                    }
                }
                "max_height" => {
                    let attr = a.value.to_string();
                    if let Some(new_max_height) = parse_size(&attr) {
                        max_height = new_max_height;
                    }
                }
                "max_width" => {
                    let attr = a.value.to_string();
                    if let Some(new_max_width) = parse_size(&attr) {
                        max_width = new_max_width;
                    }
                }
                "padding" => {
                    let total_padding: f32 = a.value.to_string().parse().unwrap();
                    let padding_for_side = total_padding / 2.0;
                    padding.0 = padding_for_side;
                    padding.1 = padding_for_side;
                    padding.2 = padding_for_side;
                    padding.3 = padding_for_side;
                }
                "direction" => {
                    direction = if a.value.to_string() == "horizontal" {
                        DirectionMode::Horizontal
                    } else if a.value.to_string() == "both" {
                        DirectionMode::Both
                    } else {
                        DirectionMode::Vertical
                    };
                }
                _ => {
                    println!("Unsupported attribute <{}>", a.name);
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
            ctx.lock().unwrap().mark_as_dirty(node.id());
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
            id: node.id().0,
        };
        changed
    }
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
        Some(SizeMode::Manual(size.parse().ok()?))
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
            calcs.push(CalcType::Manual(val.parse().ok()?));
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
