use std::sync::Arc;

use freya_engine::prelude::*;
use freya_native_core::{
    prelude::{
        ElementNode,
        NodeType,
    },
    real_dom::NodeImmutable,
    tags::TagName,
    NodeId,
};
use torin::prelude::{
    Area,
    LayoutMeasurer,
    Node,
    SendAnyMap,
    Size2D,
};

use super::{
    create_label,
    create_paragraph,
    get_or_create_image,
    ImageData,
};
use crate::{
    custom_attributes::NodeReferenceLayout,
    dom::*,
    elements::CachedParagraph,
    render::ParagraphData,
    states::LayoutState,
};

/// Provides Text measurements using Skia APIs like SkParagraph
pub struct SkiaMeasurer<'a> {
    pub font_collection: &'a FontCollection,
    pub rdom: &'a DioxusDOM,
    pub fallback_fonts: &'a [String],
    pub scale_factor: f32,
    pub images_cache: &'a mut ImagesCache,
}

impl<'a> SkiaMeasurer<'a> {
    pub fn new(
        rdom: &'a DioxusDOM,
        font_collection: &'a FontCollection,
        fallback_fonts: &'a [String],
        scale_factor: f32,
        images_cache: &'a mut ImagesCache,
    ) -> Self {
        Self {
            font_collection,
            rdom,
            fallback_fonts,
            scale_factor,
            images_cache,
        }
    }
}

impl LayoutMeasurer<NodeId> for SkiaMeasurer<'_> {
    fn measure(
        &mut self,
        node_id: NodeId,
        torin_node: &Node,
        area_size: &Size2D,
    ) -> Option<(Size2D, Arc<SendAnyMap>)> {
        let node = self.rdom.get(node_id).unwrap();
        let node_type = node.node_type();

        match &*node_type {
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Label => {
                let ParagraphData { paragraph, size } = create_label(
                    &node,
                    torin_node,
                    area_size,
                    self.font_collection,
                    self.fallback_fonts,
                    self.scale_factor,
                );
                let mut map = SendAnyMap::new();
                map.insert(CachedParagraph(paragraph));
                Some((size, Arc::new(map)))
            }
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Paragraph => {
                let ParagraphData { paragraph, size } = create_paragraph(
                    &node,
                    area_size,
                    self.font_collection,
                    false,
                    self.fallback_fonts,
                    self.scale_factor,
                );
                let mut map = SendAnyMap::new();
                map.insert(CachedParagraph(paragraph));
                Some((size, Arc::new(map)))
            }
            NodeType::Element(ElementNode { tag, .. }) if tag == &TagName::Image => {
                let Some(ImageData { size, .. }) =
                    get_or_create_image(&node, area_size, self.images_cache)
                else {
                    return Some((*area_size, Arc::default()));
                };
                Some((size, Arc::default()))
            }
            _ => None,
        }
    }

    fn should_measure(&mut self, node_id: NodeId) -> bool {
        let node = self.rdom.get(node_id).unwrap();
        let node_type: &NodeType<_> = &node.node_type();

        node_type
            .tag()
            .map(|tag| [TagName::Image, TagName::Label, TagName::Paragraph].contains(tag))
            .unwrap_or_default()
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
