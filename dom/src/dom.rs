use std::sync::{Arc, Mutex};

use dioxus_core::{Mutation, Mutations};
use dioxus_native_core::{
    prelude::{DioxusState, ElementNode, NodeType, State, TextNode},
    real_dom::{NodeImmutable, NodeRef, RealDom},
    tree::TreeRef,
    NodeId, SendAnyMap,
};
use freya_node_state::{
    CursorSettings, CustomAttributeValues, FontStyle, References, SizeState, Style, Transform,
};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use std::sync::MutexGuard;
use torin::*;

pub type DioxusDOM = RealDom<CustomAttributeValues>;
pub type DioxusNode<'a> = NodeRef<'a, CustomAttributeValues>;

/// Tiny wrapper over [FreyaDOM] to make it thread-safe if desired.
/// This is primarily used by the Devtools and Testing renderer.
pub struct SafeDOM {
    #[cfg(not(feature = "shared"))]
    pub dom: FreyaDOM,

    #[cfg(feature = "shared")]
    pub dom: Arc<Mutex<FreyaDOM>>,
}

#[cfg(feature = "shared")]
impl Clone for SafeDOM {
    fn clone(&self) -> Self {
        Self {
            dom: self.dom.clone(),
        }
    }
}

impl SafeDOM {
    #[cfg(not(feature = "shared"))]
    pub fn new(dom: FreyaDOM) -> Self {
        Self { dom }
    }

    #[cfg(feature = "shared")]
    pub fn new(dom: FreyaDOM) -> Self {
        Self {
            dom: Arc::new(Mutex::new(dom)),
        }
    }

    /// Get a reference to the DOM.
    #[cfg(not(feature = "shared"))]
    pub fn get(&self) -> &FreyaDOM {
        &self.dom
    }

    /// Get a mutable reference to the DOM.
    #[cfg(not(feature = "shared"))]
    pub fn get_mut(&mut self) -> &mut FreyaDOM {
        &mut self.dom
    }

    /// Get a reference to the DOM.
    #[cfg(feature = "shared")]
    pub fn get(&self) -> MutexGuard<FreyaDOM> {
        return self.dom.lock().unwrap();
    }

    /// Get a mutable reference to the dom.
    #[cfg(feature = "shared")]
    pub fn get_mut(&self) -> MutexGuard<FreyaDOM> {
        return self.dom.lock().unwrap();
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
        for mutation in &mutations.edits {
            #[allow(clippy::single_match)]
            match mutation {
                Mutation::SetText { id, .. } => {
                    self.torin
                        .lock()
                        .unwrap()
                        .invalidate(self.dioxus_integration_state.element_to_node_id(*id));
                }
                Mutation::Remove { id } => {
                    let node_resolver = DioxusNodeResolver::new(self.dom());
                    self.torin.lock().unwrap().remove(
                        self.dioxus_integration_state.element_to_node_id(*id),
                        &node_resolver,
                    );
                }
                _ => {}
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
    pub fn dom(&self) -> &DioxusDOM {
        &self.rdom
    }

    /// Get a mutable reference to the [`DioxusDOM`].
    pub fn dom_mut(&mut self) -> &mut DioxusDOM {
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
    fn height(&self, node_id: &NodeId) -> u16 {
        self.rdom.tree_ref().height(*node_id).unwrap()
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
        _node: &NodeData,
        area: &Rect<f32, Measure>,
        _parent_size: &Rect<f32, Measure>,
        available_parent_size: &Rect<f32, Measure>,
    ) -> Option<Rect<f32, Measure>> {
        let node = self.rdom.get(node_id).unwrap();
        let node_type = node.node_type();

        if let NodeType::Text(TextNode { text, .. }) = &*node_type {
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
            paragraph.layout(available_parent_size.width());

            Some(Area::new(
                area.origin,
                Size2D::new(paragraph.longest_line(), paragraph.height()),
            ))
        } else {
            None
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
