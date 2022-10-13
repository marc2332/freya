use dioxus_core::OwnedAttributeValue;
use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::{NodeDepState, ParentDepState, State};
use dioxus_native_core_macro::{sorted_str_slice, State};
use freya_elements::NodeLayout;
use freya_hooks::NodeRefWrapper;
use skia_safe::Color;
use std::fmt::Display;
use tokio::sync::mpsc::UnboundedSender;

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

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DirectionMode {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontStyle {
    pub color: Color,
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32, // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            font_family: "Fira Sans".to_string(),
            font_size: 16.0,
            line_height: 1.2,
        }
    }
}

#[derive(Default, Clone)]
pub struct References {
    pub node_ref: Option<UnboundedSender<NodeLayout>>,
}

#[derive(Clone, State, Default)]
pub struct NodeState {
    #[node_dep_state()]
    pub references: References,
    #[node_dep_state()]
    pub size: Size,
    #[node_dep_state()]
    pub style: Style,
    #[parent_dep_state(font_style)]
    pub font_style: FontStyle,
}

impl NodeState {
    pub fn set_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

#[derive(Default, Clone)]
pub struct Size {
    pub width: SizeMode,
    pub height: SizeMode,
    pub min_height: SizeMode,
    pub min_width: SizeMode,
    pub padding: (f32, f32, f32, f32),
    pub scroll_y: f32,
    pub scroll_x: f32,
    pub direction: DirectionMode,
}

impl Size {
    pub fn expanded() -> Self {
        Self {
            width: SizeMode::Percentage(100.0),
            height: SizeMode::Percentage(100.0),
            min_height: SizeMode::Manual(0.0),
            min_width: SizeMode::Manual(0.0),
            padding: (0.0, 0.0, 0.0, 0.0),
            scroll_y: 0.0,
            scroll_x: 0.0,
            direction: DirectionMode::Both,
        }
    }
}

impl NodeDepState<()> for References {
    type Ctx = ();

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!(["reference"])));

    fn reduce<'a>(&mut self, node: NodeView, _sibling: (), _ctx: &Self::Ctx) -> bool {
        let mut node_ref = None;

        if let Some(attributes) = node.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "reference" => {
                        if let OwnedAttributeValue::Any(v) = attr.value {
                            let ref_wrapper: &NodeRefWrapper =
                                v.value.as_any().downcast_ref().unwrap();
                            node_ref = Some(ref_wrapper.0.clone())
                        }
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = false;
        *self = Self { node_ref };
        changed
    }
}

/// Font style are inherited by default if not specified otherwise by some of the supported attributes.
impl ParentDepState for FontStyle {
    type Ctx = ();
    type DepState = Self;

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "color",
            "font_size",
            "font_family",
            "line_height"
        ])));

    fn reduce<'a>(
        &mut self,
        node: NodeView,
        parent: Option<&'a Self::DepState>,
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut font_style = parent.map(|c| c.clone()).unwrap_or_default();

        if let Some(attributes) = node.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "color" => {
                        if let Some(color) = attr.value.as_text() {
                            let new_color = parse_color(color);
                            if let Some(new_color) = new_color {
                                font_style.color = new_color;
                            }
                        }
                    }
                    "font_family" => {
                        if let Some(font_family) = attr.value.as_text() {
                            font_style.font_family = font_family.to_string();
                        }
                    }
                    "font_size" => {
                        if let Some(font_size) = attr.value.as_text() {
                            if let Ok(font_size) = font_size.parse::<f32>() {
                                font_style.font_size = font_size;
                            }
                        }
                    }
                    "line_height" => {
                        if let Some(line_height) = attr.value.as_text() {
                            if let Ok(line_height) = line_height.parse() {
                                font_style.line_height = line_height;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let changed = &font_style != self;
        *self = font_style;
        changed
    }
}

impl NodeDepState<()> for Size {
    type Ctx = ();

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "width",
            "height",
            "min_height",
            "min_width",
            "padding",
            "scroll_y",
            "scroll_x",
            "direction",
        ])))
        .with_tag();

    fn reduce<'a>(&mut self, node: NodeView, _sibling: (), _ctx: &Self::Ctx) -> bool {
        let mut width = SizeMode::default();
        let mut height = SizeMode::default();
        let mut min_height = SizeMode::default();
        let mut min_width = SizeMode::default();
        let mut padding = (0.0, 0.0, 0.0, 0.0);
        let mut scroll_y = 0.0;
        let mut scroll_x = 0.0;
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
                        if let Some(attr) = attr.value.as_text() {
                            if let Some(new_width) = parse_size(attr) {
                                width = new_width;
                            }
                        }
                    }
                    "height" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Some(new_height) = parse_size(attr) {
                                height = new_height;
                            }
                        }
                    }
                    "min_height" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Some(new_min_height) = parse_size(attr) {
                                min_height = new_min_height;
                            }
                        }
                    }
                    "min_width" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Some(new_min_width) = parse_size(attr) {
                                min_width = new_min_width;
                            }
                        }
                    }
                    "padding" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Ok(total_padding) = attr.parse::<f32>() {
                                let padding_for_side = total_padding / 2.0;
                                padding.0 = padding_for_side;
                                padding.1 = padding_for_side;
                                padding.2 = padding_for_side;
                                padding.3 = padding_for_side;
                            }
                        }
                    }
                    "scroll_y" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Ok(scroll) = attr.parse::<f32>() {
                                scroll_y = scroll;
                            }
                        }
                    }
                    "scroll_x" => {
                        if let Some(attr) = attr.value.as_text() {
                            if let Ok(scroll) = attr.parse::<f32>() {
                                scroll_x = scroll;
                            }
                        }
                    }
                    "direction" => {
                        if let Some(attr) = attr.value.as_text() {
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
            || (padding != self.padding)
            || (direction != self.direction)
            || (scroll_x != self.scroll_x)
            || (scroll_y != self.scroll_y);
        *self = Self {
            width,
            height,
            min_height,
            min_width,
            padding,
            scroll_y,
            scroll_x,
            direction,
        };
        changed
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

#[derive(Default, Clone, Debug)]
pub struct Style {
    pub background: Color,
    pub relative_layer: i16,
    pub shadow: ShadowSettings,
    pub radius: f32,
    pub image_data: Option<Vec<u8>>,
    pub svg_data: Option<Vec<u8>>,
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
            "svg_content"
        ])));

    fn reduce<'a>(&mut self, node: NodeView, _sibling: (), _ctx: &Self::Ctx) -> bool {
        let mut background = Color::TRANSPARENT;
        let mut relative_layer = 0;
        let mut shadow = ShadowSettings::default();
        let mut radius = 0.0;
        let mut image_data = None;
        let mut svg_data = None;

        if let Some(attributes) = node.attributes() {
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
                                radius = new_radius;
                            }
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
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
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
        };
        changed
    }
}

fn parse_shadow(value: &str) -> Option<ShadowSettings> {
    let value = value.to_string();
    let mut shadow_values = value.split_ascii_whitespace();
    Some(ShadowSettings {
        x: shadow_values.nth(0)?.parse().ok()?,
        y: shadow_values.nth(0)?.parse().ok()?,
        intensity: shadow_values.nth(0)?.parse().ok()?,
        size: shadow_values.nth(0)?.parse().ok()?,
        color: parse_color(shadow_values.nth(0)?)?,
    })
}

fn parse_rgb(color: &str) -> Option<Color> {
    let color = color.replace("rgb(", "").replace(")", "");
    let mut colors = color.split(",");

    let r = colors.nth(0)?.trim().parse().ok()?;
    let g = colors.nth(0)?.trim().parse().ok()?;
    let b = colors.nth(0)?.trim().parse().ok()?;
    Some(Color::from_rgb(r, g, b))
}

fn parse_color(color: &str) -> Option<Color> {
    match color {
        "red" => Some(Color::RED),
        "green" => Some(Color::GREEN),
        "blue" => Some(Color::BLUE),
        "yellow" => Some(Color::YELLOW),
        "black" => Some(Color::BLACK),
        "gray" => Some(Color::GRAY),
        "white" => Some(Color::WHITE),
        _ => parse_rgb(color),
    }
}

fn parse_size(size: &str) -> Option<SizeMode> {
    if size == "stretch" {
        Some(SizeMode::Percentage(100.0))
    } else if size == "auto" {
        Some(SizeMode::Auto)
    } else if size.contains("calc") {
        Some(SizeMode::Calculation(parse_calc(size).unwrap()))
    } else if size.contains("%") {
        Some(SizeMode::Percentage(size.replace("%", "").parse().ok()?))
    } else if size.contains("calc") {
        Some(SizeMode::Calculation(parse_calc(size).unwrap()))
    } else {
        Some(SizeMode::Manual(size.parse().ok()?))
    }
}

fn parse_calc(mut size: &str) -> Option<Vec<CalcType>> {
    let mut calcs = Vec::new();

    size = size.strip_prefix("calc(").unwrap();
    size = size.strip_suffix(")").unwrap();

    let vals = size.split_whitespace();

    for val in vals {
        if val.contains("%") {
            calcs.push(CalcType::Percentage(
                val.replace("%", "").parse().ok().unwrap(),
            ));
        } else if val == "+" {
            calcs.push(CalcType::Add);
        } else if val == "-" {
            calcs.push(CalcType::Sub);
        } else if val == "/" {
            calcs.push(CalcType::Div);
        } else if val == "*" {
            calcs.push(CalcType::Mul);
        } else {
            calcs.push(CalcType::Manual(val.parse().ok().unwrap()));
        }
    }

    Some(calcs)
}
