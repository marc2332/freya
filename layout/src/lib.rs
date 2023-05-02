mod layers;

use dioxus_native_core::{
    prelude::{ElementNode, NodeType, TextNode},
    real_dom::{NodeImmutable, RealDom},
};
use freya_common::CursorLayoutResponse;
use freya_dom::DioxusNode;
use freya_node_state::{
    CursorReference, CursorSettings, CustomAttributeValues, FontStyle, References,
};

pub use layers::*;
use skia_safe::textlayout::{
    FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle,
};
use torin::{Area, CursorPoint};
pub type DioxusDOM = RealDom<CustomAttributeValues>;

#[allow(dead_code)]
fn process_paragraph(
    node: &DioxusNode,
    node_area: &Area,
    font_collection: &FontCollection,
    is_editable: bool,
) -> Paragraph {
    let font_style = node.get::<FontStyle>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.align);
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    paragraph_builder.push_style(
        TextStyle::new()
            .set_font_style(font_style.font_style)
            .set_font_size(font_style.font_size)
            .set_font_families(&font_style.font_family),
    );

    let texts = get_inner_texts(node);

    for (font_style, text) in texts.into_iter() {
        paragraph_builder.push_style(
            TextStyle::new()
                .set_font_style(font_style.font_style)
                .set_height_override(true)
                .set_height(font_style.line_height)
                .set_color(font_style.color)
                .set_font_size(font_style.font_size)
                .set_font_families(&font_style.font_family),
        );
        paragraph_builder.add_text(text);
    }

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(node_area.width() + 1.0);
    if is_editable {
        if let Some((cursor_ref, id, cursor_position, cursor_selections)) =
            get_cursor_reference(node)
        {
            if let Some(cursor_position) = cursor_position {
                // Calculate the new cursor position
                let char_position =
                    paragraph.get_glyph_position_at_coordinate(cursor_position.to_i32().to_tuple());

                // Notify the cursor reference listener
                cursor_ref
                    .agent
                    .send(CursorLayoutResponse::CursorPosition {
                        position: char_position.position as usize,
                        id,
                    })
                    .ok();
            }

            if let Some((origin, dist)) = cursor_selections {
                // Calculate the start of the highlighting
                let origin_char =
                    paragraph.get_glyph_position_at_coordinate(origin.to_i32().to_tuple());
                // Calculate the end of the highlighting
                let dist_char =
                    paragraph.get_glyph_position_at_coordinate(dist.to_i32().to_tuple());

                cursor_ref
                    .agent
                    .send(CursorLayoutResponse::TextSelection {
                        from: origin_char.position as usize,
                        to: dist_char.position as usize,
                        id,
                    })
                    .ok();
            }
        }
    }

    paragraph
}

/// Collect all the texts and node states from a given array of children
pub fn get_inner_texts(node: &DioxusNode) -> Vec<(FontStyle, String)> {
    node.children()
        .iter()
        .filter_map(|child| {
            if let NodeType::Element(ElementNode { tag, .. }) = &*child.node_type() {
                if tag != "text" {
                    return None;
                }

                let children = child.children();
                let child_text = *children.first().unwrap();
                let child_text_type = &*child_text.node_type();

                if let NodeType::Text(TextNode { text, .. }) = child_text_type {
                    let font_style = child.get::<FontStyle>().unwrap();
                    Some((font_style.clone(), text.to_owned()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
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
    let node_references = node.get::<References>().unwrap();
    let cursor_settings = node.get::<CursorSettings>().unwrap();

    let cursor_ref = node_references.cursor_ref.clone()?;
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
