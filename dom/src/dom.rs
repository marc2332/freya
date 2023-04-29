use std::sync::{Arc, Mutex};

use dioxus_core::{Mutation, Mutations};
use dioxus_native_core::{
    prelude::{DioxusState, State},
    real_dom::{NodeRef, RealDom},
    NodeId, SendAnyMap,
};
use freya_node_state::{
    CursorSettings, CustomAttributeValues, FontStyle, References, Size, Style, Transform,
};
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
    torin: Arc<Mutex<Torin<NodeId, EmbeddedData>>>,
}

impl Default for FreyaDOM {
    fn default() -> Self {
        let mut rdom = RealDom::<CustomAttributeValues>::new([
            CursorSettings::to_type_erased(),
            FontStyle::to_type_erased(),
            References::to_type_erased(),
            Size::to_type_erased(),
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

    pub fn layout(&self) -> MutexGuard<torin::Torin<NodeId, EmbeddedData>> {
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
    /// This will notify the layout if it must recalculate
    /// or the renderer if it has to repaint.
    pub fn apply_mutations(&mut self, mutations: Mutations, scale_factor: f32) -> bool {
        for mutation in &mutations.edits {
            match mutation {
                Mutation::Remove { id } => {
                    self.torin
                        .lock()
                        .unwrap()
                        .remove(self.dioxus_integration_state.element_to_node_id(*id));
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

        let paint_changes = !diff.is_empty();

        paint_changes
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

pub struct SkiaTextMeasurer;

impl LayoutMeasurer<NodeId, EmbeddedData> for SkiaTextMeasurer {
    fn measure(
        &mut self,
        _node: &NodeData<NodeId, EmbeddedData>,
        _area: &Rect<f32, Measure>,
        _parent_size: &Rect<f32, Measure>,
        _available_parent_size: &Rect<f32, Measure>,
    ) -> Option<Rect<f32, Measure>> {
        None
    }
}
