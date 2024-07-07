use freya_common::CachedParagraph;
use freya_engine::prelude::*;

use crate::prelude::{
    align_main_align_paragraph,
    DioxusNode,
    ElementRenderer,
};

pub struct LabelElement;

impl ElementRenderer for LabelElement {
    fn render(
        self: Box<Self>,
        layout_node: &torin::prelude::LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        _font_collection: &mut FontCollection,
        _font_manager: &FontMgr,
        _default_fonts: &[String],
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
}
