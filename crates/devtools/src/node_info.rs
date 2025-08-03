use freya_core::{
    node_state_snapshot::NodeState,
    values::{
        Border,
        CornerRadius,
        Fill,
        Shadow,
        SvgPaint,
        TextAlign,
        TextOverflow,
        TextShadow,
    },
};
use freya_native_core::{
    NodeId,
    tags::TagName,
};
use serde::{
    Deserialize,
    Serialize,
};
use torin::{
    alignment::Alignment,
    direction::Direction,
    gaps::Gaps,
    prelude::{
        Content,
        LayoutNode,
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
    pub tag: TagName,
    pub height: u16,
    pub state: NodeState,
    pub layout_node: LayoutNode,
}

pub trait NodeStateAttributes {
    fn layout_attributes(&self) -> Vec<(&str, AttributeType)>;
    fn font_style_attributes(&self) -> Vec<(&str, AttributeType)>;
    fn style_attributes(&self) -> Vec<(&str, AttributeType)>;
    fn svg_attributes(&self) -> Vec<(&str, AttributeType)>;
}

impl NodeStateAttributes for NodeState {
    fn layout_attributes(&self) -> Vec<(&str, AttributeType)> {
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
        ]
    }
    fn style_attributes(&self) -> Vec<(&str, AttributeType)> {
        let mut attributes = vec![
            {
                let background = &self.style.background;
                let fill = match *background {
                    Fill::Color(_) => AttributeType::Color(background.clone()),
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

    fn font_style_attributes(&self) -> Vec<(&str, AttributeType)> {
        let mut attributes = vec![
            ("color", AttributeType::Color(self.font_style.color.into())),
            (
                "font_family",
                AttributeType::Text(self.font_style.font_family.join(",")),
            ),
            (
                "font_size",
                AttributeType::Measure(self.font_style.font_size),
            ),
            (
                "line_height",
                AttributeType::OptionalMeasure(self.font_style.line_height),
            ),
            (
                "text_align",
                AttributeType::TextAlignment(&self.font_style.text_align),
            ),
            (
                "text_overflow",
                AttributeType::TextOverflow(&self.font_style.text_overflow),
            ),
        ];

        let text_shadows = &self.font_style.text_shadows;

        for text_shadow in text_shadows.iter() {
            attributes.push(("text_shadow", AttributeType::TextShadow(text_shadow)));
        }

        attributes
    }

    fn svg_attributes(&self) -> Vec<(&str, AttributeType)> {
        vec![
            (
                "svg_fill",
                AttributeType::OptionalColor(self.svg.svg_fill.and_then(|fill| match fill {
                    SvgPaint::None => None,
                    SvgPaint::CurrentColor => Some(Fill::Color(self.font_style.color)),
                    SvgPaint::Color(color) => Some(Fill::Color(color)),
                })),
            ),
            (
                "svg_stroke",
                AttributeType::OptionalColor(self.svg.svg_stroke.and_then(|stroke| match stroke {
                    SvgPaint::None => None,
                    SvgPaint::CurrentColor => Some(Fill::Color(self.font_style.color)),
                    SvgPaint::Color(color) => Some(Fill::Color(color)),
                })),
            ),
        ]
    }
}

pub enum AttributeType<'a> {
    Color(Fill),
    OptionalColor(Option<Fill>),
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
}
