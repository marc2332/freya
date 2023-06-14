use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::prelude::DioxusNode;
use freya_node_state::{
    BorderSettings, CursorSettings, FontStyle, References, ShadowSettings, SizeState, Style,
    Transform,
};
use skia_safe::Color;
use torin::{
    direction::DirectionMode, display::DisplayMode, padding::Paddings, radius::Radius, size::Size,
};

#[derive(Clone)]
pub struct NodeState {
    pub cursor: CursorSettings,
    pub font_style: FontStyle,
    pub references: References,
    pub size: SizeState,
    pub style: Style,
    pub transform: Transform,
}

pub fn get_node_state(node: &DioxusNode) -> NodeState {
    let cursor = node.get::<CursorSettings>().unwrap().clone();
    let font_style = node.get::<FontStyle>().unwrap().clone();
    let references = node.get::<References>().unwrap().clone();
    let size = node.get::<SizeState>().unwrap().clone();
    let style = node.get::<Style>().unwrap().clone();
    let transform = node.get::<Transform>().unwrap().clone();

    NodeState {
        cursor,
        font_style,
        references,
        size,
        style,
        transform,
    }
}

impl NodeState {
    pub fn iter(&self) -> NodeStateIterator {
        NodeStateIterator {
            state: self,
            curr: 0,
        }
    }
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
            2 => Some((
                "min_width",
                AttributeType::Size(&self.state.size.minimum_width),
            )),
            3 => Some((
                "min_height",
                AttributeType::Size(&self.state.size.minimum_height),
            )),
            4 => Some((
                "max_width",
                AttributeType::Size(&self.state.size.maximum_width),
            )),
            5 => Some((
                "max_height",
                AttributeType::Size(&self.state.size.maximum_height),
            )),
            6 => Some((
                "direction",
                AttributeType::Direction(&self.state.size.direction),
            )),
            7 => Some(("padding", AttributeType::Paddings(self.state.size.padding))),
            8 => Some(("display", AttributeType::Display(&self.state.size.display))),
            9 => Some((
                "background",
                AttributeType::Color(&self.state.style.background),
            )),
            10 => Some(("border", AttributeType::Border(&self.state.style.border))),
            11 => Some(("radius", AttributeType::Radius(self.state.style.radius))),
            12 => Some(("color", AttributeType::Color(&self.state.font_style.color))),
            13 => Some((
                "font_family",
                AttributeType::Text(self.state.font_style.font_family.join(",")),
            )),
            14 => Some((
                "font_size",
                AttributeType::Measure(self.state.font_style.font_size),
            )),
            15 => Some((
                "line_height",
                AttributeType::Measure(self.state.font_style.line_height),
            )),
            16 => Some(("offset_x", AttributeType::Measure(self.state.size.offset_x))),
            17 => Some(("offset_y", AttributeType::Measure(self.state.size.offset_y))),
            n => {
                let shadows = &self.state.style.shadows;
                shadows
                    .get(n - 18)
                    .map(|shadow| ("shadow", AttributeType::Shadow(shadow)))
            }
        }
    }

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr += 1;

        self.nth(current)
    }
}

pub enum AttributeType<'a> {
    Color(&'a Color),
    Size(&'a Size),
    Measure(f32),
    Paddings(Paddings),
    Radius(Radius),
    Direction(&'a DirectionMode),
    Display(&'a DisplayMode),
    Shadow(&'a ShadowSettings),
    Text(String),
    Border(&'a BorderSettings),
}
