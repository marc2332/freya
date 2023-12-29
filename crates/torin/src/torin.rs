use std::{collections::HashMap, mem};

pub use euclid::Rect;
use rustc_hash::{FxHashMap, FxHashSet};
use tracing::info;

use crate::{
    custom_measurer::LayoutMeasurer,
    dom_adapter::{DOMAdapter, NodeAreas, NodeKey},
    geometry::{Area, Size2D},
    measure::measure_node,
    prelude::Gaps,
};

pub struct LayoutMetadata {
    pub root_area: Area,
}

/// Contains the best Root node candidate from where to start measuring
#[derive(PartialEq, Debug, Clone)]
pub enum RootNodeCandidate<Key: NodeKey> {
    /// A valid Node ID
    Valid(Key),

    /// None
    None,
}

impl<Key: NodeKey> RootNodeCandidate<Key> {
    pub fn take(&mut self) -> Self {
        mem::replace(self, Self::None)
    }
}

pub struct Torin<Key: NodeKey> {
    /// Layout results of the registered Nodes
    pub results: FxHashMap<Key, NodeAreas>,

    /// Invalid registered nodes since previous layout measurement
    pub dirty: FxHashSet<Key>,

    /// Best Root node candidate from where to start measuring
    pub root_node_candidate: RootNodeCandidate<Key>,
}

impl<Key: NodeKey> Default for Torin<Key> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "dioxus")]
use dioxus_core::Mutations;
#[cfg(feature = "dioxus")]
use dioxus_native_core::prelude::*;

#[cfg(feature = "dioxus")]
impl Torin<NodeId> {
    pub fn apply_mutations(
        &mut self,
        mutations: &Mutations,
        dioxus_integration_state: &DioxusState,
        dom_adapter: &mut impl DOMAdapter<NodeId>,
    ) {
        use dioxus_core::Mutation;

        for mutation in &mutations.edits {
            match mutation {
                Mutation::SetText { id, .. } => {
                    self.invalidate(dioxus_integration_state.element_to_node_id(*id));
                }
                Mutation::InsertAfter { id, m } => {
                    if *m > 0 {
                        self.invalidate(dioxus_integration_state.element_to_node_id(*id));
                    }
                }
                Mutation::InsertBefore { id, m } => {
                    if *m > 0 {
                        self.invalidate(dioxus_integration_state.element_to_node_id(*id));
                    }
                }
                Mutation::Remove { id } => {
                    self.remove(
                        dioxus_integration_state.element_to_node_id(*id),
                        dom_adapter,
                        true,
                    );
                }
                Mutation::ReplaceWith { id, m } => {
                    if *m > 0 {
                        self.remove(
                            dioxus_integration_state.element_to_node_id(*id),
                            dom_adapter,
                            true,
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

impl<Key: NodeKey> Torin<Key> {
    /// Create a new Layout
    pub fn new() -> Self {
        Self {
            results: HashMap::default(),
            dirty: FxHashSet::default(),
            root_node_candidate: RootNodeCandidate::None,
        }
    }

    pub fn size(&self) -> usize {
        self.results.len()
    }

    /// Reset the layout
    pub fn reset(&mut self) {
        self.root_node_candidate = RootNodeCandidate::None;
        self.results.clear();
        self.dirty.clear();
    }

    /// Read the HashSet of dirty nodes
    pub fn get_dirty_nodes(&self) -> &FxHashSet<Key> {
        &self.dirty
    }

    /// Remove a Node's result and data
    pub fn raw_remove(&mut self, node_id: Key) {
        self.results.remove(&node_id);
        self.dirty.remove(&node_id);
        if let RootNodeCandidate::Valid(id) = self.root_node_candidate {
            if id == node_id {
                self.root_node_candidate = RootNodeCandidate::None
            }
        }
    }

    /// Remove a Node from the layout
    pub fn remove(
        &mut self,
        node_id: Key,
        dom_adapter: &mut impl DOMAdapter<Key>,
        invalidate_parent: bool,
    ) {
        // Remove itself
        self.raw_remove(node_id);

        // Mark as dirty the Node's parent
        if invalidate_parent {
            self.invalidate(dom_adapter.parent_of(&node_id).unwrap());
        }

        // Remove all it's children
        for child_id in dom_adapter.children_of(&node_id) {
            self.remove(child_id, dom_adapter, false);
        }
    }

    /// Mark as dirty a Node
    pub fn invalidate(&mut self, node_id: Key) {
        self.dirty.insert(node_id);
    }

    pub fn safe_invalidate(&mut self, node_id: Key, dom_adapter: &mut impl DOMAdapter<Key>) {
        if dom_adapter.is_node_valid(&node_id) {
            self.invalidate(node_id)
        }
    }

    // Mark as dirty the given Node and all the nodes that depend on it
    pub fn check_dirty_dependants(
        &mut self,
        node_id: Key,
        dom_adapter: &mut impl DOMAdapter<Key>,
        ignore: bool,
    ) {
        if (self.dirty.contains(&node_id) && ignore) || !dom_adapter.is_node_valid(&node_id) {
            return;
        }

        // Mark this node as dirty
        self.invalidate(node_id);

        if RootNodeCandidate::None == self.root_node_candidate {
            self.root_node_candidate = RootNodeCandidate::Valid(node_id);
        } else if let RootNodeCandidate::Valid(root_candidate) = &mut self.root_node_candidate {
            if node_id != *root_candidate {
                let closest_parent = dom_adapter.closest_common_parent(&node_id, root_candidate);

                if let Some(closest_parent) = closest_parent {
                    *root_candidate = closest_parent;
                }
            }
        }

        // Mark as dirty this Node's children
        for child in dom_adapter.children_of(&node_id) {
            self.check_dirty_dependants(child, dom_adapter, true)
        }

        // Mark this Node's parent if it is affected
        let parent_id = dom_adapter.parent_of(&node_id);

        if let Some(parent_id) = parent_id {
            let parent = dom_adapter.get_node(&parent_id);

            if let Some(parent) = parent {
                // Mark parent if it depeneds on it's inner children
                if parent.does_depend_on_inner() {
                    self.check_dirty_dependants(parent_id, dom_adapter, true);
                }
                // Mark as dirty all the siblings that come after this node
                else {
                    let mut found_node = false;
                    let mut multiple_children = false;
                    for child_id in dom_adapter.children_of(&parent_id) {
                        if found_node {
                            self.check_dirty_dependants(child_id, dom_adapter, true);
                        }
                        if child_id == node_id {
                            found_node = true;
                        } else {
                            multiple_children = true;
                        }
                    }

                    // Try saving using  node's parent as root candidate if it has multiple children
                    if multiple_children {
                        if let RootNodeCandidate::Valid(root_candidate) = self.root_node_candidate {
                            let closest_parent =
                                dom_adapter.closest_common_parent(&parent_id, &root_candidate);

                            if let Some(closest_parent) = closest_parent {
                                self.root_node_candidate = RootNodeCandidate::Valid(closest_parent);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Get the Root Node candidate
    pub fn get_root_candidate(&self) -> RootNodeCandidate<Key> {
        self.root_node_candidate.clone()
    }

    /// Find the best root Node from where to start measuring
    pub fn find_best_root(&mut self, dom_adapter: &mut impl DOMAdapter<Key>) {
        if self.results.is_empty() {
            return;
        }
        for dirty in self.dirty.clone() {
            self.check_dirty_dependants(dirty, dom_adapter, false);
        }
    }

    /// Measure dirty Nodes
    pub fn measure(
        &mut self,
        suggested_root_id: Key,
        root_area: Area,
        measurer: &mut Option<impl LayoutMeasurer<Key>>,
        dom_adapter: &mut impl DOMAdapter<Key>,
    ) {
        // If there are previosuly cached results
        // But no dirty nodes, we can simply skip the measurement
        // as this means no changes has been made to the layout
        if self.dirty.is_empty() && !self.results.is_empty() {
            return;
        }

        // Try the Root candidate otherwise use the provided Root
        let root_id = if let RootNodeCandidate::Valid(id) = self.root_node_candidate.take() {
            id
        } else {
            suggested_root_id
        };
        let root_parent = dom_adapter.parent_of(&root_id);
        let areas = root_parent
            .and_then(|root_parent| self.get(root_parent).cloned())
            .unwrap_or(NodeAreas {
                area: root_area,
                inner_area: root_area,
                inner_sizes: Size2D::default(),
                margin: Gaps::default(),
            });
        let root = dom_adapter.get_node(&root_id).unwrap();
        let root_height = dom_adapter.height(&root_id).unwrap();

        info!(
            "Processing {} dirty nodes and {} cached nodes from a height of {}",
            self.dirty.len(),
            self.results.len(),
            root_height
        );

        let metadata = LayoutMetadata { root_area };

        let (root_revalidated, root_areas) = measure_node(
            root_id,
            &root,
            self,
            &areas.inner_area,
            &areas.inner_area,
            measurer,
            true,
            dom_adapter,
            &metadata,
        );

        // Cache the root Node results if it was modified
        if root_revalidated {
            self.cache_node(root_id, root_areas);
        }

        self.dirty.clear();
        self.root_node_candidate = RootNodeCandidate::None;
    }

    /// Get the areas of a Node
    pub fn get(&self, node_id: Key) -> Option<&NodeAreas> {
        self.results.get(&node_id)
    }

    /// Cache a Node's areas
    pub fn cache_node(&mut self, node_id: Key, areas: NodeAreas) {
        self.results.insert(node_id, areas);
    }
}
