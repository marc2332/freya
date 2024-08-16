use freya_engine::prelude::{
    Canvas,
    FontCollection,
    FontMgr,
};
use freya_native_core::tags::TagName;
use torin::prelude::{
    Area,
    CursorPoint,
    LayoutNode,
};

use super::*;
use crate::dom::DioxusNode;

pub trait ElementUtils {
    fn is_point_inside_area(
        &self,
        point: &CursorPoint,
        _node_ref: &DioxusNode,
        layout_node: &LayoutNode,
        _scale_factor: f32,
    ) -> bool {
        layout_node.area.contains(point.to_f32())
    }

    fn clip(
        &self,
        _layout_node: &LayoutNode,
        _node_ref: &DioxusNode,
        _canvas: &Canvas,
        _scale_factor: f32,
    ) {
    }

    #[allow(clippy::too_many_arguments)]
    fn render(
        self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        default_fonts: &[String],
        scale_factor: f32,
    );

    fn drawing_area(
        &self,
        layout_node: &LayoutNode,
        _node_ref: &DioxusNode,
        _scale_factor: f32,
    ) -> Area {
        layout_node.visible_area()
    }
}

pub trait ElementUtilsResolver {
    fn utils(&self) -> Option<ElementWithUtils>;
}

impl ElementUtilsResolver for TagName {
    fn utils(&self) -> Option<ElementWithUtils> {
        match self {
            TagName::Rect => Some(ElementWithUtils::Rect(RectElement)),
            TagName::Svg => Some(ElementWithUtils::Svg(SvgElement)),
            TagName::Paragraph => Some(ElementWithUtils::Paragraph(ParagraphElement)),
            TagName::Image => Some(ElementWithUtils::Image(ImageElement)),
            TagName::Label => Some(ElementWithUtils::Label(LabelElement)),
            _ => None,
        }
    }
}

pub enum ElementWithUtils {
    Rect(RectElement),
    Svg(SvgElement),
    Paragraph(ParagraphElement),
    Image(ImageElement),
    Label(LabelElement),
}

impl ElementUtils for ElementWithUtils {
    fn clip(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        scale_factor: f32,
    ) {
        match self {
            Self::Rect(el) => el.clip(layout_node, node_ref, canvas, scale_factor),
            Self::Svg(el) => el.clip(layout_node, node_ref, canvas, scale_factor),
            Self::Paragraph(el) => el.clip(layout_node, node_ref, canvas, scale_factor),
            Self::Image(el) => el.clip(layout_node, node_ref, canvas, scale_factor),
            Self::Label(el) => el.clip(layout_node, node_ref, canvas, scale_factor),
        }
    }

    fn is_point_inside_area(
        &self,
        point: &CursorPoint,
        node_ref: &DioxusNode,
        layout_node: &LayoutNode,
        scale_factor: f32,
    ) -> bool {
        match self {
            Self::Rect(el) => el.is_point_inside_area(point, node_ref, layout_node, scale_factor),
            Self::Svg(el) => el.is_point_inside_area(point, node_ref, layout_node, scale_factor),
            Self::Paragraph(el) => {
                el.is_point_inside_area(point, node_ref, layout_node, scale_factor)
            }
            Self::Image(el) => el.is_point_inside_area(point, node_ref, layout_node, scale_factor),
            Self::Label(el) => el.is_point_inside_area(point, node_ref, layout_node, scale_factor),
        }
    }

    fn render(
        self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        default_fonts: &[String],
        scale_factor: f32,
    ) {
        match self {
            Self::Rect(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
            ),
            Self::Svg(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
            ),
            Self::Paragraph(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
            ),
            Self::Image(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
            ),
            Self::Label(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
            ),
        }
    }

    fn drawing_area(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        scale_factor: f32,
    ) -> Area {
        match self {
            Self::Rect(el) => el.drawing_area(layout_node, node_ref, scale_factor),
            Self::Svg(el) => el.drawing_area(layout_node, node_ref, scale_factor),
            Self::Paragraph(el) => el.drawing_area(layout_node, node_ref, scale_factor),
            Self::Image(el) => el.drawing_area(layout_node, node_ref, scale_factor),
            Self::Label(el) => el.drawing_area(layout_node, node_ref, scale_factor),
        }
    }
}
