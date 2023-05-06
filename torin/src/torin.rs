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
    size::Size,
};

/// Indicates what's the closest Node to the root from which start measuring the dirty Nodes
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TallestDirtyNode<Key: NodeKey> {
    /// A valid Node ID
    Valid(Key),

    /// Coudln't decide
    Invalid,

    /// Not decided yet
    None,
}

pub struct Torin<Key: NodeKey> {
    /// Layout results of the registered Nodes
    pub results: FxHashMap<Key, NodeAreas>,

    /// Invalid registered nodes since previous layout measurement
    pub dirty: HashSet<Key>,

    /// Closes dirty Node to the Root
    pub tallest_dirty_node: TallestDirtyNode<Key>,
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
            results: HashMap::default(),
            dirty: HashSet::new(),
            tallest_dirty_node: TallestDirtyNode::None,
        }
    }

    /// Reset the layout
    pub fn reset(&mut self) {
        self.tallest_dirty_node = TallestDirtyNode::Invalid;
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
        if let TallestDirtyNode::Valid(id) = self.tallest_dirty_node {
            if id == node_id {
                self.tallest_dirty_node = TallestDirtyNode::None
            }
        }
    }

    /// Remove a Node from the layout
    pub fn remove(
        &mut self,
        node_id: Key,
        node_resolver: &impl DOMAdapter<Key>,
        invalidate_parent: bool,
    ) {
        // Remove itself
        self.raw_remove(node_id);

        // Mark as dirty the Node's parent
        if invalidate_parent {
            self.invalidate(node_resolver.parent_of(&node_id).unwrap());
        }

        // Remove all it's children
        for child_id in node_resolver.children_of(&node_id) {
            self.remove(child_id, node_resolver, false);
        }
    }

    /// Mark as dirty a Node
    pub fn invalidate(&mut self, node_id: Key) {
        self.dirty.insert(node_id);
    }

    /// Mark as dirty a Node
    pub fn safe_invalidate(&mut self, node_id: Key, node_resolver: &impl DOMAdapter<Key>) {
        if node_resolver.is_node_valid(&node_id) {
            self.dirty.insert(node_id);
        }
    }

    // Mark as dirty the given Node and all the nodes that depend on it
    pub fn check_dirty_dependants(
        &mut self,
        node_id: Key,
        node_resolver: &impl DOMAdapter<Key>,
        ignore: bool,
    ) {
        if self.dirty.contains(&node_id) && ignore {
            return;
        }

        // Mark this node as dirty
        self.invalidate(node_id);

        // Save the tallest dirty Node
        if TallestDirtyNode::None == self.tallest_dirty_node {
            self.tallest_dirty_node = TallestDirtyNode::Valid(node_id);
        } else if let TallestDirtyNode::Valid(tallest_dirty_node) = self.tallest_dirty_node {
            let node_height = node_resolver.height(&node_id);
            let current_height = node_resolver.height(&tallest_dirty_node);

            if node_id != tallest_dirty_node {
                match node_height.cmp(&current_height) {
                    // Update the tallest node if this node is dirty and is taller than the current tallest node
                    std::cmp::Ordering::Less => {
                        self.tallest_dirty_node = TallestDirtyNode::Valid(node_id);
                    }
                    // If both this node and the tallest Node are in the same height, we set the tallest node as invalid
                    // as we can't figure out which is one is the tallest
                    //
                    // Improvements idea:
                    // It could try finding the closest common parent Node from these two and set it as tallest Node
                    std::cmp::Ordering::Equal => {
                        self.tallest_dirty_node = TallestDirtyNode::Invalid
                    }
                    _ => {}
                }
            }
        }

        // Mark as dirty this Node's children
        for child in node_resolver.children_of(&node_id) {
            self.check_dirty_dependants(child, node_resolver, true)
        }

        // Mark this Node's parent if it is affected
        let parent_id = node_resolver.parent_of(&node_id);

        if let Some(parent_id) = parent_id {
            let parent = node_resolver.get_node(&parent_id);

            if let Some(parent) = parent {
                // Mark parent if it depeneds on it's inner children
                if parent.does_depend_on_inner() {
                    self.check_dirty_dependants(parent_id, node_resolver, false);
                }
                // Otherwise we simply mark this Node siblings
                else {
                    // TODO(marc2332): Only mark those who come before this node.
                    for child_id in node_resolver.children_of(&parent_id) {
                        if child_id != node_id {
                            self.check_dirty_dependants(child_id, node_resolver, true)
                        }
                    }
                }
            }
        }
    }

    /// Get the closest dirty node to the root
    pub fn get_tallest_dirty_node(&self) -> TallestDirtyNode<Key> {
        self.tallest_dirty_node
    }

    /// Find the best root Node from where to start measuring
    pub fn find_best_root(&mut self, node_resolver: &impl DOMAdapter<Key>) {
        if self.results.is_empty() {
            return;
        }
        for dirty in self.dirty.clone() {
            if node_resolver.is_node_valid(&dirty) {
                self.check_dirty_dependants(dirty, node_resolver, false);
            }
        }
    }

    /// Measure dirty Nodes
    pub fn measure(
        &mut self,
        suggested_root_id: Key,
        suggested_root_area: Area,
        measurer: &mut Option<impl LayoutMeasurer<Key>>,
        node_resolver: &impl DOMAdapter<Key>,
    ) {
        // If there are previosuly cached results
        // But no dirty nodes, we can simply skip the measurement
        // as this means no changes has been made to the layout
        if self.dirty.is_empty() && !self.results.is_empty() {
            return;
        }

        info!(
            "Found {} dirty nodes and {} cached nodes",
            self.dirty.len(),
            self.results.len()
        );

        // Try using the closest Node to the root that is dirty, otherwise use the provided Root
        let root_id = if let TallestDirtyNode::Valid(id) = self.tallest_dirty_node {
            id
        } else {
            suggested_root_id
        };
        let root_parent = node_resolver.parent_of(&root_id);
        let areas = root_parent
            .and_then(|root_parent| self.get(root_parent).cloned())
            .unwrap_or(NodeAreas {
                area: suggested_root_area,
                inner_area: suggested_root_area,
                inner_sizes: Size2D::default(),
            });
        let root = node_resolver.get_node(&root_id).unwrap();

        let (root_revalidated, root_areas) = measure_node(
            root_id,
            &root,
            self,
            &areas.area,
            &areas.inner_area,
            measurer,
            true,
            node_resolver,
        );

        // Cache the root Node results if it was modified
        if root_revalidated {
            self.cache_node(root_id, root_areas);
        }

        self.dirty.clear();
        self.tallest_dirty_node = TallestDirtyNode::None;
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
    node_resolver: &impl DOMAdapter<Key>,
) -> (bool, NodeAreas) {
    let must_run = layout.dirty.contains(&node_id) || layout.results.get(&node_id).is_none();
    if must_run {
        let mut area = Rect::new(available_parent_area.origin, Size2D::default());

        area.size.width = node
            .width
            .eval(parent_area.size.width)
            .unwrap_or(area.size.width);
        area.size.height = node
            .height
            .eval(parent_area.size.height)
            .unwrap_or(area.size.height);

        let minimum_width = node.minimum_width.eval(parent_area.size.width);
        let maximum_width = node.maximum_width.eval(parent_area.size.width);

        let minimum_height = node.minimum_height.eval(parent_area.size.height);
        let maximum_height = node.maximum_height.eval(parent_area.size.height);

        area.size.width = area.size.width.clamp(
            minimum_width.unwrap_or(area.size.width),
            maximum_width.unwrap_or(area.size.width),
        );
        area.size.height = area.size.height.clamp(
            minimum_height.unwrap_or(area.size.height),
            maximum_height.unwrap_or(area.size.height),
        );

        // Custom measure
        let skip_inner = if let Some(measurer) = measurer {
            let custom_measure =
                measurer.measure(node_id, node, &area, parent_area, available_parent_area);
            if let Some(new_area) = custom_measure {
                if Size::Inner == node.width {
                    area.size.width = new_area.width().clamp(
                        minimum_width.unwrap_or(new_area.width()),
                        maximum_width.unwrap_or(new_area.width()),
                    );
                }
                if Size::Inner == node.height {
                    area.size.height = new_area.height().clamp(
                        minimum_height.unwrap_or(new_area.height()),
                        maximum_height.unwrap_or(new_area.height()),
                    );
                }
            }
            custom_measure.is_some()
        } else {
            false
        };

        let horizontal_padding = node.padding.horizontal_paddings();
        let vertical_padding = node.padding.vertical_paddings();

        let mut inner_sizes = Size2D::default();

        // Node's inner area
        let mut inner_area = {
            let mut inner_area = area;
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
        available_area.origin.x += node.scroll_x.get();
        available_area.origin.y += node.scroll_y.get();

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
                node_resolver,
            );
        }

        (
            must_cache,
            NodeAreas {
                area,
                inner_area,
                inner_sizes,
            },
        )
    } else {
        let areas = layout.get(node_id).unwrap().clone();

        let mut inner_sizes = areas.inner_sizes;
        let mut available_area = areas.inner_area;

        // TODO(marc2332): Should I also cache these?
        available_area.origin.x += node.scroll_x.get();
        available_area.origin.y += node.scroll_y.get();

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
            node_resolver,
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
    node_resolver: &impl DOMAdapter<Key>,
) {
    let children = node_resolver.children_of(node_id);

    // Center display

    if node.display == DisplayMode::Center {
        let child_id = children.first();

        if let Some(child_id) = child_id {
            let inner_area = *mode.inner_area();
            let child_data = node_resolver.get_node(child_id).unwrap();

            let (_, child_areas) = measure_node(
                *child_id,
                &child_data,
                layout,
                &inner_area,
                available_area,
                measurer,
                false,
                node_resolver,
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

        let child_data = node_resolver.get_node(&child_id).unwrap().clone();

        let (child_revalidated, child_areas) = measure_node(
            child_id,
            &child_data,
            layout,
            &inner_area,
            available_area,
            measurer,
            must_cache,
            node_resolver,
        );

        match node.direction {
            DirectionMode::Horizontal => {
                // Move the available area
                available_area.origin.x = child_areas.area.max_x();
                available_area.size.width -= child_areas.area.size.width;

                if let MeasureMode::ParentIsNotCached {
                    area,
                    vertical_padding,
                    horizontal_padding,
                    inner_area,
                } = mode
                {
                    inner_sizes.height = child_areas.area.height();
                    inner_sizes.width += child_areas.area.width();

                    // Keep the biggest height
                    if node.height == Size::Inner {
                        area.size.height =
                            area.size.height.max(child_areas.area.size.height) + *vertical_padding;
                        // Keep the inner area in sync
                        inner_area.size.height = area.size.height - *vertical_padding;
                    }

                    // Accumulate width
                    if node.width == Size::Inner {
                        area.size.width += child_areas.area.size.width + *horizontal_padding;
                    }
                }
            }
            DirectionMode::Vertical => {
                // Move the available area
                available_area.origin.y = child_areas.area.max_y();
                available_area.size.height -= child_areas.area.size.height;

                if let MeasureMode::ParentIsNotCached {
                    area,
                    vertical_padding,
                    horizontal_padding,
                    inner_area,
                } = mode
                {
                    inner_sizes.width = child_areas.area.width();
                    inner_sizes.height += child_areas.area.height();

                    // Keep the biggest width
                    if node.width == Size::Inner {
                        area.size.width =
                            area.size.width.max(child_areas.area.size.width) + *horizontal_padding;
                        // Keep the inner area in sync
                        inner_area.size.width = area.size.width - *horizontal_padding;
                    }

                    // Accumulate height
                    if node.height == Size::Inner {
                        area.size.height += child_areas.area.size.height + *vertical_padding;
                    }
                }
            }
            DirectionMode::Both => {
                // Move the available area
                available_area.origin.x = child_areas.area.max_x();
                available_area.origin.y = child_areas.area.max_y();

                available_area.size.width -= child_areas.area.size.width;
                available_area.size.height -= child_areas.area.size.height;

                if let MeasureMode::ParentIsNotCached {
                    area,
                    vertical_padding,
                    horizontal_padding,
                    ..
                } = mode
                {
                    inner_sizes.width += child_areas.area.width();
                    inner_sizes.height += child_areas.area.height();

                    // Accumulate width
                    if node.width == Size::Inner {
                        area.size.width += child_areas.area.size.width + *horizontal_padding;
                    }

                    // Accumulate height
                    if node.height == Size::Inner {
                        area.size.height += child_areas.area.size.height + *vertical_padding;
                    }
                }
            }
        }

        if child_revalidated && must_cache {
            layout.cache_node(child_id, child_areas);
        }
    }
}
