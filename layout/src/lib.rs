mod layers;

use dioxus_native_core::{
    prelude::{ElementNode, NodeType, TextNode},
    real_dom::NodeImmutable,
    NodeId,
};
use freya_common::CursorLayoutResponse;
use freya_dom::prelude::{DioxusDOM, DioxusNode};
use freya_node_state::{CursorReference, CursorSettings, FontStyle, References};

pub use layers::*;
use skia_safe::textlayout::{
    FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle,
};
use torin::{
    geometry::{Area, CursorPoint},
    prelude::{LayoutMeasurer, Node, Size2D},
};

/// Provides Text measurements using Skia APIs like SkParagraph
pub struct SkiaMeasurer<'a> {
    pub font_collection: &'a FontCollection,
    pub rdom: &'a DioxusDOM,
}

impl<'a> SkiaMeasurer<'a> {
    pub fn new(rdom: &'a DioxusDOM, font_collection: &'a FontCollection) -> Self {
        Self {
            font_collection,
            rdom,
        }
    }
}

impl<'a> LayoutMeasurer<NodeId> for SkiaMeasurer<'a> {
    fn measure(
        &mut self,
        node_id: NodeId,
        _node: &Node,
        area: &Area,
        _parent_area: &Area,
        available_parent_area: &Area,
    ) -> Option<Area> {
        let node = self.rdom.get(node_id).unwrap();
        let node_type = node.node_type();

        match &*node_type {
            NodeType::Text(TextNode { text, .. }) => {
                let text_paragraph =
                    create_text(&node, available_parent_area, self.font_collection, text);

                Some(Area::new(
                    area.origin,
                    Size2D::new(text_paragraph.longest_line(), text_paragraph.height()),
                ))
            }
            NodeType::Element(ElementNode { tag, .. }) if tag == "paragraph" => {
                let paragraph =
                    create_paragraph(&node, available_parent_area, self.font_collection, false);

                Some(Area::new(
                    available_parent_area.origin,
                    Size2D::new(paragraph.longest_line(), paragraph.height()),
                ))
            }
            _ => None,
        }
    }
}

/// Collect all the texts and FontStyles from all the given Node's children
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

fn create_text(
    node: &DioxusNode,
    area: &Area,
    font_collection: &FontCollection,
    text: &str,
) -> Paragraph {
    let font_style = &*node.get::<FontStyle>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.align);
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    paragraph_builder.push_style(
        TextStyle::new()
            .set_font_style(font_style.into())
            .set_font_size(font_style.font_size)
            .set_font_families(&font_style.font_family),
    );

    paragraph_builder.add_text(text);

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(area.width());
    paragraph
}

/// Compose a new SkParagraph
pub fn create_paragraph(
    node: &DioxusNode,
    node_area: &Area,
    font_collection: &FontCollection,
    is_rendering: bool,
) -> Paragraph {
    let font_style = &*node.get::<FontStyle>().unwrap();
    let node_cursor_settings = &*node.get::<CursorSettings>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.align);
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    paragraph_builder.push_style(
        TextStyle::new()
            .set_font_style(font_style.into())
            .set_font_size(font_style.font_size)
            .set_font_families(&font_style.font_family),
    );

    let texts = get_inner_texts(node);

    for (font_style, text) in texts.into_iter() {
        paragraph_builder.push_style(
            TextStyle::new()
                .set_font_style(skia_safe::FontStyle::from(&font_style))
                .set_height_override(true)
                .set_height(font_style.line_height)
                .set_color(font_style.color)
                .set_font_size(font_style.font_size)
                .set_font_families(&font_style.font_family),
        );
        paragraph_builder.add_text(text);
    }

    if node_cursor_settings.position.is_some() && is_rendering {
        // This is very tricky, but it works! It allows freya to render the cursor at the end of a line.
        paragraph_builder.add_text(" ");
    }

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(node_area.width() + 1.0);
    paragraph
}

fn measure_paragraph(
    node: &DioxusNode,
    node_area: &Area,
    font_collection: &FontCollection,
    is_editable: bool,
) -> Paragraph {
    let paragraph = create_paragraph(node, node_area, font_collection, false);

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
