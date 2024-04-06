use std::sync::Arc;

use dioxus_native_core::prelude::SendAnyMap;
use freya_common::CachedParagraph;
use freya_engine::prelude::*;
use torin::geometry::Area;

/// Render a `label` element
pub fn render_label(area: &Area, data: &Option<Arc<SendAnyMap>>, canvas: &Canvas) {
    let paragraph = &data.as_ref().unwrap().get::<CachedParagraph>().unwrap().0;

    let x = area.min_x();
    let y = area.min_y();

    paragraph.paint(canvas, (x, y));
}
