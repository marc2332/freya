use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::{
    Border,
    CornerRadius,
    CursorState,
    Fill,
    FontStyleState,
    LayoutState,
    ReferencesState,
    Shadow,
    StyleState,
    TextOverflow,
    TransformState,
};
use torin::{
    alignment::Alignment,
    direction::DirectionMode,
    gaps::Gaps,
    prelude::{
        Content,
        Position,
    },
    size::Size,
};

use crate::dom::DioxusNode;

#[derive(Clone, PartialEq)]
pub struct NodeState {
    pub cursor: CursorState,
    pub font_style: FontStyleState,
    pub references: ReferencesState,
    pub size: LayoutState,
    pub style: StyleState,
    pub transform: TransformState,
}

pub fn get_node_state(node: &DioxusNode) -> NodeState {
    let cursor = node
        .get::<CursorState>()
        .as_deref()
        .cloned()
        .unwrap_or_default();
    let font_style = node
        .get::<FontStyleState>()
        .as_deref()
        .cloned()
        .unwrap_or_default();
    let references = node
        .get::<ReferencesState>()
        .as_deref()
        .cloned()
        .unwrap_or_default();
    let size = node
        .get::<LayoutState>()
        .as_deref()
        .cloned()
        .unwrap_or_default();
    let style = node
        .get::<StyleState>()
        .as_deref()
        .cloned()
        .unwrap_or_default();
    let transform = node
        .get::<TransformState>()
        .as_deref()
        .cloned()
        .unwrap_or_default();

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
    pub fn attributes(&self) -> Vec<(&str, AttributeType)> {
        let mut attributes = vec![
            ("width", AttributeType::Size(&self.size.width)),
            ("height", AttributeType::Size(&self.size.height)),
            ("min_width", AttributeType::Size(&self.size.minimum_width)),
            ("min_height", AttributeType::Size(&self.size.minimum_height)),
            ("max_width", AttributeType::Size(&self.size.maximum_width)),
            ("max_height", AttributeType::Size(&self.size.maximum_height)),
            ("direction", AttributeType::Direction(&self.size.direction)),
            ("padding", AttributeType::Measures(self.size.padding)),
            ("margin", AttributeType::Measures(self.size.margin)),
            ("position", AttributeType::Position(&self.size.position)),
            (
                "main_alignment",
                AttributeType::Alignment(&self.size.main_alignment),
            ),
            (
                "cross_alignment",
                AttributeType::Alignment(&self.size.cross_alignment),
            ),
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
            ("offset_x", AttributeType::Measure(self.size.offset_x.get())),
            ("offset_y", AttributeType::Measure(self.size.offset_y.get())),
            ("content", AttributeType::Content(&self.size.content)),
            (
                "fill",
                AttributeType::OptionalColor(self.style.svg_fill.map(|color| color.into())),
            ),
            (
                "svg_stroke",
                AttributeType::OptionalColor(self.style.svg_stroke.map(|color| color.into())),
            ),
        ];

        let shadows = &self.style.shadows;
        for shadow in shadows {
            attributes.push(("shadow", AttributeType::Shadow(shadow)));
        }

        let borders = &self.style.borders;
        for border in borders {
            attributes.push(("border", AttributeType::Border(border)));
        }

        let text_shadows = &self.font_style.text_shadows;

        for text_shadow in text_shadows {
            attributes.push(("text_shadow", AttributeType::TextShadow(text_shadow)));
        }

        attributes
    }
}

pub enum AttributeType<'a> {
    Color(Fill),
    OptionalColor(Option<Fill>),
    Gradient(Fill),
    Size(&'a Size),
    Measure(f32),
    OptionalMeasure(Option<f32>),
    Measures(Gaps),
    CornerRadius(CornerRadius),
    Direction(&'a DirectionMode),
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

pub trait ExternalPretty {
    fn pretty(&self) -> String;
}

impl ExternalPretty for TextAlign {
    fn pretty(&self) -> String {
        match self {
            TextAlign::Left => "left".to_string(),
            TextAlign::Right => "right".to_string(),
            TextAlign::Center => "center".to_string(),
            TextAlign::Justify => "justify".to_string(),
            TextAlign::Start => "start".to_string(),
            TextAlign::End => "end".to_string(),
        }
    }
}
