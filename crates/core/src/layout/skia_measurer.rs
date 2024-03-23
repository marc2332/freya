use std::{ops::Mul, sync::Arc};

use dioxus_native_core::{
    prelude::{ElementNode, NodeType, SendAnyMap},
    real_dom::NodeImmutable,
    tags::TagName,
    NodeId,
};
use freya_common::CursorLayoutResponse;
use freya_dom::prelude::{DioxusDOM, DioxusNode};
use freya_node_state::{CursorReference, CursorSettings, FontStyleState, References, TextOverflow};

use freya_engine::prelude::*;
use torin::{
    geometry::CursorPoint,
    prelude::{LayoutMeasurer, LayoutNode, Node, Size2D},
};

pub struct CachedParagraph(pub Paragraph);

/// # Safety
/// Skia `Paragraph` are neither Sync or Send, but in order to store them in the Associated
/// data of the Nodes in Torin (which will be used across threads when making the attributes diffing),
/// we must manually mark the Paragraph as Send and Sync, this is fine because `Paragraph`s will only be accessed and modified
/// In the main thread when measuring the layout and painting.
unsafe impl Send for CachedParagraph {}
unsafe impl Sync for CachedParagraph {}

/// Provides Text measurements using Skia APIs like SkParagraph
pub struct SkiaMeasurer<'a> {
    pub font_collection: &'a FontCollection,
    pub rdom: &'a DioxusDOM,
    pub default_fonts: &'a [String],
}

impl<'a> SkiaMeasurer<'a> {
    pub fn new(
        rdom: &'a DioxusDOM,
        font_collection: &'a FontCollection,
        default_fonts: &'a [String],
    ) -> Self {
        Self {
            font_collection,
            rdom,
            default_fonts,
        }
    }
}

impl<'a> LayoutMeasurer<NodeId> for SkiaMeasurer<'a> {
    fn measure(
        &mut self,
        node_id: NodeId,
        _node: &Node,
        area_size: &Size2D,
    ) -> Option<(Size2D, Arc<SendAnyMap>)> {
        let node = self.rdom.get(node_id).unwrap();
        let node_type = node.node_type();

        match &*node_type {
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Label => {
                let label =
                    create_label(&node, area_size, self.font_collection, self.default_fonts);
                let res = Size2D::new(label.longest_line(), label.height());
                let mut map = SendAnyMap::new();
                map.insert(CachedParagraph(label));
                Some((res, Arc::new(map)))
            }
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Paragraph => {
                let paragraph = create_paragraph(
                    &node,
                    area_size,
                    self.font_collection,
                    false,
                    self.default_fonts,
                );
                let res = Size2D::new(paragraph.longest_line(), paragraph.height());
                let mut map = SendAnyMap::new();
                map.insert(CachedParagraph(paragraph));
                Some((res, Arc::new(map)))
            }
            _ => None,
        }
    }

    fn should_measure_inner_children(&mut self, node_id: NodeId) -> bool {
        let node = self.rdom.get(node_id).unwrap();
        let node_type: &NodeType<_> = &node.node_type();

        !matches!(node_type.tag(), Some(TagName::Label | TagName::Paragraph))
    }
}

pub fn create_label(
    node: &DioxusNode,
    area_size: &Size2D,
    font_collection: &FontCollection,
    default_font_family: &[String],
) -> Paragraph {
    let font_style = &*node.get::<FontStyleState>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.text_align);
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);
    let text_style = font_style.text_style(default_font_family);
    paragraph_style.set_text_style(&text_style);

    if let Some(ellipsis) = font_style.text_overflow.get_ellipsis() {
        paragraph_style.set_ellipsis(ellipsis);
    }

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    for child in node.children() {
        if let NodeType::Text(text) = &*child.node_type() {
            paragraph_builder.add_text(text);
        }
    }

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(area_size.width + 1.0);
    paragraph
}

/// Compose a new SkParagraph
pub fn create_paragraph(
    node: &DioxusNode,
    area_size: &Size2D,
    font_collection: &FontCollection,
    is_rendering: bool,
    default_font_family: &[String],
) -> Paragraph {
    let font_style = &*node.get::<FontStyleState>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.text_align);
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);

    if font_style.text_overflow == TextOverflow::Ellipsis {
        paragraph_style.set_ellipsis("…");
    }

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    let text_style = font_style.text_style(default_font_family);
    paragraph_builder.push_style(&text_style);

    for text_span in node.children() {
        match &*text_span.node_type() {
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Text => {
                let text_nodes = text_span.children();
                let text_node = *text_nodes.first().unwrap();
                let text_node_type = &*text_node.node_type();
                let font_style = text_span.get::<FontStyleState>().unwrap();
                let text_style = font_style.text_style(default_font_family);
                paragraph_builder.push_style(&text_style);

                if let NodeType::Text(text) = text_node_type {
                    paragraph_builder.add_text(text);
                }
            }
            _ => {}
        }
    }

    if is_rendering {
        // This is very tricky, but it works! It allows freya to render the cursor at the end of a line.
        paragraph_builder.add_text(" ");
    }

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(area_size.width + 1.0);
    paragraph
}

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
