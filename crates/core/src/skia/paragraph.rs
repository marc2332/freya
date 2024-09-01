use std::ops::Mul;

use freya_common::{
    CachedParagraph,
    CursorLayoutResponse,
};
use freya_native_core::prelude::NodeImmutable;
use freya_node_state::CursorState;
use torin::prelude::{
    CursorPoint,
    LayoutNode,
};

use crate::prelude::{
    align_main_align_paragraph,
    DioxusNode,
    TextGroupMeasurement,
};

/// Merasure the cursor positio and text selection and notify the subscribed component of the element.
pub fn measure_paragraph(
    node: &DioxusNode,
    layout_node: &LayoutNode,
    text_measurement: &TextGroupMeasurement,
    scale_factor: f64,
) {
    let paragraph = &layout_node
        .data
        .as_ref()
        .unwrap()
        .get::<CachedParagraph>()
        .unwrap()
        .0;

    let cursor_state = node.get::<CursorState>().unwrap();

    if cursor_state.cursor_id != Some(text_measurement.cursor_id) {
        return;
    }

    let y = align_main_align_paragraph(node, &layout_node.area, paragraph);

    if let Some(cursor_reference) = &cursor_state.cursor_ref {
        if let Some(cursor_position) = text_measurement.cursor_position {
            let position = CursorPoint::new(cursor_position.x, cursor_position.y - y as f64);

            // Calculate the new cursor position
            let char_position = paragraph
                .get_glyph_position_at_coordinate(position.mul(scale_factor).to_i32().to_tuple());

            // Notify the cursor reference listener
            cursor_reference
                .cursor_sender
                .send(CursorLayoutResponse::CursorPosition {
                    position: char_position.position as usize,
                    id: text_measurement.cursor_id,
                })
                .ok();
        }

        if let Some((origin, dist)) = text_measurement.cursor_selection {
            let origin_position = CursorPoint::new(origin.x, origin.y - y as f64);
            let dist_position = CursorPoint::new(dist.x, dist.y - y as f64);

            // Calculate the start of the highlighting
            let origin_char = paragraph.get_glyph_position_at_coordinate(
                origin_position.mul(scale_factor).to_i32().to_tuple(),
            );
            // Calculate the end of the highlighting
            let dist_char = paragraph.get_glyph_position_at_coordinate(
                dist_position.mul(scale_factor).to_i32().to_tuple(),
            );

            cursor_reference
                .cursor_sender
                .send(CursorLayoutResponse::TextSelection {
                    from: origin_char.position as usize,
                    to: dist_char.position as usize,
                    id: text_measurement.cursor_id,
                })
                .ok();
        }
    }
}
