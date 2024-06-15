use std::sync::Arc;

use freya_common::CachedParagraph;
use freya_engine::prelude::*;
use freya_native_core::prelude::SendAnyMap;
use torin::geometry::Area;

use crate::prelude::{
    align_main_align_paragraph,
    DioxusNode,
};

/// Render a `label` element
pub fn render_label(
    area: &Area,
    data: &Option<Arc<SendAnyMap>>,
    dioxus_node: &DioxusNode,
    canvas: &Canvas,
) {
    let paragraph = &data.as_ref().unwrap().get::<CachedParagraph>().unwrap().0;

    let x = area.min_x();
    let y = area.min_y() + align_main_align_paragraph(dioxus_node, area, paragraph);

    paragraph.paint(canvas, (x, y));
}
