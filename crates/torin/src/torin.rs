use alloc::vec::Vec;
use core::mem;

use itertools::Itertools;

use crate::{
    collections::FxHashMap,
    custom_measurer::LayoutMeasurer,
    geometry::Area,
    measure::{
        MeasureContext,
        Phase,
    },
    prelude::{
        AreaConverter,
        AreaModel,
        Gaps,
        Length,
        Size2D,
    },
    tree_adapter::{
        LayoutNode,
        NodeKey,
        TreeAdapter,
    },
};

pub struct LayoutMetadata {
    pub root_area: Area,
}

/// Non-overlapping roots from which to start incremental measurement.
#[derive(PartialEq, Debug, Clone)]
pub enum RootNodeCandidate<Key: NodeKey> {
    /// A valid Node ID
    Valid(Key),

    /// Independent branch roots.
    Multiple(Vec<Key>),

    /// None
    None,
}

impl<Key: NodeKey> RootNodeCandidate<Key> {
    #[must_use]
    pub const fn take(&mut self) -> Self {
        mem::replace(self, Self::None)
    }

    /// Retain disjoint roots and collapse roots covered by an ancestor.
    pub fn propose_new_candidate(
        &mut self,
        proposed_candidate: &Key,
        tree_adapter: &impl TreeAdapter<Key>,
        dirty: &mut FxHashMap<Key, DirtyReason>,
    ) {
        if matches!(self, Self::Valid(current) if *current == *proposed_candidate) {
            return;
        }

        let candidates = match self.take() {
            Self::None => {
                *self = Self::Valid(*proposed_candidate);
                return;
            }
            Self::Valid(candidate) => alloc::vec![candidate],
            Self::Multiple(candidates) => candidates,
        };
        let mut roots = Vec::with_capacity(candidates.len() + 1);
        let mut covered = false;

        for current_candidate in candidates {
            if current_candidate == *proposed_candidate {
                covered = true;
                roots.push(current_candidate);
            } else {
                let mut continue_walking = true;
                let closest_parent = tree_adapter.closest_common_parent(
                    proposed_candidate,
                    &current_candidate,
                    |id| {
                        if !continue_walking {
                            return;
                        }
                        let reason = dirty.get(&id);
                        match reason {
                            Some(DirtyReason::InnerLayout) => {
                                // Remeasure intermediate nodes instead of only translating their children.
                                dirty.insert(id, DirtyReason::None);
                            }
                            Some(DirtyReason::None | DirtyReason::Reorder)
                                if id != *proposed_candidate =>
                            {
                                // Stop updating dirty reasons once an ancestor already requires remeasurement.
                                continue_walking = false;
                            }
                            _ => {}
                        }
                    },
                );

                // Keep covering ancestors and independent branches, dropping covered descendants.
                if closest_parent == Some(current_candidate) {
                    covered = true;
                    roots.push(current_candidate);
                } else if closest_parent != Some(*proposed_candidate) {
                    roots.push(current_candidate);
                }
            }
        }

        // Add the proposed root only when no retained ancestor covers it.
        if !covered {
            roots.push(*proposed_candidate);
        }

        *self = if roots.len() == 1 {
            Self::Valid(roots[0])
        } else {
            Self::Multiple(roots)
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DirtyReason {
    None,
    /// Node was moved from one position to another in its parent' children list.
    Reorder,
    /// The inner layout of the Node changed, e.g the offsets.
    InnerLayout,
}

pub struct Torin<Key: NodeKey> {
    /// Layout results of the registered Nodes
    pub results: FxHashMap<Key, LayoutNode>,

    /// Invalid registered nodes since previous layout measurement
    pub dirty: FxHashMap<Key, DirtyReason>,

    /// Non-overlapping incremental measurement roots.
    pub root_node_candidate: RootNodeCandidate<Key>,
}

impl<Key: NodeKey> Default for Torin<Key> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Key: NodeKey> Torin<Key> {
    /// Create a new Layout
    pub fn new() -> Self {
        Self {
            results: FxHashMap::default(),
            dirty: FxHashMap::default(),
            root_node_candidate: RootNodeCandidate::None,
        }
    }

    pub fn size(&self) -> usize {
        self.results.len()
    }

    /// Clear the dirty nodes buffer
    pub fn clear_dirty(&mut self) {
        self.dirty.clear();
    }

    /// Reset the layout
    pub fn reset(&mut self) {
        self.root_node_candidate = RootNodeCandidate::None;
        self.results.clear();
        self.dirty.clear();
    }

    /// Read the HashSet of dirty nodes
    pub const fn get_dirty_nodes(&self) -> &FxHashMap<Key, DirtyReason> {
        &self.dirty
    }

    /// Remove a Node's result and data
    pub fn raw_remove(&mut self, node_id: Key) {
        self.results.remove(&node_id);
        self.dirty.remove(&node_id);
        match &mut self.root_node_candidate {
            RootNodeCandidate::Valid(id) if *id == node_id => {
                self.root_node_candidate = RootNodeCandidate::None;
            }
            RootNodeCandidate::Multiple(roots) => {
                roots.retain(|id| *id != node_id);
                if roots.len() == 1 {
                    self.root_node_candidate = RootNodeCandidate::Valid(roots[0]);
                } else if roots.is_empty() {
                    self.root_node_candidate = RootNodeCandidate::None;
                }
            }
            _ => {}
        }
    }

    /// Remove a Node from the layout
    /// # Panics
    /// Might panic if the parent is not found.
    pub fn remove(
        &mut self,
        node_id: Key,
        tree_adapter: &impl TreeAdapter<Key>,
        invalidate_parent: bool,
    ) {
        // Remove itself
        self.raw_remove(node_id);

        // Mark as dirty the Node's parent
        if invalidate_parent && let Some(parent) = tree_adapter.parent_of(&node_id) {
            self.invalidate(parent);
        }

        // Remove all it's children
        tree_adapter.read_node(&node_id, |_, children| {
            for child_id in children {
                self.remove(*child_id, tree_adapter, false);
            }
        });
    }

    /// Safely mark as dirty a Node, with no reason.
    pub fn safe_invalidate(&mut self, node_id: Key) {
        self.dirty.insert(node_id, DirtyReason::None);
    }

    /// Mark as dirty a Node, with no reason.
    pub fn invalidate(&mut self, node_id: Key) {
        self.dirty.insert(node_id, DirtyReason::None);
    }

    /// Mark as dirty a Node, with a reason.
    pub fn invalidate_with_reason(&mut self, node_id: Key, reason: DirtyReason) {
        self.dirty.entry(node_id).or_insert(reason);
    }

    // Mark as dirty the given Node and all the nodes that depend on it
    pub fn check_dirty_dependants(
        &mut self,
        node_id: Key,
        reason: DirtyReason,
        tree_adapter: &impl TreeAdapter<Key>,
        ignore: bool,
    ) {
        if self.dirty.contains_key(&node_id) && ignore {
            return;
        }

        // Mark this node as dirty
        self.invalidate_with_reason(node_id, reason);

        self.root_node_candidate
            .propose_new_candidate(&node_id, tree_adapter, &mut self.dirty);

        // Mark this Node's parent if it is affected
        let parent_id = tree_adapter.parent_of(&node_id);

        if let Some(parent_id) = parent_id {
            if reason == DirtyReason::InnerLayout {
                self.root_node_candidate.propose_new_candidate(
                    &parent_id,
                    tree_adapter,
                    &mut self.dirty,
                );
                return;
            }

            tree_adapter.read_node(&parent_id, |parent, parent_children| {
                if parent.does_depend_on_inner() {
                    // Mark parent if it depends on it's inner children
                    self.check_dirty_dependants(parent_id, DirtyReason::None, tree_adapter, true);
                } else {
                    let mut found_node = match reason {
                        DirtyReason::None | DirtyReason::InnerLayout => false,
                        // Invalidate all siblings if the node was reordered
                        DirtyReason::Reorder => true,
                    };
                    for child_id in parent_children {
                        if found_node {
                            self.safe_invalidate(*child_id);
                        }
                        if *child_id == node_id {
                            found_node = true;
                        }
                    }

                    // Refresh the parent's accumulated content sizes and sibling positions.
                    self.root_node_candidate.propose_new_candidate(
                        &parent_id,
                        tree_adapter,
                        &mut self.dirty,
                    );
                }
            });
        }
    }

    /// Get the non-overlapping measurement roots.
    pub fn get_root_candidate(&self) -> RootNodeCandidate<Key> {
        self.root_node_candidate.clone()
    }

    /// Resolve independent measurement roots through layout dependencies.
    pub fn find_best_root(&mut self, tree_adapter: &impl TreeAdapter<Key>) {
        if self.results.is_empty() {
            return;
        }
        for (id, reason) in self
            .dirty
            .clone()
            .into_iter()
            .sorted_by_key(|e| tree_adapter.height(&e.0))
        {
            self.check_dirty_dependants(id, reason, tree_adapter, false);
        }
    }

    /// Measure dirty Nodes
    /// # Panics
    /// Might panic if the final root node is not found.
    pub fn measure(
        &mut self,
        suggested_root_id: Key,
        root_area: Area,
        measurer: &mut Option<impl LayoutMeasurer<Key>>,
        tree_adapter: &impl TreeAdapter<Key>,
    ) {
        // If there are previously cached results
        // But no dirty nodes, we can simply skip the measurement
        // as this means no changes has been made to the layout
        if self.dirty.is_empty() && !self.results.is_empty() {
            return;
        }

        match self.root_node_candidate.take() {
            RootNodeCandidate::Valid(id) => {
                self.measure_root(id, root_area, measurer, tree_adapter);
            }
            RootNodeCandidate::Multiple(roots) => {
                for root_id in roots {
                    self.measure_root(root_id, root_area, measurer, tree_adapter);
                }
            }
            RootNodeCandidate::None => {
                self.measure_root(suggested_root_id, root_area, measurer, tree_adapter);
            }
        }

        self.dirty.clear();
    }

    fn measure_root(
        &mut self,
        root_id: Key,
        root_area: Area,
        measurer: &mut Option<impl LayoutMeasurer<Key>>,
        tree_adapter: &impl TreeAdapter<Key>,
    ) {
        let root_parent_id = tree_adapter.parent_of(&root_id);
        let layout_node = root_parent_id
            .and_then(|root_parent_id| self.get(&root_parent_id).cloned())
            .unwrap_or(LayoutNode {
                area: root_area,
                inner_area: root_area.as_inner(),
                inner_sizes: Size2D::default(),
                margin: Gaps::default(),
                offset_x: Length::default(),
                offset_y: Length::default(),
                hidden: false,
                data: None,
            });

        let layout_metadata = LayoutMetadata { root_area };

        let inner_area = layout_node.inner_area.as_inner();

        let available_area = inner_area.as_available();

        tree_adapter
            .read_node(&root_id, |root, root_children| {
                let mut measure_context = MeasureContext {
                    layout: self,
                    layout_metadata,
                    tree_adapter,
                    measurer,
                };

                let (root_revalidated, mut root_layout_node) = measure_context.measure_node(
                    root_id,
                    root,
                    root_children,
                    inner_area.as_parent(),
                    available_area,
                    true,
                    false,
                    Phase::Final,
                );

                // Cache the root Node results if it was modified
                if root_revalidated {
                    // Adjust the size of the area if needed
                    root_layout_node.area.adjust_size(root);

                    measure_context.layout.cache_node(root_id, root_layout_node);
                }
            })
            .expect("Root node does not exist");
    }

    /// Get a reference to [LayoutNode] of a Node
    pub fn get(&self, node_id: &Key) -> Option<&LayoutNode> {
        self.results.get(node_id)
    }

    /// Get a mutable reference to [LayoutNode] of a Node
    pub fn get_mut(&mut self, node_id: &Key) -> Option<&mut LayoutNode> {
        self.results.get_mut(node_id)
    }

    /// Cache a Node's [LayoutNode]
    pub fn cache_node(&mut self, node_id: Key, layout_node: LayoutNode) {
        self.results.insert(node_id, layout_node);
    }
}
