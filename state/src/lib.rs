use dioxus_native_core::state::{NodeDepState, ParentDepState, State};
use dioxus_native_core_macro::State;
use skia_safe::Color;
use std::fmt::Debug;

mod cursor;
mod custom_attributes;
mod font_style;
mod references;
mod scroll;
mod size;
mod style;

pub use cursor::*;
pub use custom_attributes::*;
pub use font_style::*;
pub use references::*;
pub use scroll::*;
pub use size::*;
pub use style::*;

#[derive(Clone, State, Default, Debug)]
#[state(custom_value = CustomAttributeValues)]
pub struct NodeState {
    #[parent_dep_state(cursor_settings)]
    pub cursor_settings: CursorSettings,
    #[parent_dep_state(references)]
    pub references: References,
    #[parent_dep_state(size, Arc<Mutex<LayoutManager>>)]
    pub size: Size,
    #[node_dep_state((), Arc<Mutex<LayoutManager>>)]
    pub scroll: Scroll,
    #[node_dep_state()]
    pub style: Style,
    #[parent_dep_state(font_style, Arc<Mutex<LayoutManager>>)]
    pub font_style: FontStyle,
}

impl NodeState {
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn iter(&self) -> NodeStateIterator {
        NodeStateIterator {
            state: self,
            curr: 0,
        }
    }

    /// Check if this NodeState has any sizing determined by it's children or not
    pub fn is_inner_static(&self) -> bool {
        if SizeMode::Auto == self.size.width {
            return false;
        }

        if SizeMode::Auto == self.size.height {
            return false;
        }

        true
    }
}

pub enum AttributeType<'a> {
    Color(&'a Color),
    Size(&'a SizeMode),
    Measure(f32),
    Measures((f32, f32, f32, f32)),
    Direction(&'a DirectionMode),
    Display(&'a DisplayMode),
    Shadow(&'a ShadowSettings),
    Text(&'a str),
}

pub struct NodeStateIterator<'a> {
    state: &'a NodeState,
    curr: usize,
}

impl<'a> Iterator for NodeStateIterator<'a> {
    type Item = (&'a str, AttributeType<'a>);

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match n {
            0 => Some(("width", AttributeType::Size(&self.state.size.width))),
            1 => Some(("height", AttributeType::Size(&self.state.size.height))),
            2 => Some(("min_width", AttributeType::Size(&self.state.size.min_width))),
            3 => Some((
                "min_height",
                AttributeType::Size(&self.state.size.min_height),
            )),
            4 => Some(("max_width", AttributeType::Size(&self.state.size.max_width))),
            5 => Some((
                "max_height",
                AttributeType::Size(&self.state.size.max_height),
            )),
            6 => Some((
                "direction",
                AttributeType::Direction(&self.state.size.direction),
            )),
            7 => Some(("padding", AttributeType::Measures(self.state.size.padding))),
            8 => Some((
                "background",
                AttributeType::Color(&self.state.style.background),
            )),
            9 => Some(("display", AttributeType::Display(&self.state.style.display))),
            10 => Some(("radius", AttributeType::Measure(self.state.style.radius))),
            11 => Some(("shadow", AttributeType::Shadow(&self.state.style.shadow))),
            12 => Some(("color", AttributeType::Color(&self.state.font_style.color))),
            13 => Some((
                "font_family",
                AttributeType::Text(&self.state.font_style.font_family),
            )),
            14 => Some((
                "font_size",
                AttributeType::Measure(self.state.font_style.font_size),
            )),
            15 => Some((
                "line_height",
                AttributeType::Measure(self.state.font_style.line_height),
            )),
            16 => Some((
                "scroll_x",
                AttributeType::Measure(self.state.scroll.scroll_x),
            )),
            17 => Some((
                "scroll_y",
                AttributeType::Measure(self.state.scroll.scroll_y),
            )),
            _ => None,
        }
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr += 1;

        self.nth(current)
    }
}

pub fn parse_rgb(color: &str) -> Option<Color> {
    let color = color.replace("rgb(", "").replace(')', "");
    let mut colors = color.split(',');

    let r = colors.next()?.trim().parse().ok()?;
    let g = colors.next()?.trim().parse().ok()?;
    let b = colors.next()?.trim().parse().ok()?;
    let a: Option<&str> = colors.next();
    if let Some(a) = a {
        let a = a.trim().parse::<u8>().ok()?;
        Some(Color::from_argb(a, r, g, b))
    } else {
        Some(Color::from_rgb(r, g, b))
    }
}

pub fn parse_color(color: &str) -> Option<Color> {
    match color {
        "inherit" => None,
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
