use std::sync::Arc;

use freya_common::{
    CachedParagraph,
    NodeReferenceLayout,
};
use freya_engine::prelude::*;
use freya_native_core::{
    prelude::{
        ElementNode,
        NodeType,
        SendAnyMap,
    },
    real_dom::NodeImmutable,
    tags::TagName,
    NodeId,
};
use freya_node_state::{
    CursorState,
    FontStyleState,
    HighlightMode,
    LayoutState,
    TextOverflow,
};
use torin::prelude::{
    Alignment,
    Area,
    LayoutMeasurer,
    LayoutNode,
    Node,
    Point2D,
    Size2D,
};

use crate::dom::*;

/// Provides Text measurements using Skia APIs like SkParagraph
pub struct SkiaMeasurer<'a> {
    pub font_collection: &'a FontCollection,
    pub rdom: &'a DioxusDOM,
    pub default_fonts: &'a [String],
    pub scale_factor: f32,
}

impl<'a> SkiaMeasurer<'a> {
    pub fn new(
        rdom: &'a DioxusDOM,
        font_collection: &'a FontCollection,
        default_fonts: &'a [String],
        scale_factor: f32,
    ) -> Self {
        Self {
            font_collection,
            rdom,
            default_fonts,
            scale_factor,
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
                let label = create_label(
                    &node,
                    area_size,
                    self.font_collection,
                    self.default_fonts,
                    self.scale_factor,
                );
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
                    self.scale_factor,
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

        node_type
            .tag()
            .map(|tag| tag.has_children_with_intrinsic_layout())
            .unwrap_or_default()
    }

    fn notify_layout_references(&self, node_id: NodeId, layout_node: &LayoutNode) {
        let node = self.rdom.get(node_id).unwrap();
        let size_state = &*node.get::<LayoutState>().unwrap();

        if let Some(reference) = &size_state.node_ref {
            let mut node_layout = NodeReferenceLayout {
                area: layout_node.area,
                inner: layout_node.inner_sizes,
            };
            node_layout.div(self.scale_factor);
            reference.0.send(node_layout).ok();
        }
    }
}

pub fn create_label(
    node: &DioxusNode,
    area_size: &Size2D,
    font_collection: &FontCollection,
    default_font_family: &[String],
    scale_factor: f32,
) -> Paragraph {
    let font_style = &*node.get::<FontStyleState>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.text_align);
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);
    let text_style = font_style.text_style(default_font_family, scale_factor);
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

/// Align the Y axis of the highlights and cursor of a paragraph
pub fn align_highlights_and_cursor_paragraph(
    node: &DioxusNode,
    area: &Area,
    paragraph: &Paragraph,
    cursor_rect: &TextBox,
    width: Option<f32>,
) -> (Point2D, Point2D) {
    let cursor_state = node.get::<CursorState>().unwrap();

    let x = area.min_x() + cursor_rect.rect.left;
    let x2 = x + width.unwrap_or(cursor_rect.rect.right - cursor_rect.rect.left);

    match cursor_state.highlight_mode {
        HighlightMode::Fit => {
            let y = area.min_y()
                + align_main_align_paragraph(node, area, paragraph)
                + cursor_rect.rect.top;
            let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

            (Point2D::new(x, y), Point2D::new(x2, y2))
        }
        HighlightMode::Expanded => {
            let y = area.min_y();
            let y2 = area.max_y();

            (Point2D::new(x, y), Point2D::new(x2, y2))
        }
    }
}

/// Align the main alignment of a paragraph
pub fn align_main_align_paragraph(node: &DioxusNode, area: &Area, paragraph: &Paragraph) -> f32 {
    let layout = node.get::<LayoutState>().unwrap();

    match layout.main_alignment {
        Alignment::Start => 0.,
        Alignment::Center => (area.height() / 2.0) - (paragraph.height() / 2.0),
        Alignment::End => area.height() - paragraph.height(),
    }
}

/// Compose a new SkParagraph
pub fn create_paragraph(
    node: &DioxusNode,
    area_size: &Size2D,
    font_collection: &FontCollection,
    is_rendering: bool,
    default_font_family: &[String],
    scale_factor: f32,
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

    let text_style = font_style.text_style(default_font_family, scale_factor);
    paragraph_builder.push_style(&text_style);

    for text_span in node.children() {
        match &*text_span.node_type() {
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Text => {
                let text_nodes = text_span.children();
                let text_node = *text_nodes.first().unwrap();
                let text_node_type = &*text_node.node_type();
                let font_style = text_span.get::<FontStyleState>().unwrap();
                let text_style = font_style.text_style(default_font_family, scale_factor);
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
