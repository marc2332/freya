use std::sync::{Arc, Mutex};

use dioxus_core::{Mutation, Mutations};
use dioxus_native_core::{node::Node, real_dom::RealDom, SendAnyMap};
use freya_common::LayoutNotifier;
use freya_node_state::{CustomAttributeValues, NodeState};

#[cfg(feature = "shared")]
use std::sync::MutexGuard;

pub type DioxusDOM = RealDom<NodeState, CustomAttributeValues>;
pub type DioxusNode = Node<NodeState, CustomAttributeValues>;

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

    #[cfg(not(feature = "shared"))]
    pub fn get(&self) -> &FreyaDOM {
        &self.dom
    }

    #[cfg(not(feature = "shared"))]
    pub fn get_mut(&mut self) -> &mut FreyaDOM {
        &mut self.dom
    }

    #[cfg(feature = "shared")]
    pub fn get(&self) -> MutexGuard<FreyaDOM> {
        return self.dom.lock().unwrap();
    }

    #[cfg(feature = "shared")]
    pub fn get_mut(&self) -> MutexGuard<FreyaDOM> {
        return self.dom.lock().unwrap();
    }
}

/// Manages the application DOM.
pub struct FreyaDOM {
    rdom: DioxusDOM,
    layout_notifier: LayoutNotifier,
}

impl FreyaDOM {
    pub fn new(rdom: DioxusDOM) -> Self {
        Self {
            rdom,
            layout_notifier: Arc::new(Mutex::new(false)),
        }
    }

    /// Create the initial DOM from the given Mutations
    pub fn init_dom(&mut self, mutations: Mutations) {
        let (to_update, _diff) = self.rdom.apply_mutations(mutations);

        *self.layout_notifier.lock().unwrap() = false;

        let mut ctx = SendAnyMap::new();
        ctx.insert(self.layout_notifier.clone());

        self.rdom.update_state(to_update, ctx);
    }

    /// Process the given mutations from the [VirtualDOM]
    /// This will notify the layout if it must recalculate
    /// or the renderer if it has to repaint.
    pub fn apply_mutations(&mut self, mutations: Mutations) -> (bool, bool) {
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
        let (to_update, diff) = self.rdom.apply_mutations(mutations);

        // Update the Nodes states
        let mut ctx = SendAnyMap::new();
        ctx.insert(self.layout_notifier.clone());
        self.rdom.update_state(to_update, ctx);

        // Calculate whether it must repaint or relayout
        let paint_changes = !diff.is_empty();
        if *self.layout_notifier.lock().unwrap() {
            layout_changes = true;
        }

        (paint_changes, layout_changes)
    }

    /// Get a reference to the [RealDOM].
    pub fn dom(&self) -> &DioxusDOM {
        &self.rdom
    }

    /// Get a mutable reference to the [RealDOM].
    pub fn dom_mut(&mut self) -> &mut DioxusDOM {
        &mut self.rdom
    }
}
