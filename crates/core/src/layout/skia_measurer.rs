use std::sync::Arc;

use dioxus_native_core::{
    prelude::{ElementNode, NodeType, SendAnyMap},
    real_dom::NodeImmutable,
    tags::TagName,
    NodeId,
};
use freya_common::{CachedParagraph, NodeReferenceLayout};
use freya_dom::prelude::{DioxusDOM, DioxusNode};
use freya_node_state::{FontStyleState, LayoutState, TextOverflow};

use freya_engine::prelude::*;
use torin::prelude::{LayoutMeasurer, LayoutNode, Node, Size2D};
/// Provides Text measurements using Skia APIs like SkParagraph
pub struct SkiaMeasurer<'a> {
    pub font_collection: &'a FontCollection,
    pub rdom: &'a DioxusDOM,
    pub scale_factor: f32,
}

impl<'a> SkiaMeasurer<'a> {
    pub fn new(
        rdom: &'a DioxusDOM,
        font_collection: &'a FontCollection,
        scale_factor: f32,
    ) -> Self {
        Self {
            font_collection,
            rdom,
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
        mut old_layout_node: Option<LayoutNode>,
    ) -> Option<(Size2D, Arc<SendAnyMap>)> {
        let node = self.rdom.get(node_id).unwrap();
        let node_type = node.node_type();

        match &*node_type {
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Label => {
                let font_style = &*node.get::<FontStyleState>().unwrap();
                let results = if let Some(old_layout_node) = old_layout_node.as_mut() {
                    let map = old_layout_node.data.take().unwrap();

                    let old_font_style = map.get::<FontStyleState>().unwrap();
                    let old_area_size = map.get::<Size2D>().unwrap();

                    if old_font_style == font_style && area_size == old_area_size {
                        let label = &map.get::<CachedParagraph>().unwrap().0;
                        let size = Size2D::new(label.longest_line(), label.height());
                        Some((map, size))
                    } else {
                        None
                    }
                } else {
                    None
                };

                let (map, res) = results.unwrap_or_else(|| {
                    let mut map = SendAnyMap::new();
                    let label = create_label(&node, area_size, self.font_collection);
                    map.insert(CachedParagraph(label));
                    map.insert(font_style.clone());
                    map.insert(*area_size);
                    let map = Arc::new(map);
                    let label = &map.get::<CachedParagraph>().unwrap().0;
                    let res = Size2D::new(label.longest_line(), label.height());
                    (map, res)
                });

                Some((res, map))
            }
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Paragraph => {
                let mut map = SendAnyMap::new();
                let paragraph = create_paragraph(&node, area_size, self.font_collection, false);
                let res = Size2D::new(paragraph.longest_line(), paragraph.height());
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
) -> Paragraph {
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

    paragraph_builder.push_style(&font_style.into());

    for text_span in node.children() {
        match &*text_span.node_type() {
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Text => {
                let text_nodes = text_span.children();
                let text_node = *text_nodes.first().unwrap();
                let text_node_type = &*text_node.node_type();
                let font_style = text_span.get::<FontStyleState>().unwrap();
                paragraph_builder.push_style(&TextStyle::from(&*font_style));

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
