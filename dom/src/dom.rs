use std::sync::{Arc, Mutex};

use dioxus_core::{Mutation, Mutations};
use dioxus_native_core::{
    prelude::{DioxusState, State},
    real_dom::{NodeRef, RealDom},
    SendAnyMap,
};
use freya_common::LayoutNotifier;
use freya_node_state::{
    AccessibilitySettings, CursorSettings, CustomAttributeValues, FontStyle, References, Scroll,
    Size, Style, Transform,
};

#[cfg(feature = "shared")]
use std::sync::MutexGuard;

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
    layout_notifier: LayoutNotifier,
}

impl Default for FreyaDOM {
    fn default() -> Self {
        let mut rdom = RealDom::<CustomAttributeValues>::new([
            CursorSettings::to_type_erased(),
            FontStyle::to_type_erased(),
            References::to_type_erased(),
            Scroll::to_type_erased(),
            Size::to_type_erased(),
            Style::to_type_erased(),
            Transform::to_type_erased(),
            AccessibilitySettings::to_type_erased(),
        ]);
        let dioxus_integration_state = DioxusState::create(&mut rdom);
        Self {
            rdom,
            dioxus_integration_state,
            layout_notifier: Arc::new(Mutex::new(false)),
        }
    }
}

impl FreyaDOM {
    pub fn new(rdom: DioxusDOM, dioxus_integration_state: DioxusState) -> Self {
        Self {
            rdom,
            dioxus_integration_state,
            layout_notifier: Arc::new(Mutex::new(false)),
        }
    }

    /// Create the initial DOM from the given Mutations
    pub fn init_dom(&mut self, mutations: Mutations, scale_factor: f32) {
        self.dioxus_integration_state
            .apply_mutations(&mut self.rdom, mutations);

        *self.layout_notifier.lock().unwrap() = false;

        let mut ctx = SendAnyMap::new();
        ctx.insert(self.layout_notifier.clone());
        ctx.insert(scale_factor);

        self.rdom.update_state(ctx);
    }

    /// Process the given mutations from the [`VirtualDOM`](dioxus_core::VirtualDom).
    /// This will notify the layout if it must recalculate
    /// or the renderer if it has to repaint.
    pub fn apply_mutations(&mut self, mutations: Mutations, scale_factor: f32) -> (bool, bool) {
        *self.layout_notifier.lock().unwrap() = false;
        let mut layout_changes = false;

        // Notify the layout of any major layout change
        // TODO: Implement granual layout changes
        for mutation in &mutations.edits {
            match mutation {
                Mutation::AssignId { .. } => {}
                Mutation::SetAttribute { .. } => {}
                Mutation::NewEventListener { .. } => {}
                Mutation::RemoveEventListener { .. } => {}
                Mutation::InsertAfter { m, .. } => {
                    if *m > 0 {
                        layout_changes = true;
                    }
                }
                Mutation::InsertBefore { m, .. } => {
                    if *m > 0 {
                        layout_changes = true;
                    }
                }
                Mutation::ReplacePlaceholder { m, .. } => {
                    if *m > 0 {
                        layout_changes = true;
                    }
                }
                Mutation::ReplaceWith { m, .. } => {
                    if *m > 0 {
                        layout_changes = true;
                    }
                }
                Mutation::AppendChildren { m, .. } => {
                    if *m > 0 {
                        layout_changes = true;
                    }
                }
                _ => {
                    layout_changes = true;
                }
            }
        }

        // Apply the mutations to the RealDOM

        self.dioxus_integration_state
            .apply_mutations(&mut self.rdom, mutations);

        // Update the Nodes states
        let mut ctx = SendAnyMap::new();
        ctx.insert(self.layout_notifier.clone());
        ctx.insert(scale_factor);

        let (_, diff) = self.rdom.update_state(ctx);

        // Calculate whether it must repaint or relayout
        let paint_changes = !diff.is_empty();
        if *self.layout_notifier.lock().unwrap() {
            layout_changes = true;
        }

        (paint_changes, layout_changes)
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
