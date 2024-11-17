use freya_engine::prelude::{
    Canvas,
    FontCollection,
    FontMgr,
};
use freya_native_core::{
    prelude::NodeImmutable,
    tags::TagName,
    NodeId,
};
use freya_node_state::{
    TransformState,
    ViewportState,
};
use torin::{
    prelude::{
        Area,
        AreaModel,
        CursorPoint,
        LayoutNode,
    },
    torin::Torin,
};

use super::*;
use crate::{
    dom::DioxusNode,
    render::ParagraphCache,
};

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
        paragraph_cache: &mut ParagraphCache,
    );

    fn element_drawing_area(
        &self,
        layout_node: &LayoutNode,
        _node_ref: &DioxusNode,
        _scale_factor: f32,
    ) -> Area {
        // Images neither SVG elements have support for shadows or borders, so its fine so simply return the visible area.
        layout_node.visible_area()
    }

    fn drawing_area_with_viewports(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        layout: &Torin<NodeId>,
        scale_factor: f32,
    ) -> Option<Area> {
        let mut drawing_area = self.drawing_area(layout_node, node_ref, scale_factor);
        let node_viewports = node_ref.get::<ViewportState>().unwrap();

        for viewport_id in &node_viewports.viewports {
            let viewport = layout.get(*viewport_id).unwrap().visible_area();
            drawing_area.clip(&viewport);
            if !viewport.intersects(&drawing_area) {
                return None;
            }
        }

        // Inflate the area by 1px in each side to cover potential off-bounds rendering caused by antialising
        Some(drawing_area.inflate(1.0, 1.0))
    }

    /// Measure the area for this element considering other
    /// factors like shadows or borders, which are not part of the layout.
    fn drawing_area(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        scale_factor: f32,
    ) -> Area {
        let drawing_area = self.element_drawing_area(layout_node, node_ref, scale_factor);
        let transform = node_ref.get::<TransformState>().unwrap();

        if !transform.rotations.is_empty() {
            let area = layout_node.visible_area();
            drawing_area.max_area_when_rotated(area.center())
        } else {
            drawing_area
        }
    }

    /// Check if this element requires any kind of special caching.
    /// Mainly used for text-like elements with shadows.
    /// See [crate::compositor::CompositorCache].
    /// Default to `false`.
    #[inline]
    fn element_needs_cached_area(&self, _node_ref: &DioxusNode) -> bool {
        false
    }

    #[inline]
    fn needs_cached_area(&self, node_ref: &DioxusNode) -> bool {
        let element_check = self.element_needs_cached_area(node_ref);

        let transform = node_ref.get::<TransformState>().unwrap();
        let rotate_effect = !transform.rotations.is_empty();

        element_check || rotate_effect
    }
}

pub trait ElementUtilsResolver {
    fn utils(&self) -> Option<ElementWithUtils>;
}

impl ElementUtilsResolver for TagName {
    #[inline]
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
        paragraph_cache: &mut ParagraphCache,
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
                paragraph_cache,
            ),
            Self::Svg(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
                paragraph_cache,
            ),
            Self::Paragraph(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
                paragraph_cache,
            ),
            Self::Image(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
                paragraph_cache,
            ),
            Self::Label(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
                paragraph_cache,
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

    fn needs_cached_area(&self, node_ref: &DioxusNode) -> bool {
        match self {
            Self::Rect(el) => el.needs_cached_area(node_ref),
            Self::Svg(el) => el.needs_cached_area(node_ref),
            Self::Paragraph(el) => el.needs_cached_area(node_ref),
            Self::Image(el) => el.needs_cached_area(node_ref),
            Self::Label(el) => el.needs_cached_area(node_ref),
        }
    }
}
