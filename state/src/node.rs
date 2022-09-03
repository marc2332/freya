use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::{ChildDepState, NodeDepState, State};
use dioxus_native_core_macro::{sorted_str_slice, State};
use skia_safe::Color;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub enum SizeMode {
    #[default]
    Auto,
    Percentage(f32),
    Manual(f32),
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DirectionMode {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

#[derive(Debug, Clone, State, Default)]
pub struct NodeState {
    #[child_dep_state(size)]
    pub size: Size,
    #[node_dep_state()]
    pub style: Style,
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub width: SizeMode,
    pub height: SizeMode,
    pub padding: (f32, f32, f32, f32),
    pub scroll_y: f32,
    pub scroll_x: f32,
    pub direction: DirectionMode,
}

impl ChildDepState for Size {
    type Ctx = ();

    type DepState = Self;

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "width",
            "height",
            "padding",
            "scroll_y",
            "scroll_x",
            "direction"
        ])))
        .with_text()
        .with_tag();
    fn reduce<'a>(
        &mut self,
        node: NodeView,
        _children: impl Iterator<Item = &'a Self::DepState>,
        _ctx: &Self::Ctx,
    ) -> bool
    where
        Self::DepState: 'a,
    {
        let mut width = SizeMode::default();
        let mut height = SizeMode::default();
        let mut padding = (0.0, 0.0, 0.0, 0.0);
        let mut scroll_y = 0.0;
        let mut scroll_x = 0.0;
        let mut direction = DirectionMode::Vertical;

        // if the node contains a width or height attribute it overrides the other size
        for a in node.attributes() {
            match a.name {
                "width" => {
                    let attr = a.value.to_string();
                    if &attr == "stretch" {
                        width = SizeMode::Percentage(100.0);
                    } else if &attr == "auto" {
                        width = SizeMode::Auto;
                    } else if attr.contains("%") {
                        width = SizeMode::Percentage(attr.replace("%", "").parse().unwrap());
                    } else {
                        width = SizeMode::Manual(attr.parse().unwrap());
                    }
                }
                "height" => {
                    let attr = a.value.to_string();
                    if &attr == "stretch" {
                        height = SizeMode::Percentage(100.0);
                    } else if &attr == "auto" {
                        height = SizeMode::Auto;
                    } else if attr.contains("%") {
                        height = SizeMode::Percentage(attr.replace("%", "").parse().unwrap());
                    } else {
                        height = SizeMode::Manual(attr.parse().unwrap());
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
                "scroll_y" => {
                    let scroll: f32 = a.value.to_string().parse().unwrap();
                    scroll_y = scroll;
                }
                "scroll_x" => {
                    let scroll: f32 = a.value.to_string().parse().unwrap();
                    scroll_x = scroll;
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
            || (padding != self.padding)
            || (direction != self.direction)
            || (scroll_x != self.scroll_x)
            || (scroll_y != self.scroll_y);
        *self = Self {
            width,
            height,
            padding,
            scroll_y,
            scroll_x,
            direction,
        };
        changed
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct ShadowSettings {
    pub x: f32,
    pub y: f32,
    pub intensity: u8,
    pub size: f32,
    pub color: Color,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Style {
    pub background: Color,
    pub relative_layer: i16,
    pub shadow: ShadowSettings,
    pub radius: f32,
}

impl NodeDepState<()> for Style {
    type Ctx = ();

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "background",
            "layer",
            "shadow",
            "radius"
        ])))
        .with_text();
    fn reduce<'a>(&mut self, node: NodeView, _sibling: (), _ctx: &Self::Ctx) -> bool {
        let mut background = Color::TRANSPARENT;
        let mut relative_layer = 0;
        let mut shadow = ShadowSettings::default();
        let mut radius = 0.0;

        for attr in node.attributes() {
            match attr.name {
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
                _ => {
                    println!("Unsupported attribute <{}>", attr.name);
                }
            }
        }

        let changed = (background != self.background) || (relative_layer != self.relative_layer);
        *self = Self {
            background,
            relative_layer,
            shadow,
            radius,
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
