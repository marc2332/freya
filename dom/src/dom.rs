use std::sync::{Arc, Mutex};

use dioxus_core::{Mutation, Mutations};
use dioxus_native_core::{
    prelude::{DioxusState, ElementNode, NodeType, State, TextNode},
    real_dom::{NodeImmutable, NodeRef, RealDom},
    tree::TreeRef,
    NodeId, SendAnyMap,
};
use freya_node_state::{
    CursorMode, CursorSettings, CustomAttributeValues, FontStyle, References, SizeState, Style,
    Transform,
};
use rustc_hash::{FxHashMap, FxHashSet};
use skia_safe::textlayout::{
    FontCollection, ParagraphBuilder, ParagraphStyle, TextHeightBehavior, TextStyle,
};
use std::sync::MutexGuard;
use torin::*;
use uuid::Uuid;

pub type DioxusDOM = RealDom<CustomAttributeValues>;
pub type DioxusNode<'a> = NodeRef<'a, CustomAttributeValues>;

/// Tiny wrapper over [FreyaDOM] to make it thread-safe if desired.
/// This is primarily used by the Devtools and Testing renderer.
pub struct SafeDOM {
    #[cfg(not(feature = "shared"))]
    pub fdom: FreyaDOM,

    #[cfg(feature = "shared")]
    pub fdom: Arc<Mutex<FreyaDOM>>,
}

#[cfg(feature = "shared")]
impl Clone for SafeDOM {
    fn clone(&self) -> Self {
        Self {
            fdom: self.fdom.clone(),
        }
    }
}

impl SafeDOM {
    #[cfg(not(feature = "shared"))]
    pub fn new(fdom: FreyaDOM) -> Self {
        Self { fdom }
    }

    #[cfg(feature = "shared")]
    pub fn new(fdom: FreyaDOM) -> Self {
        Self {
            fdom: Arc::new(Mutex::new(fdom)),
        }
    }

    /// Get a reference to the DOM.
    #[cfg(not(feature = "shared"))]
    pub fn get(&self) -> &FreyaDOM {
        &self.fdom
    }

    /// Get a mutable reference to the DOM.
    #[cfg(not(feature = "shared"))]
    pub fn get_mut(&mut self) -> &mut FreyaDOM {
        &mut self.fdom
    }

    /// Get a reference to the DOM.
    #[cfg(feature = "shared")]
    pub fn get(&self) -> MutexGuard<FreyaDOM> {
        return self.fdom.lock().unwrap();
    }

    /// Get a mutable reference to the dom.
    #[cfg(feature = "shared")]
    pub fn get_mut(&self) -> MutexGuard<FreyaDOM> {
        return self.fdom.lock().unwrap();
    }
}

/// Manages the application DOM.
pub struct FreyaDOM {
    rdom: DioxusDOM,
    dioxus_integration_state: DioxusState,
    torin: Arc<Mutex<Torin<NodeId>>>,
}

impl Default for FreyaDOM {
    fn default() -> Self {
        let mut rdom = RealDom::<CustomAttributeValues>::new([
            CursorSettings::to_type_erased(),
            FontStyle::to_type_erased(),
            References::to_type_erased(),
            SizeState::to_type_erased(),
            Style::to_type_erased(),
            Transform::to_type_erased(),
        ]);
        let dioxus_integration_state = DioxusState::create(&mut rdom);
        Self {
            rdom,
            dioxus_integration_state,
            torin: Arc::new(Mutex::new(Torin::new())),
        }
    }
}

impl FreyaDOM {
    pub fn new(rdom: DioxusDOM, dioxus_integration_state: DioxusState) -> Self {
        Self {
            rdom,
            dioxus_integration_state,
            torin: Arc::new(Mutex::new(Torin::new())),
        }
    }

    pub fn layout(&self) -> MutexGuard<Torin<NodeId>> {
        self.torin.lock().unwrap()
    }

    /// Create the initial DOM from the given Mutations
    pub fn init_dom(&mut self, mutations: Mutations, scale_factor: f32) {
        self.dioxus_integration_state
            .apply_mutations(&mut self.rdom, mutations);

        let mut ctx = SendAnyMap::new();
        ctx.insert(scale_factor);
        ctx.insert(self.torin.clone());

        self.rdom.update_state(ctx);
    }

    /// Process the given mutations from the [`VirtualDOM`](dioxus_core::VirtualDom).
    pub fn apply_mutations(&mut self, mutations: Mutations, scale_factor: f32) -> bool {
        {
            for mutation in &mutations.edits {
                match mutation {
                    Mutation::SetText { id, .. } => {
                        self.torin
                            .lock()
                            .unwrap()
                            .invalidate(self.dioxus_integration_state.element_to_node_id(*id));
                    }
                    Mutation::InsertAfter { id, .. } => {
                        self.torin
                            .lock()
                            .unwrap()
                            .invalidate(self.dioxus_integration_state.element_to_node_id(*id));
                    }
                    Mutation::InsertBefore { id, .. } => {
                        self.torin
                            .lock()
                            .unwrap()
                            .invalidate(self.dioxus_integration_state.element_to_node_id(*id));
                    }
                    Mutation::Remove { id } => {
                        let node_resolver = DioxusNodeResolver::new(self.rdom());
                        self.torin.lock().unwrap().remove(
                            self.dioxus_integration_state.element_to_node_id(*id),
                            &node_resolver,
                            true,
                        );
                    }
                    _ => {}
                }
            }
        }

        // Apply the mutations to the RealDOM

        if !mutations.edits.is_empty() {
            self.dioxus_integration_state
                .apply_mutations(&mut self.rdom, mutations);
        }

        // Update the Nodes states
        let mut ctx = SendAnyMap::new();
        ctx.insert(scale_factor);
        ctx.insert(self.torin.clone());

        let (_, diff) = self.rdom.update_state(ctx);

        !diff.is_empty()
    }

    /// Get a reference to the [`DioxusDOM`].
    pub fn rdom(&self) -> &DioxusDOM {
        &self.rdom
    }

    /// Get a mutable reference to the [`DioxusDOM`].
    pub fn rdom_mut(&mut self) -> &mut DioxusDOM {
        &mut self.rdom
    }
}

pub struct DioxusNodeResolver<'a> {
    pub rdom: &'a DioxusDOM,
}

impl<'a> DioxusNodeResolver<'a> {
    pub fn new(rdom: &'a DioxusDOM) -> Self {
        Self { rdom }
    }
}

impl NodeResolver<NodeId> for DioxusNodeResolver<'_> {
    fn height(&self, node_id: &NodeId) -> Option<u16> {
        self.rdom.tree_ref().height(*node_id)
    }

    fn parent_of(&self, node_id: &NodeId) -> Option<NodeId> {
        self.rdom.tree_ref().parent_id(*node_id)
    }

    fn children_of(&self, node_id: &NodeId) -> Vec<NodeId> {
        self.rdom.tree_ref().children_ids(*node_id)
    }
}

/// Provides Text measurements using Skia SkParagraph
pub struct SkiaMeasurer<'a> {
    pub font_collection: &'a FontCollection,
    pub rdom: &'a DioxusDOM,
    pub paragraph_elements: &'a mut FxHashMap<Uuid, FxHashSet<NodeId>>,
}

impl<'a> SkiaMeasurer<'a> {
    pub fn new(
        rdom: &'a DioxusDOM,
        font_collection: &'a FontCollection,
        paragraph_elements: &'a mut FxHashMap<Uuid, FxHashSet<NodeId>>,
    ) -> Self {
        Self {
            font_collection,
            rdom,
            paragraph_elements,
        }
    }
}

impl<'a> LayoutMeasurer<NodeId> for SkiaMeasurer<'a> {
    fn measure(
        &mut self,
        node_id: NodeId,
        _node: &NodeData,
        area: &Area,
        _parent_area: &Area,
        available_parent_area: &Area,
    ) -> Option<Area> {
        let node = self.rdom.get(node_id).unwrap();
        let node_type = node.node_type();

        match &*node_type {
            NodeType::Text(TextNode { text, .. }) => {
                let font_style = node.get::<FontStyle>().unwrap();

                let mut paragraph_style = ParagraphStyle::default();
                paragraph_style.set_text_align(font_style.align);
                paragraph_style.set_max_lines(font_style.max_lines);
                paragraph_style.set_replace_tab_characters(true);

                let mut paragraph_builder =
                    ParagraphBuilder::new(&paragraph_style, self.font_collection);

                paragraph_builder.push_style(
                    TextStyle::new()
                        .set_font_style(font_style.font_style)
                        .set_font_size(font_style.font_size)
                        .set_font_families(&font_style.font_family),
                );

                paragraph_builder.add_text(text);

                let mut paragraph = paragraph_builder.build();
                paragraph.layout(available_parent_area.width());

                Some(Area::new(
                    area.origin,
                    Size2D::new(paragraph.longest_line(), paragraph.height()),
                ))
            }
            NodeType::Element(ElementNode { tag, .. }) if tag == "paragraph" => {
                let font_style = node.get::<FontStyle>().unwrap();

                let mut paragraph_style = ParagraphStyle::default();
                paragraph_style.set_text_align(font_style.align);
                paragraph_style.set_max_lines(font_style.max_lines);
                paragraph_style.set_replace_tab_characters(true);
                paragraph_style.set_text_height_behavior(TextHeightBehavior::DisableAll);

                let mut paragraph_builder =
                    ParagraphBuilder::new(&paragraph_style, self.font_collection);

                paragraph_builder.push_style(
                    TextStyle::new()
                        .set_font_style(font_style.font_style)
                        .set_font_size(font_style.font_size)
                        .set_font_families(&font_style.font_family),
                );

                let texts = get_inner_texts(&node);

                for (font_style, text) in texts.into_iter() {
                    paragraph_builder.push_style(
                        TextStyle::new()
                            .set_font_style(font_style.font_style)
                            .set_height(font_style.line_height)
                            .set_color(font_style.color)
                            .set_font_size(font_style.font_size)
                            .set_font_families(&font_style.font_family),
                    );
                    paragraph_builder.add_text(text);
                }

                let mut paragraph = paragraph_builder.build();

                paragraph.layout(available_parent_area.width());

                let cursor_settings = node.get::<CursorSettings>().unwrap();
                let is_editable = CursorMode::Editable == cursor_settings.mode;

                let references = node.get::<References>().unwrap();
                if is_editable {
                    if let Some(cursor_ref) = &references.cursor_ref {
                        let text_group = self
                            .paragraph_elements
                            .entry(cursor_ref.text_id)
                            .or_insert_with(FxHashSet::default);

                        text_group.insert(node_id);
                    }
                }

                Some(Area::new(
                    available_parent_area.origin,
                    Size2D::new(paragraph.longest_line(), paragraph.height()),
                ))
            }
            _ => None,
        }
    }
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
