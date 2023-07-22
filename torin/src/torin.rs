use std::collections::{HashMap, HashSet};

pub use euclid::Rect;
use fxhash::FxHashMap;
use tracing::info;

use crate::{
    custom_measurer::LayoutMeasurer,
    direction::DirectionMode,
    display::DisplayMode,
    dom_adapter::{DOMAdapter, NodeAreas, NodeKey},
    geometry::{Area, Size2D},
    node::Node,
    prelude::{BoxModel, Gaps},
    size::Size,
};

/// Contains the best Root node candidate from where to start measuring
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RootNodeCandidate<Key: NodeKey> {
    /// A valid Node ID
    Valid(Key),

    /// None
    None,
}

pub struct Torin<Key: NodeKey> {
    /// Layout results of the registered Nodes
    pub results: FxHashMap<Key, NodeAreas>,

    /// Invalid registered nodes since previous layout measurement
    pub dirty: HashSet<Key>,

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
        dom_adapter: &impl DOMAdapter<NodeId>,
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
            dirty: HashSet::new(),
            root_node_candidate: RootNodeCandidate::None,
        }
    }

    /// Reset the layout
    pub fn reset(&mut self) {
        self.root_node_candidate = RootNodeCandidate::None;
        self.results.clear();
        self.dirty.clear();
    }

    /// Read the HashSet of dirty nodes
    pub fn get_dirty_nodes(&self) -> &HashSet<Key> {
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
        dom_adapter: &impl DOMAdapter<Key>,
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

    pub fn safe_invalidate(&mut self, node_id: Key, dom_adapter: &impl DOMAdapter<Key>) {
        if dom_adapter.is_node_valid(&node_id) {
            self.invalidate(node_id)
        }
    }

    // Mark as dirty the given Node and all the nodes that depend on it
    pub fn check_dirty_dependants(
        &mut self,
        node_id: Key,
        dom_adapter: &impl DOMAdapter<Key>,
        ignore: bool,
    ) {
        if (self.dirty.contains(&node_id) && ignore) || !dom_adapter.is_node_valid(&node_id) {
            return;
        }

        // Mark this node as dirty
        self.invalidate(node_id);

        if RootNodeCandidate::None == self.root_node_candidate {
            self.root_node_candidate = RootNodeCandidate::Valid(node_id);
        } else if let RootNodeCandidate::Valid(root_candidate) = self.root_node_candidate {
            if node_id != root_candidate {
                let closest_parent = dom_adapter.closest_common_parent(&node_id, &root_candidate);

                if let Some(closest_parent) = closest_parent {
                    self.root_node_candidate = RootNodeCandidate::Valid(closest_parent);
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
                // Otherwise we simply mark this Node siblings
                else {
                    // TODO(marc2332): Only mark those who come before this node.
                    for child_id in dom_adapter.children_of(&parent_id) {
                        if child_id != node_id {
                            self.check_dirty_dependants(child_id, dom_adapter, true)
                        }
                    }
                }
            }
        }
    }

    /// Get the Root Node candidate
    pub fn get_root_candidate(&self) -> RootNodeCandidate<Key> {
        self.root_node_candidate
    }

    /// Find the best root Node from where to start measuring
    pub fn find_best_root(&mut self, dom_adapter: &impl DOMAdapter<Key>) {
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
        suggested_root_area: Area,
        measurer: &mut Option<impl LayoutMeasurer<Key>>,
        dom_adapter: &impl DOMAdapter<Key>,
    ) {
        // If there are previosuly cached results
        // But no dirty nodes, we can simply skip the measurement
        // as this means no changes has been made to the layout
        if self.dirty.is_empty() && !self.results.is_empty() {
            return;
        }

        // Try the Root candidate otherwise use the provided Root
        let root_id = if let RootNodeCandidate::Valid(id) = self.root_node_candidate {
            id
        } else {
            suggested_root_id
        };
        let root_parent = dom_adapter.parent_of(&root_id);
        let areas = root_parent
            .and_then(|root_parent| self.get(root_parent).cloned())
            .unwrap_or(NodeAreas {
                area: suggested_root_area,
                inner_area: suggested_root_area,
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

        let (root_revalidated, root_areas) = measure_node(
            root_id,
            &root,
            self,
            &areas.inner_area,
            &areas.inner_area,
            measurer,
            true,
            dom_adapter,
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

/// Measure this node and all it's children
/// The caller of this function is responsible of caching the Node's layout results
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn measure_node<Key: NodeKey>(
    node_id: Key,
    node: &Node,
    layout: &mut Torin<Key>,
    parent_area: &Area,
    available_parent_area: &Area,
    measurer: &mut Option<impl LayoutMeasurer<Key>>,
    must_cache: bool,
    dom_adapter: &impl DOMAdapter<Key>,
) -> (bool, NodeAreas) {
    let must_run = layout.dirty.contains(&node_id) || layout.results.get(&node_id).is_none();
    if must_run {
        let horizontal_padding = node.padding.horizontal();
        let vertical_padding = node.padding.vertical();

        let mut area = Rect::new(
            available_parent_area.origin,
            Size2D::new(horizontal_padding, vertical_padding),
        );

        area.size.width = node.width.min_max(
            area.size.width,
            parent_area.size.width,
            node.margin.horizontal(),
            &node.minimum_width,
            &node.maximum_width,
        );
        area.size.height = node.height.min_max(
            area.size.height,
            parent_area.size.height,
            node.margin.vertical(),
            &node.minimum_height,
            &node.maximum_height,
        );

        // Custom measure
        let skip_inner = if let Some(measurer) = measurer {
            let custom_measure =
                measurer.measure(node_id, node, &area, parent_area, available_parent_area);
            if let Some(new_area) = custom_measure {
                if Size::Inner == node.width {
                    area.size.width = node.width.min_max(
                        new_area.width(),
                        parent_area.size.width,
                        node.margin.horizontal(),
                        &node.minimum_width,
                        &node.maximum_width,
                    );
                }
                if Size::Inner == node.height {
                    area.size.height = node.height.min_max(
                        new_area.height(),
                        parent_area.size.height,
                        node.margin.vertical(),
                        &node.minimum_height,
                        &node.maximum_height,
                    );
                }
            }
            custom_measure.is_some()
        } else {
            false
        };

        let mut inner_sizes = Size2D::default();

        // Node's inner area
        let mut inner_area = {
            let mut inner_area = area.box_area(&node.margin);
            if Size::Inner == node.width {
                inner_area.size.width = available_parent_area.width()
            }
            if Size::Inner == node.height {
                inner_area.size.height = available_parent_area.height()
            }
            inner_area
        };

        // Apply padding
        inner_area.origin.x += node.padding.left();
        inner_area.origin.y += node.padding.top();
        inner_area.size.width -= horizontal_padding;
        inner_area.size.height -= vertical_padding;

        // Node's available inner area
        let mut available_area = inner_area;

        // Apply scroll
        available_area.origin.x += node.offset_x.get();
        available_area.origin.y += node.offset_y.get();

        let mut measurement_mode = MeasureMode::ParentIsNotCached {
            area: &mut area,
            inner_area: &mut inner_area,
            vertical_padding,
            horizontal_padding,
        };

        if !skip_inner {
            measure_inner_nodes(
                &node_id,
                node,
                layout,
                &mut available_area,
                &mut inner_sizes,
                measurer,
                must_cache,
                &mut measurement_mode,
                dom_adapter,
            );
        }

        (
            must_cache,
            NodeAreas {
                area,
                margin: node.margin,
                inner_area,
                inner_sizes,
            },
        )
    } else {
        let areas = layout.get(node_id).unwrap().clone();

        let mut inner_sizes = areas.inner_sizes;
        let mut available_area = areas.inner_area;

        // TODO(marc2332): Should I also cache these?
        available_area.origin.x += node.offset_x.get();
        available_area.origin.y += node.offset_y.get();

        let mut measurement_mode = MeasureMode::ParentIsCached {
            inner_area: &areas.inner_area,
        };

        measure_inner_nodes(
            &node_id,
            node,
            layout,
            &mut available_area,
            &mut inner_sizes,
            measurer,
            must_cache,
            &mut measurement_mode,
            dom_adapter,
        );

        (false, areas)
    }
}

/// Measurement data for the inner Nodes of a Node
#[derive(Debug)]
enum MeasureMode<'a> {
    ParentIsCached {
        inner_area: &'a Area,
    },
    ParentIsNotCached {
        area: &'a mut Area,
        inner_area: &'a mut Area,
        vertical_padding: f32,
        horizontal_padding: f32,
    },
}

impl<'a> MeasureMode<'a> {
    /// Get a reference to the inner area
    pub fn inner_area(&'a self) -> &'a Area {
        match self {
            Self::ParentIsCached { inner_area } => inner_area,
            Self::ParentIsNotCached { inner_area, .. } => inner_area,
        }
    }
}

/// Measure the inner Nodes of a Node
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn measure_inner_nodes<Key: NodeKey>(
    node_id: &Key,
    node: &Node,
    layout: &mut Torin<Key>,
    available_area: &mut Area,
    inner_sizes: &mut Size2D,
    measurer: &mut Option<impl LayoutMeasurer<Key>>,
    must_cache: bool,
    mode: &mut MeasureMode,
    dom_adapter: &impl DOMAdapter<Key>,
) {
    let children = dom_adapter.children_of(node_id);

    // Center display

    if node.display == DisplayMode::Center {
        let child_id = children.first();

        if let Some(child_id) = child_id {
            let inner_area = *mode.inner_area();
            let child_data = dom_adapter.get_node(child_id).unwrap();

            let (_, child_areas) = measure_node(
                *child_id,
                &child_data,
                layout,
                &inner_area,
                available_area,
                measurer,
                false,
                dom_adapter,
            );

            // TODO(marc2332): Should I also reduce the width and heights?
            match node.direction {
                DirectionMode::Horizontal => {
                    let new_origin_x =
                        (inner_area.width() / 2.0) - (child_areas.area.width() / 2.0);
                    available_area.origin.x = inner_area.min_x() + new_origin_x;
                }
                DirectionMode::Vertical => {
                    let new_origin_y =
                        (inner_area.height() / 2.0) - (child_areas.area.height() / 2.0);
                    available_area.origin.y = inner_area.min_y() + new_origin_y;
                }
                DirectionMode::Both => {
                    let new_origin_x =
                        (inner_area.width() / 2.0) - (child_areas.area.width() / 2.0);
                    let new_origin_y =
                        (inner_area.height() / 2.0) - (child_areas.area.height() / 2.0);
                    available_area.origin.x = inner_area.min_x() + new_origin_x;
                    available_area.origin.y = inner_area.min_y() + new_origin_y;
                }
            }
        }
    }

    // Normal display

    for child_id in children {
        let inner_area = *mode.inner_area();

        let child_data = dom_adapter.get_node(&child_id).unwrap().clone();

        let (child_revalidated, child_areas) = measure_node(
            child_id,
            &child_data,
            layout,
            &inner_area,
            available_area,
            measurer,
            must_cache,
            dom_adapter,
        );

        match node.direction {
            DirectionMode::Horizontal => {
                // Move the available area
                available_area.origin.x = child_areas.area.max_x();
                available_area.size.width -= child_areas.area.size.width;

                if let MeasureMode::ParentIsNotCached {
                    area,
                    vertical_padding,
                    inner_area,
                    ..
                } = mode
                {
                    inner_sizes.height = child_areas.area.height();
                    inner_sizes.width += child_areas.area.width();

                    // Keep the biggest height
                    if node.height == Size::Inner {
                        area.size.height = area
                            .size
                            .height
                            .max(child_areas.area.size.height + *vertical_padding);
                        // Keep the inner area in sync
                        inner_area.size.height = area.size.height - *vertical_padding;
                    }

                    // Accumulate width
                    if node.width == Size::Inner {
                        area.size.width += child_areas.area.size.width;
                    }
                }
            }
            DirectionMode::Vertical => {
                // Move the available area
                available_area.origin.y = child_areas.area.max_y();
                available_area.size.height -= child_areas.area.size.height;

                if let MeasureMode::ParentIsNotCached {
                    area,
                    horizontal_padding,
                    inner_area,
                    ..
                } = mode
                {
                    inner_sizes.width = child_areas.area.width();
                    inner_sizes.height += child_areas.area.height();

                    // Keep the biggest width
                    if node.width == Size::Inner {
                        area.size.width = area
                            .size
                            .width
                            .max(child_areas.area.size.width + *horizontal_padding);
                        // Keep the inner area in sync
                        inner_area.size.width = area.size.width - *horizontal_padding;
                    }

                    // Accumulate height
                    if node.height == Size::Inner {
                        area.size.height += child_areas.area.size.height;
                    }
                }
            }
            DirectionMode::Both => {
                // Move the available area
                available_area.origin.x = child_areas.area.max_x();
                available_area.origin.y = child_areas.area.max_y();

                available_area.size.width -= child_areas.area.size.width;
                available_area.size.height -= child_areas.area.size.height;

                if let MeasureMode::ParentIsNotCached { area, .. } = mode {
                    inner_sizes.width += child_areas.area.width();
                    inner_sizes.height += child_areas.area.height();

                    // Accumulate width
                    if node.width == Size::Inner {
                        area.size.width += child_areas.area.size.width;
                    }

                    // Accumulate height
                    if node.height == Size::Inner {
                        area.size.height += child_areas.area.size.height;
                    }
                }
            }
        }

        if child_revalidated && must_cache {
            layout.cache_node(child_id, child_areas);
        }
    }
}
