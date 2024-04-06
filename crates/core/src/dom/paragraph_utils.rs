use freya_native_core::real_dom::NodeImmutable;

use freya_common::{CachedParagraph, CursorLayoutResponse};
use freya_node_state::{CursorReference, CursorSettings};
use std::ops::Mul;

use torin::prelude::*;

use crate::dom::DioxusNode;

pub fn measure_paragraph(
    node: &DioxusNode,
    layout_node: &LayoutNode,
    is_editable: bool,
    scale_factor: f32,
) {
    let paragraph = &layout_node
        .data
        .as_ref()
        .unwrap()
        .get::<CachedParagraph>()
        .unwrap()
        .0;
    let scale_factors = scale_factor as f64;

    if is_editable {
        if let Some((cursor_ref, id, cursor_position, cursor_selections)) =
            get_cursor_reference(node)
        {
            if let Some(cursor_position) = cursor_position {
                // Calculate the new cursor position
                let char_position = paragraph.get_glyph_position_at_coordinate(
                    cursor_position.mul(scale_factors).to_i32().to_tuple(),
                );

                // Notify the cursor reference listener
                cursor_ref
                    .cursor_sender
                    .send(CursorLayoutResponse::CursorPosition {
                        position: char_position.position as usize,
                        id,
                    })
                    .ok();
            }

            if let Some((origin, dist)) = cursor_selections {
                // Calculate the start of the highlighting
                let origin_char = paragraph.get_glyph_position_at_coordinate(
                    origin.mul(scale_factors).to_i32().to_tuple(),
                );
                // Calculate the end of the highlighting
                let dist_char = paragraph
                    .get_glyph_position_at_coordinate(dist.mul(scale_factors).to_i32().to_tuple());

                cursor_ref
                    .cursor_sender
                    .send(CursorLayoutResponse::TextSelection {
                        from: origin_char.position as usize,
                        to: dist_char.position as usize,
                        id,
                    })
                    .ok();
            }
        }
    }
}

/// Get the info related to a cursor reference
#[allow(clippy::type_complexity)]
fn get_cursor_reference(
    node: &DioxusNode,
) -> Option<(
    CursorReference,
    usize,
    Option<CursorPoint>,
    Option<(CursorPoint, CursorPoint)>,
)> {
    let cursor_settings = node.get::<CursorSettings>().unwrap();

    let cursor_ref = cursor_settings.cursor_ref.clone()?;
    let cursor_id = cursor_settings.cursor_id?;

    let current_cursor_id = { *cursor_ref.cursor_id.lock().unwrap().as_ref()? };
    let cursor_selections = *cursor_ref.cursor_selections.lock().unwrap();
    let cursor_position = *cursor_ref.cursor_position.lock().unwrap();

    if current_cursor_id == cursor_id {
        Some((cursor_ref, cursor_id, cursor_position, cursor_selections))
    } else {
        None
    }
}
