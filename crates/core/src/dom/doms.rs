use std::sync::{Arc, Mutex};

use dioxus_core::VirtualDom;
use freya_native_core::{
    prelude::{DioxusState, State},
    real_dom::{NodeRef, RealDom},
    NodeId, SendAnyMap,
};

use freya_common::{Layers, ParagraphElements};
use freya_node_state::{
    AccessibilityNodeState, CursorSettings, CustomAttributeValues, FontStyleState, LayerState,
    LayoutState, References, Style, Transform, ViewportState,
};
use std::sync::MutexGuard;
use torin::prelude::*;
use tracing::info;
use uuid::Uuid;

use super::{mutations_writer::MutationsWriter, paragraph_utils::measure_paragraph};

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

    /// Get a reference to the DOM.
    #[cfg(not(feature = "shared"))]
    pub fn try_get(&self) -> Option<&FreyaDOM> {
        Some(&self.fdom)
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

    /// Get a reference to the DOM.
    #[cfg(feature = "shared")]
    pub fn try_get(&self) -> Option<MutexGuard<FreyaDOM>> {
        return self.fdom.try_lock().ok();
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
    paragraphs: ParagraphElements,
    layers: Layers,
}

impl Default for FreyaDOM {
    fn default() -> Self {
        let mut rdom = RealDom::<CustomAttributeValues>::new([
            CursorSettings::to_type_erased(),
            FontStyleState::to_type_erased(),
            References::to_type_erased(),
            LayoutState::to_type_erased(),
            Style::to_type_erased(),
            Transform::to_type_erased(),
            AccessibilityNodeState::to_type_erased(),
            ViewportState::to_type_erased(),
            LayerState::to_type_erased(),
        ]);
        let dioxus_integration_state = DioxusState::create(&mut rdom);
        Self {
            rdom,
            dioxus_integration_state,
            torin: Arc::new(Mutex::new(Torin::new())),
            paragraphs: ParagraphElements::default(),
            layers: Layers::default(),
        }
    }
}

impl FreyaDOM {
    pub fn layout(&self) -> MutexGuard<Torin<NodeId>> {
        self.torin.lock().unwrap()
    }

    pub fn layers(&self) -> &Layers {
        &self.layers
    }

    pub fn paragraphs(&self) -> &ParagraphElements {
        &self.paragraphs
    }

    /// Create the initial DOM from the given Mutations
    pub fn init_dom(&mut self, vdom: &mut VirtualDom, scale_factor: f32) {
        // Build the RealDOM
        vdom.rebuild(&mut MutationsWriter {
            native_writer: self
                .dioxus_integration_state
                .create_mutation_writer(&mut self.rdom),
            layout: &mut self.torin.lock().unwrap(),
            layers: &self.layers,
            paragraphs: &self.paragraphs,
        });

        let mut ctx = SendAnyMap::new();
        ctx.insert(scale_factor);
        ctx.insert(self.torin.clone());
        ctx.insert(self.layers.clone());
        ctx.insert(self.paragraphs.clone());

        self.rdom.update_state(ctx);
    }

    /// Process the given mutations from the [`VirtualDOM`](dioxus_core::VirtualDom).
    pub fn render_mutations(&mut self, vdom: &mut VirtualDom, scale_factor: f32) -> (bool, bool) {
        // Update the RealDOM
        vdom.render_immediate(&mut MutationsWriter {
            native_writer: self
                .dioxus_integration_state
                .create_mutation_writer(&mut self.rdom),
            layout: &mut self.torin.lock().unwrap(),
            layers: &self.layers,
            paragraphs: &self.paragraphs,
        });

        // Update the Nodes states
        let mut ctx = SendAnyMap::new();
        ctx.insert(scale_factor);
        ctx.insert(self.torin.clone());
        ctx.insert(self.layers.clone());
        ctx.insert(self.paragraphs.clone());

        // Update the Node's states
        let (_, diff) = self.rdom.update_state(ctx);

        let must_repaint = !diff.is_empty();
        let must_relayout = !self.layout().get_dirty_nodes().is_empty();

        if !diff.is_empty() {
            info!(
                "Updated DOM, now with {} nodes",
                self.rdom().tree_ref().len()
            );
        }

        (must_repaint, must_relayout)
    }

    /// Get a reference to the [`DioxusDOM`].
    pub fn rdom(&self) -> &DioxusDOM {
        &self.rdom
    }

    /// Get a mutable reference to the [`DioxusDOM`].
    pub fn rdom_mut(&mut self) -> &mut DioxusDOM {
        &mut self.rdom
    }

    pub fn state_mut(&mut self) -> &mut DioxusState {
        &mut self.dioxus_integration_state
    }

    pub fn measure_all_paragraphs(&self, scale_factor: f32) {
        let layout = self.layout();
        let rdom = self.rdom();
        for group in self.paragraphs.paragraphs().values() {
            for node_id in group {
                let node = rdom.get(*node_id);
                let layout_node = layout.get(*node_id);
                if let Some((node, layout_node)) = node.zip(layout_node) {
                    measure_paragraph(&node, layout_node, true, scale_factor);
                }
            }
        }
    }

    /// Measure all the paragraphs registered under the given TextId
    pub fn measure_paragraphs(&self, text_id: &Uuid, scale_factor: f32) {
        let paragraphs = self.paragraphs.paragraphs();
        let group = paragraphs.get(text_id);
        let layout = self.layout();
        if let Some(group) = group {
            for node_id in group {
                let node = self.rdom().get(*node_id);
                let layout_node = layout.get(*node_id);

                if let Some((node, layout_node)) = node.zip(layout_node) {
                    measure_paragraph(&node, layout_node, true, scale_factor);
                }
            }
        }
    }
}
