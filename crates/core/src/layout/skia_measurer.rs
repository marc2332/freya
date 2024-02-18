use std::ops::Mul;

use dioxus_native_core::{
    prelude::{ElementNode, NodeType, TextNode},
    real_dom::NodeImmutable,
    NodeId,
};
use freya_common::CursorLayoutResponse;
use freya_dom::prelude::{DioxusDOM, DioxusNode};
use freya_node_state::{CursorReference, CursorSettings, FontStyleState, References, TextOverflow};

use freya_engine::prelude::*;
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
        _parent_area: &Area,
        available_parent_area: &Area,
    ) -> Option<Size2D> {
        let node = self.rdom.get(node_id).unwrap();
        let node_type = node.node_type();

        match &*node_type {
            NodeType::Element(ElementNode { tag, .. }) if tag == "label" => {
                let label = create_label(&node, available_parent_area, self.font_collection);

                Some(Size2D::new(label.longest_line(), label.height()))
            }
            NodeType::Element(ElementNode { tag, .. }) if tag == "paragraph" => {
                let paragraph =
                    create_paragraph(&node, available_parent_area, self.font_collection, false);

                Some(Size2D::new(paragraph.longest_line(), paragraph.height()))
            }
            _ => None,
        }
    }
}

pub fn create_label(node: &DioxusNode, area: &Area, font_collection: &FontCollection) -> Paragraph {
    let font_style = &*node.get::<FontStyleState>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.text_align);
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);
    paragraph_style.set_text_style(&font_style.into());

    if let Some(ellipsis) = font_style.text_overflow.get_ellipsis() {
        paragraph_style.set_ellipsis(ellipsis);
    }

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    for child in node.children() {
        if let NodeType::Text(TextNode { text, .. }) = &*child.node_type() {
            paragraph_builder.add_text(text);
        }
    }

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(area.width() + 1.0);
    paragraph
}

/// Compose a new SkParagraph
pub fn create_paragraph(
    node: &DioxusNode,
    node_area: &Area,
    font_collection: &FontCollection,
    is_rendering: bool,
) -> Paragraph {
    let font_style = &*node.get::<FontStyleState>().unwrap();
    let node_cursor_settings = &*node.get::<CursorSettings>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.text_align);
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);

    if font_style.text_overflow == TextOverflow::Ellipsis {
        paragraph_style.set_ellipsis("…");
    }

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    paragraph_builder.push_style(&font_style.into());

    for text_span in node.children() {
        match &*text_span.node_type() {
            NodeType::Element(ElementNode { tag, .. }) if tag == "text" => {
                let text_nodes = text_span.children();
                let text_node = *text_nodes.first().unwrap();
                let text_node_type = &*text_node.node_type();

                if let NodeType::Text(TextNode { text, .. }) = text_node_type {
                    let font_style = text_node.get::<FontStyleState>().unwrap();
                    paragraph_builder.push_style(&TextStyle::from(&*font_style));
                    paragraph_builder.add_text(text);
                }
            }
            _ => {}
        }
    }

    if node_cursor_settings.position.is_some() && is_rendering {
        // This is very tricky, but it works! It allows freya to render the cursor at the end of a line.
        paragraph_builder.add_text(" ");
    }

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(node_area.width() + 1.0);
    paragraph
}

pub fn measure_paragraph(
    node: &DioxusNode,
    node_area: &Area,
    font_collection: &FontCollection,
    is_editable: bool,
    scale_factor: f32,
) -> Paragraph {
    let paragraph = create_paragraph(node, node_area, font_collection, false);
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
