use freya_core::{
    integration::*,
    prelude::{
        Border,
        Color,
        CornerRadius,
        CursorMode,
        Fill,
        FontSlant,
        Shadow,
        TextAlign,
        TextHeightBehavior,
        TextOverflow,
        TextShadow,
        VerticalAlign,
    },
};
use serde::{
    Deserialize,
    Serialize,
};
use torin::{
    alignment::Alignment,
    direction::Direction,
    gaps::Gaps,
    geometry::Length,
    prelude::{
        Area,
        AreaOf,
        Content,
        Inner,
        Position,
        VisibleSize,
    },
    size::Size,
};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct NodeInfo {
    pub window_id: u64,
    pub is_window: bool,
    pub node_id: NodeId,
    pub parent_id: Option<NodeId>,
    pub children_len: usize,
    pub height: u16,
    pub layer: i16,
    pub state: NodeState,
    pub area: Area,
    pub inner_area: AreaOf<Inner>,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct NodeState {
    pub style: StyleState,
    pub text_style: TextStyleState,
    pub layout: torin::node::Node,
    pub accessibility: AccessibilityData,
}

pub trait NodeStateAttributes {
    fn layout_attributes(&'_ self) -> Vec<(&'_ str, AttributeType<'_>)>;
    fn text_style_attributes(&'_ self) -> Vec<(&'_ str, AttributeType<'_>)>;
    fn style_attributes(&'_ self) -> Vec<(&'_ str, AttributeType<'_>)>;
}

impl NodeStateAttributes for NodeState {
    fn layout_attributes(&'_ self) -> Vec<(&'_ str, AttributeType<'_>)> {
        vec![
            ("width", AttributeType::Size(&self.layout.width)),
            ("height", AttributeType::Size(&self.layout.height)),
            ("min_width", AttributeType::Size(&self.layout.minimum_width)),
            (
                "min_height",
                AttributeType::Size(&self.layout.minimum_height),
            ),
            ("max_width", AttributeType::Size(&self.layout.maximum_width)),
            (
                "max_height",
                AttributeType::Size(&self.layout.maximum_height),
            ),
            (
                "visible_width",
                AttributeType::VisibleSize(&self.layout.visible_width),
            ),
            (
                "visible_height",
                AttributeType::VisibleSize(&self.layout.visible_height),
            ),
            (
                "direction",
                AttributeType::Direction(&self.layout.direction),
            ),
            ("padding", AttributeType::Measures(self.layout.padding)),
            ("margin", AttributeType::Measures(self.layout.margin)),
            ("position", AttributeType::Position(&self.layout.position)),
            (
                "main_alignment",
                AttributeType::Alignment(&self.layout.main_alignment),
            ),
            (
                "cross_alignment",
                AttributeType::Alignment(&self.layout.cross_alignment),
            ),
            (
                "offset_x",
                AttributeType::Measure(self.layout.offset_x.get()),
            ),
            (
                "offset_y",
                AttributeType::Measure(self.layout.offset_y.get()),
            ),
            ("content", AttributeType::Content(&self.layout.content)),
            ("spacing", AttributeType::Length(self.layout.spacing)),
        ]
    }
    fn style_attributes(&'_ self) -> Vec<(&'_ str, AttributeType<'_>)> {
        let mut attributes = vec![
            {
                let background = &self.style.background;
                let fill = match *background {
                    Fill::Color(background) => AttributeType::Color(background),
                    Fill::LinearGradient(_) => AttributeType::Gradient(background.clone()),
                    Fill::RadialGradient(_) => AttributeType::Gradient(background.clone()),
                    Fill::ConicGradient(_) => AttributeType::Gradient(background.clone()),
                };
                ("background", fill)
            },
            (
                "corner_radius",
                AttributeType::CornerRadius(self.style.corner_radius),
            ),
        ];

        let shadows = &self.style.shadows;
        for shadow in shadows.iter() {
            attributes.push(("shadow", AttributeType::Shadow(shadow)));
        }

        let borders = &self.style.borders;
        for border in borders.iter() {
            attributes.push(("border", AttributeType::Border(border)));
        }

        attributes
    }

    fn text_style_attributes(&'_ self) -> Vec<(&'_ str, AttributeType<'_>)> {
        let mut attributes = vec![
            ("color", AttributeType::Color(self.text_style.color)),
            (
                "font_family",
                AttributeType::Text(self.text_style.font_families.join(", ")),
            ),
            (
                "font_size",
                AttributeType::Measure(f32::from(self.text_style.font_size)),
            ),
            (
                "text_align",
                AttributeType::TextAlignment(&self.text_style.text_align),
            ),
            (
                "text_overflow",
                AttributeType::TextOverflow(&self.text_style.text_overflow),
            ),
            (
                "text_height",
                AttributeType::TextHeightBehavior(&self.text_style.text_height),
            ),
            (
                "font_slant",
                AttributeType::FontSlant(self.text_style.font_slant),
            ),
            (
                "font_weight",
                AttributeType::Measure(self.text_style.font_weight.into()),
            ),
            (
                "font_width",
                AttributeType::Measure(self.text_style.font_width.into()),
            ),
        ];

        for shadow in self.style.shadows.iter() {
            attributes.push(("shadow", AttributeType::Shadow(shadow)));
        }

        for text_shadow in self.text_style.text_shadows.iter() {
            attributes.push(("text_shadow", AttributeType::TextShadow(text_shadow)));
        }

        attributes
    }
}

pub enum AttributeType<'a> {
    Color(Color),
    OptionalColor(Option<Color>),
    Gradient(Fill),
    Size(&'a Size),
    VisibleSize(&'a VisibleSize),
    Measure(f32),
    OptionalMeasure(Option<f32>),
    Measures(Gaps),
    CornerRadius(CornerRadius),
    Direction(&'a Direction),
    Position(&'a Position),
    Content(&'a Content),
    Alignment(&'a Alignment),
    Shadow(&'a Shadow),
    TextShadow(&'a TextShadow),
    Text(String),
    Border(&'a Border),
    TextAlignment(&'a TextAlign),
    TextOverflow(&'a TextOverflow),
    TextHeightBehavior(&'a TextHeightBehavior),
    FontSlant(FontSlant),
    Length(Length),
    Layer(i16),
    CursorMode(CursorMode),
    VerticalAlign(VerticalAlign),
}
