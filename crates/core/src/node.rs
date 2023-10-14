use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::prelude::DioxusNode;
use freya_engine::prelude::*;
use freya_node_state::{
    Border, CornerRadius, CursorSettings, Fill, FontStyleState, LayoutState, References, Shadow,
    Style, Transform,
};
use torin::{alignment::Alignment, direction::DirectionMode, gaps::Gaps, size::Size};

#[derive(Clone)]
pub struct NodeState {
    pub cursor: CursorSettings,
    pub font_style: FontStyleState,
    pub references: References,
    pub size: LayoutState,
    pub style: Style,
    pub transform: Transform,
}

pub fn get_node_state(node: &DioxusNode) -> NodeState {
    let cursor = node.get::<CursorSettings>().unwrap().clone();
    let font_style = node.get::<FontStyleState>().unwrap().clone();
    let references = node.get::<References>().unwrap().clone();
    let size = node.get::<LayoutState>().unwrap().clone();
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
            7 => Some(("padding", AttributeType::Measures(self.state.size.padding))),
            8 => Some((
                "main_alignment",
                AttributeType::Alignment(&self.state.size.main_alignment),
            )),
            9 => Some((
                "cross_alignment",
                AttributeType::Alignment(&self.state.size.cross_alignment),
            )),
            10 => {
                let background = &self.state.style.background;
                let fill = match *background {
                    Fill::Color(_) => AttributeType::Color(background.clone()),
                    Fill::LinearGradient(_) => AttributeType::LinearGradient(background.clone()),
                };
                Some(("background", fill))
            }
            11 => Some(("border", AttributeType::Border(&self.state.style.border))),
            12 => Some((
                "corner_radius",
                AttributeType::CornerRadius(self.state.style.corner_radius),
            )),
            13 => Some((
                "color",
                AttributeType::Color(self.state.font_style.color.into()),
            )),
            14 => Some((
                "font_family",
                AttributeType::Text(self.state.font_style.font_family.join(",")),
            )),
            15 => Some((
                "font_size",
                AttributeType::Measure(self.state.font_style.font_size),
            )),
            16 => Some((
                "line_height",
                AttributeType::Measure(self.state.font_style.line_height),
            )),
            17 => Some(("offset_x", AttributeType::Measure(self.state.size.offset_x))),
            18 => Some(("offset_y", AttributeType::Measure(self.state.size.offset_y))),
            n => {
                let shadows = &self.state.style.shadows;
                let shadow = shadows
                    .get(n - 19)
                    .map(|shadow| ("shadow", AttributeType::Shadow(shadow)));

                if shadow.is_some() {
                    shadow
                } else {
                    let text_shadows = &self.state.font_style.text_shadows;
                    text_shadows
                        .get(n - 19 + shadows.len())
                        .map(|text_shadow| ("text_shadow", AttributeType::TextShadow(text_shadow)))
                }
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
    Color(Fill),
    LinearGradient(Fill),
    Size(&'a Size),
    Measure(f32),
    Measures(Gaps),
    CornerRadius(CornerRadius),
    Direction(&'a DirectionMode),
    Alignment(&'a Alignment),
    Shadow(&'a Shadow),
    TextShadow(&'a TextShadow),
    Text(String),
    Border(&'a Border),
}
