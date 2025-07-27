use freya_engine::prelude::*;
use freya_native_core::prelude::NodeImmutable;
use torin::prelude::{
    Area,
    AreaModel,
    LayoutNode,
    Length,
    Size2D,
};

use super::utils::ElementUtils;
use crate::{
    dom::{
        DioxusNode,
        ImagesCache,
    },
    elements::paragraph::CachedParagraph,
    render::align_main_align_paragraph,
    states::{
        FontStyleState,
        StyleState,
    },
};

pub struct LabelElement;

impl ElementUtils for LabelElement {
    fn render(
        self,
        layout_node: &torin::prelude::LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        _font_collection: &mut FontCollection,
        _font_manager: &FontMgr,
        _fallback_fonts: &[String],
        _images_cache: &mut ImagesCache,
        _scale_factor: f32,
    ) {
        let paragraph = &layout_node
            .data
            .as_ref()
            .unwrap()
            .get::<CachedParagraph>()
            .unwrap()
            .0;
        let area = layout_node.visible_area();

        let x = area.min_x();
        let y = area.min_y() + align_main_align_paragraph(node_ref, &area, paragraph);

        paragraph.paint(canvas, (x, y));
    }

    fn clip(
        &self,
        layout_node: &LayoutNode,
        _node_ref: &DioxusNode,
        canvas: &Canvas,
        _scale_factor: f32,
    ) {
        canvas.clip_rect(
            Rect::new(
                layout_node.area.min_x(),
                layout_node.area.min_y(),
                layout_node.area.max_x(),
                layout_node.area.max_y(),
            ),
            ClipOp::Intersect,
            true,
        );
    }

    #[inline]
    fn element_needs_cached_area(&self, node_ref: &DioxusNode, _style_state: &StyleState) -> bool {
        let font_style = node_ref.get::<FontStyleState>().unwrap();

        !font_style.text_shadows.is_empty()
    }

    fn element_drawing_area(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        scale_factor: f32,
        _node_style: &StyleState,
    ) -> Area {
        let area = layout_node.visible_area();

        let font_style = node_ref.get::<FontStyleState>().unwrap();

        let mut text_shadow_area = area;

        for text_shadow in font_style.text_shadows.iter() {
            text_shadow_area.move_with_offsets(
                &Length::new(text_shadow.offset.0),
                &Length::new(text_shadow.offset.1),
            );

            let expanded_size = text_shadow.blur_sigma.ceil() as f32 * scale_factor;

            text_shadow_area.expand(&Size2D::new(expanded_size, expanded_size))
        }

        area.union(&text_shadow_area)
    }
}
