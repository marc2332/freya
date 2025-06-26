use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use torin::{
    alignment::Alignment,
    direction::Direction,
    gaps::Gaps,
    prelude::{
        Content,
        Position,
        VisibleSize,
    },
    size::Size,
};

use crate::{
    dom::DioxusNode,
    states::{
        AccessibilityNodeState,
        CursorState,
        FontStyleState,
        LayoutState,
        StyleState,
        SvgState,
        TransformState,
    },
    values::{
        Border,
        CornerRadius,
        Fill,
        Shadow,
        SvgPaint,
        TextOverflow,
    },
};

#[derive(Clone, PartialEq, Debug)]
pub struct NodeState {
    pub cursor: CursorState,
    pub font_style: FontStyleState,
    pub size: LayoutState,
    pub style: StyleState,
    pub transform: TransformState,
    pub accessibility: AccessibilityNodeState,
    pub svg: SvgState,
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
    let accessibility = node
        .get::<AccessibilityNodeState>()
        .as_deref()
        .cloned()
        .unwrap_or_default();
    let svg = node
        .get::<SvgState>()
        .as_deref()
        .cloned()
        .unwrap_or_default();

    NodeState {
        cursor,
        font_style,
        size,
        style,
        transform,
        accessibility,
        svg,
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
            (
                "visible_width",
                AttributeType::VisibleSize(&self.size.visible_width),
            ),
            (
                "visible_height",
                AttributeType::VisibleSize(&self.size.visible_height),
            ),
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
        ];

        let shadows = &self.style.shadows;
        for shadow in shadows.iter() {
            attributes.push(("shadow", AttributeType::Shadow(shadow)));
        }

        let borders = &self.style.borders;
        for border in borders.iter() {
            attributes.push(("border", AttributeType::Border(border)));
        }

        let text_shadows = &self.font_style.text_shadows;

        for text_shadow in text_shadows.iter() {
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
