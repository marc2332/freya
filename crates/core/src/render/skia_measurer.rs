use std::sync::Arc;

use freya_common::NodeReferenceLayout;
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
use freya_node_state::LayoutState;
use torin::prelude::{
    Area,
    LayoutMeasurer,
    Node,
    Size2D,
};

use super::{
    create_label,
    create_paragraph,
    ParagraphCache,
};
use crate::dom::*;

/// Provides Text measurements using Skia APIs like SkParagraph
pub struct SkiaMeasurer<'a> {
    pub font_collection: &'a FontCollection,
    pub rdom: &'a DioxusDOM,
    pub default_fonts: &'a [String],
    pub scale_factor: f32,
    pub paragraph_cache: &'a mut ParagraphCache,
}

impl<'a> SkiaMeasurer<'a> {
    pub fn new(
        rdom: &'a DioxusDOM,
        font_collection: &'a FontCollection,
        default_fonts: &'a [String],
        scale_factor: f32,
        paragraph_cache: &'a mut ParagraphCache,
    ) -> Self {
        Self {
            font_collection,
            rdom,
            default_fonts,
            scale_factor,
            paragraph_cache,
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

        // println!("Measured in {}ms", a.elapsed().as_millis());

        match &*node_type {
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Label => {
                let label = create_label(
                    &node,
                    area_size,
                    self.font_collection,
                    self.default_fonts,
                    self.scale_factor,
                    self.paragraph_cache,
                );
                let height = label.0.borrow().height();
                let res = Size2D::new(label.0.borrow().longest_line(), height);
                let mut map = SendAnyMap::new();
                map.insert(label);
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
                    self.paragraph_cache,
                );
                let height = paragraph.0.borrow().height();
                let res = Size2D::new(paragraph.0.borrow().longest_line(), height);
                let mut map = SendAnyMap::new();
                map.insert(paragraph);
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

    fn notify_layout_references(&self, node_id: NodeId, area: Area, inner_sizes: Size2D) {
        let node = self.rdom.get(node_id).unwrap();
        let size_state = &*node.get::<LayoutState>().unwrap();

        if let Some(reference) = &size_state.node_ref {
            let mut node_layout = NodeReferenceLayout {
                area,
                inner: inner_sizes,
            };
            node_layout.div(self.scale_factor);
            reference.0.send(node_layout).ok();
        }
    }
}
