use freya_engine::prelude::{
    Canvas,
    FontCollection,
    FontMgr,
};
use freya_native_core::tags::TagName;
use torin::prelude::{
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
        self: Box<Self>,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        default_fonts: &[String],
        scale_factor: f32,
    );
}

pub trait ElementUtilsResolver {
    fn utils(&self) -> Option<Box<dyn ElementUtils>>;
}

impl ElementUtilsResolver for TagName {
    fn utils(&self) -> Option<Box<dyn ElementUtils>> {
        match self {
            TagName::Rect => Some(Box::new(RectElement)),
            TagName::Svg => Some(Box::new(SvgElement)),
            TagName::Paragraph => Some(Box::new(ParagraphElement)),
            TagName::Image => Some(Box::new(ImageElement)),
            TagName::Label => Some(Box::new(LabelElement)),
            _ => None,
        }
    }
}
