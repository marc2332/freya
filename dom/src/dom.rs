use std::sync::{Arc, Mutex};

use dioxus_core::Mutations;
use dioxus_native_core::{
    prelude::{DioxusState, State},
    real_dom::{NodeRef, RealDom},
    NodeId, SendAnyMap,
};
use freya_node_state::{
    AccessibilityState, CursorSettings, CustomAttributeValues, FontStyle, LayoutState, References,
    Style, Transform,
};
use std::sync::MutexGuard;
use torin::prelude::*;
use tracing::info;

use crate::dom_adapter::DioxusDOMAdapter;

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
            LayoutState::to_type_erased(),
            Style::to_type_erased(),
            Transform::to_type_erased(),
            AccessibilityState::to_type_erased(),
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
    pub fn apply_mutations(&mut self, mutations: Mutations, scale_factor: f32) -> (bool, bool) {
        let dom_adapter = DioxusDOMAdapter::new(self.rdom());
        // Apply the mutations to the layout
        self.layout()
            .apply_mutations(&mutations, &self.dioxus_integration_state, &dom_adapter);

        // Apply the mutations the integration state
        self.dioxus_integration_state
            .apply_mutations(&mut self.rdom, mutations);

        // Update the Nodes states
        let mut ctx = SendAnyMap::new();
        ctx.insert(scale_factor);
        ctx.insert(self.torin.clone());

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
}
