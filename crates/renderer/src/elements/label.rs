use std::sync::Arc;

use dioxus_native_core::prelude::SendAnyMap;
use freya_core::layout::{create_label, SafeParagraph};
use freya_dom::prelude::DioxusNode;
use freya_engine::prelude::*;
use torin::geometry::Area;

/// Render a `label` element
pub fn render_label(
    area: &Area,
    data: &Option<Arc<SendAnyMap>>,
    node_ref: &DioxusNode,
    canvas: &Canvas,
    font_collection: &mut FontCollection,
) {
    let paragraph = &data.as_ref().unwrap().get::<SafeParagraph>().unwrap().0;

    let x = area.min_x();
    let y = area.min_y();

    paragraph.paint(canvas, (x, y));
}
