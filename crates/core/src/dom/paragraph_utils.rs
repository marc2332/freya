use freya_native_core::real_dom::NodeImmutable;

use freya_common::{CachedParagraph, CursorLayoutResponse, TextGroupMeasurement};
use freya_node_state::CursorSettings;
use std::ops::Mul;

use torin::prelude::*;

use crate::dom::DioxusNode;

pub fn measure_paragraph(
    node: &DioxusNode,
    layout_node: &LayoutNode,
    text_measurement: &TextGroupMeasurement,
    scale_factor: f32,
) {
    let paragraph = &layout_node
        .data
        .as_ref()
        .unwrap()
        .get::<CachedParagraph>()
        .unwrap()
        .0;

    let cursor_settings = node.get::<CursorSettings>().unwrap();

    let scale_factors = scale_factor as f64;

    if cursor_settings.cursor_id != Some(text_measurement.cursor_id) {
        return;
    }

    if let Some(cursor_reference) = &cursor_settings.cursor_ref {
        if let Some(cursor_position) = text_measurement.cursor_position {
            // Calculate the new cursor position
            let char_position = paragraph.get_glyph_position_at_coordinate(
                cursor_position.mul(scale_factors).to_i32().to_tuple(),
            );

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
            // Calculate the start of the highlighting
            let origin_char = paragraph
                .get_glyph_position_at_coordinate(origin.mul(scale_factors).to_i32().to_tuple());
            // Calculate the end of the highlighting
            let dist_char = paragraph
                .get_glyph_position_at_coordinate(dist.mul(scale_factors).to_i32().to_tuple());

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
