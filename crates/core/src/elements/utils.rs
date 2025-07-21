use freya_engine::prelude::{
    Canvas,
    FontCollection,
    FontMgr,
};
use freya_native_core::{
    tags::TagName,
    NodeId,
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
    dom::{
        DioxusNode,
        ImagesCache,
    },
    states::{
        StyleState,
        TransformState,
        ViewportState,
    },
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
        fallback_fonts: &[String],
        images_cache: &mut ImagesCache,
        scale_factor: f32,
    );

    fn element_drawing_area(
        &self,
        layout_node: &LayoutNode,
        _node_ref: &DioxusNode,
        _scale_factor: f32,
        _node_style: &StyleState,
    ) -> Area {
        // Images neither SVG elements have support for shadows or borders, so its fine so simply return the visible area.
        layout_node.visible_area()
    }

    /// Check if this element requires any kind of special caching.
    /// Mainly used for text-like elements with shadows.
    /// See [crate::render::CompositorCache].
    /// Default to `false`.
    #[inline]
    fn element_needs_cached_area(&self, _node_ref: &DioxusNode, _style_state: &StyleState) -> bool {
        false
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

impl ElementWithUtils {
    /// Measure the area that this element once rendered will use.
    /// This takes into consideration things like borders and shadows.
    pub fn get_drawing_area(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        layout: &Torin<NodeId>,
        scale_factor: f32,
        node_style: &StyleState,
        transform_state: &TransformState,
    ) -> Area {
        let mut drawing_area =
            self.element_drawing_area(layout_node, node_ref, scale_factor, node_style);

        // Apply scale effect
        for (id, scale_x, scale_y) in &transform_state.scales {
            let layout_node = layout.get(*id).unwrap();
            let center = layout_node.area.center();
            drawing_area = drawing_area.translate(-center.to_vector());
            drawing_area = drawing_area.scale(*scale_x, *scale_y);
            drawing_area = drawing_area.translate(center.to_vector());
            drawing_area = drawing_area.inflate(1.0, 1.0);
        }

        if !transform_state.rotations.is_empty() {
            // Apply rotation effect
            let area = layout_node.visible_area();
            drawing_area.max_area_when_rotated(area.center())
        } else {
            drawing_area
        }
    }

    /// Just like [Self::get_drawing_area] but only if all the viewports allow the element to be visible.
    #[allow(clippy::too_many_arguments)]
    pub fn get_drawing_area_if_viewports_allow(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        layout: &Torin<NodeId>,
        scale_factor: f32,
        node_style: &StyleState,
        node_viewports: &ViewportState,
        transform_state: &TransformState,
    ) -> Option<Area> {
        let mut drawing_area = self.get_drawing_area(
            layout_node,
            node_ref,
            layout,
            scale_factor,
            node_style,
            transform_state,
        );

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

    /// Check if this element needs its drawing area to be cached in case it needs to be
    /// invalidated in the next frame due to potential rotation or scale changes.
    #[inline]
    pub fn needs_cached_drawing_area(
        &self,
        node_ref: &DioxusNode,
        transform_state: &TransformState,
        style_state: &StyleState,
    ) -> bool {
        let element_check = self.element_needs_cached_area(node_ref, style_state);

        let rotate_effect = !transform_state.rotations.is_empty();
        let scales_effect = !transform_state.scales.is_empty();

        element_check || rotate_effect || scales_effect
    }

    /// Some elements such as `rect` might always need to rerender as Skia doesnt work well with clipped canvases with applied blur.
    pub fn needs_explicit_render(
        &self,
        node_transform: &TransformState,
        node_style: &StyleState,
    ) -> bool {
        match self {
            Self::Rect(el) => el.has_blur(node_transform, node_style),
            _ => false,
        }
    }
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
        fallback_fonts: &[String],
        images_cache: &mut ImagesCache,
        scale_factor: f32,
    ) {
        match self {
            Self::Rect(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                fallback_fonts,
                images_cache,
                scale_factor,
            ),
            Self::Svg(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                fallback_fonts,
                images_cache,
                scale_factor,
            ),
            Self::Paragraph(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                fallback_fonts,
                images_cache,
                scale_factor,
            ),
            Self::Image(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                fallback_fonts,
                images_cache,
                scale_factor,
            ),
            Self::Label(el) => el.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                fallback_fonts,
                images_cache,
                scale_factor,
            ),
        }
    }

    fn element_drawing_area(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        scale_factor: f32,
        node_style: &StyleState,
    ) -> Area {
        match self {
            Self::Rect(el) => {
                el.element_drawing_area(layout_node, node_ref, scale_factor, node_style)
            }
            Self::Svg(el) => {
                el.element_drawing_area(layout_node, node_ref, scale_factor, node_style)
            }
            Self::Paragraph(el) => {
                el.element_drawing_area(layout_node, node_ref, scale_factor, node_style)
            }
            Self::Image(el) => {
                el.element_drawing_area(layout_node, node_ref, scale_factor, node_style)
            }
            Self::Label(el) => {
                el.element_drawing_area(layout_node, node_ref, scale_factor, node_style)
            }
        }
    }

    fn element_needs_cached_area(&self, node_ref: &DioxusNode, style_state: &StyleState) -> bool {
        match self {
            Self::Rect(el) => el.element_needs_cached_area(node_ref, style_state),
            Self::Svg(el) => el.element_needs_cached_area(node_ref, style_state),
            Self::Paragraph(el) => el.element_needs_cached_area(node_ref, style_state),
            Self::Image(el) => el.element_needs_cached_area(node_ref, style_state),
            Self::Label(el) => el.element_needs_cached_area(node_ref, style_state),
        }
    }
}
