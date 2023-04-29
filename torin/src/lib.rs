use std::collections::{HashMap, HashSet};

use dioxus_native_core::NodeId;
pub use euclid::{Length, Rect, Size2D};
use fxhash::FxHashMap;

pub type Area = Rect<f32, Measure>;

/// Cached layout results of a Node
#[derive(Debug, PartialEq, Clone, Default)]
pub struct NodeAreas {
    pub area: Rect<f32, Measure>,
    pub inner_area: Rect<f32, Measure>,
}

pub trait NodeKey: Clone + PartialEq + Eq + std::hash::Hash + Copy + std::fmt::Debug {}

impl NodeKey for usize {}
impl NodeKey for NodeId {}

/// Node's data for the layout algorithm
#[derive(PartialEq, Clone, Debug)]
pub struct NodeData<Key: NodeKey, Data: Clone + std::fmt::Debug> {
    pub data: Data,
    pub parent: Option<Key>,
    pub children: Vec<Key>,
    pub depth: usize,
    node: Node,
}

impl<Key: NodeKey, Data: Clone + std::fmt::Debug> NodeData<Key, Data> {
    /// Has properties that depend on the inner Nodes?
    pub fn does_depend_on_inner(&self) -> bool {
        Size::Inner == self.node.width || Size::Inner == self.node.height
    }

    /// Has properties that depend on the parent Node?
    pub fn does_depend_on_parent(&self) -> bool {
        matches!(self.node.width, Size::Percentage(_))
            || matches!(self.node.height, Size::Percentage(_))
            || matches!(self.node.height, Size::DynamicCalculations(_))
            || matches!(self.node.height, Size::DynamicCalculations(_))
    }
}

#[derive(Clone, Default, Debug)]
pub struct EmbeddedData {
    text: Option<String>,
}

pub struct Torin<Key: NodeKey, Data: Clone + std::fmt::Debug> {
    /// Registered Nodes
    pub nodes: FxHashMap<Key, NodeData<Key, Data>>,

    /// Layout results of the registered Nodes
    pub results: FxHashMap<Key, NodeAreas>,

    /// Invalid registered nodes since previous layout measurement
    pub dirty: HashSet<Key>,

    /// Closes dirty Node to the Root
    pub tallest_dirty_node: Option<Key>,
}

impl<Key: NodeKey, Data: Clone + std::fmt::Debug> Default for Torin<Key, Data> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Key: NodeKey, Data: Clone + std::fmt::Debug> Torin<Key, Data> {
    /// Create a new Layout
    pub fn new() -> Self {
        Self {
            nodes: FxHashMap::default(),
            results: HashMap::default(),
            dirty: HashSet::new(),
            tallest_dirty_node: None,
        }
    }

    pub fn clear(&mut self) {
        self.results.clear();
        self.dirty.clear();
    }

    /// Mark as dirty the given Node
    pub fn invalidate_node(&mut self, node_id: Key) {
        self.dirty.insert(node_id);
        self.check_dirty_dependants(node_id);
    }

    /// Remove all Node results
    pub fn invalidate_all_nodes(&mut self) {
        self.results.clear();
        self.dirty.clear();
    }

    /// Read the HashSet of dirty nodes
    pub fn get_dirty_nodes(&self) -> &HashSet<Key> {
        &self.dirty
    }

    // Update a Node
    pub fn set_node(&mut self, node_id: Key, node: Node) {
        self.nodes.get_mut(&node_id).unwrap().node = node;

        self.check_dirty_dependants(node_id);
    }

    /// Remove a Node's result and data
    pub fn raw_remove(&mut self, node_id: Key) {
        self.results.remove(&node_id);
        self.nodes.remove(&node_id);
        self.dirty.retain(|n| n != &node_id);
    }

    /// Remove a Node from the layout
    pub fn remove(&mut self, node_id: Key) {
        let node = self.safe_get(node_id);

        if let Some(node) = node {
            // Remove all it's children
            for child_id in node.children {
                self.raw_remove(child_id);
            }

            // Check for collisions
            self.check_dirty_dependants(node_id);

            // Remove itself
            self.raw_remove(node_id);

            // Remove it from it's parent
            for (_, node) in self.nodes.iter_mut() {
                if node.children.contains(&node_id) {
                    node.children.retain(|it| it != &node_id)
                }
            }
        }
    }

    /// Get the closes dirty node to the root
    pub fn get_tallest_dirty_node(&self) -> Option<Key> {
        self.tallest_dirty_node
    }

    // Mark as dirty the given node and all the nodes that depend on it
    pub fn check_dirty_dependants(&mut self, node_id: Key) {
        if self.dirty.contains(&node_id) {
            return;
        }

        // Mark this node
        self.dirty.insert(node_id);

        let self_node = self.get(node_id);

        println!("> {:?} {:?}", node_id, self_node);

        // Save the tallest dirty Node
        if self.tallest_dirty_node.is_none() {
            self.tallest_dirty_node = Some(node_id);
        } else if let Some(tallest_dirty_node) = self.tallest_dirty_node {
            if self_node.depth < self.get(tallest_dirty_node).depth {
                self.tallest_dirty_node = Some(node_id);
            }
        }

        // Mark the children
        for child in &self_node.children {
            self.check_dirty_dependants(*child)
        }

        // Mark this Node's parent if it is affected
        let parent = self
            .nodes
            .iter()
            .find(|(_, node)| node.children.contains(&node_id))
            .map(|(k, c)| (k.clone(), c.clone()));

        if let Some((parent_id, parent)) = parent {
            // Mark parent
            if parent.does_depend_on_inner() {
                self.check_dirty_dependants(parent_id);
            }

            // Mark siblings
            // TODO: Only mark those who come before this node.
            for child in &parent.children {
                if *child != node_id {
                    self.check_dirty_dependants(*child)
                }
            }
        }
       
    }

    pub fn is_dirty(&self) -> bool {
        self.results.is_empty() || !self.dirty.is_empty()
    }

    /// Add a node to the layout without a parent
    pub fn add(
        &mut self,
        node_id: Key,
        node: Node,
        data: Data,
        parent: Option<Key>,
        children: Vec<Key>,
    ) {
        let depth = parent
            .map(|p| Some(self.safe_get(p)?.depth))
            .unwrap_or(Some(0))
            .unwrap_or(0)
            + 1;
        self.nodes.insert(
            node_id,
            NodeData {
                node,
                data,
                children,
                parent,
                depth,
            },
        );
    }

    /// Has a Node
    pub fn has(&self, node_id: Key) -> bool {
        self.nodes.get(&node_id).is_some()
    }

    /// Get a Node's data
    pub fn safe_get(&self, node_id: Key) -> Option<NodeData<Key, Data>> {
        self.nodes.get(&node_id).cloned()
    }

    /// Get a Node's data
    pub fn get(&self, node_id: Key) -> NodeData<Key, Data> {
        self.nodes.get(&node_id).unwrap().clone()
    }

    /// Add a Node to the layout under the given parent
    pub fn insert(
        &mut self,
        node_id: Key,
        parent: Key,
        node: Node,
        data: Data,
        children: Vec<Key>,
    ) {
        self.add(node_id, node, data, Some(parent), children);
    }

    /// Measure a root Node
    pub fn measure(
        &mut self,
        root_id: Key,
        root_area: Rect<f32, Measure>,
        measurer: &mut Option<impl LayoutMeasurer<Key, Data>>,
    ) {
        // If there are previosuly cached results
        // But no dirty nodes, we can simply skip the measurement
        if self.dirty.is_empty() && !self.results.is_empty() {
            return;
        }

        // Try to find the closest dirty Node to the root,
        // otherwise just use the give root_id
        let root_id = self.tallest_dirty_node.unwrap_or(root_id);
        let root_area = self
            .get_size(root_id)
            .map(|areas| areas.area)
            .unwrap_or(root_area);

        let root = self.nodes.get(&root_id).unwrap();

        let (root_revalidated, root_areas) = measure_node(
            root_id,
            root.clone(),
            self,
            &root_area,
            &mut root_area.clone(),
            measurer,
            true,
        );

        if root_revalidated {
            self.save(root_id, root_areas);
        }

        self.dirty.clear();
        self.tallest_dirty_node = None;
    }

    /// Get the size of a Node
    pub fn get_size(&self, node_id: Key) -> Option<&NodeAreas> {
        self.results.get(&node_id)
    }

    /// Cache a Node's areas
    pub fn save(&mut self, node_id: Key, areas: NodeAreas) {
        self.results.insert(node_id, areas);
    }
}

/// Measure this node and all it's children
/// The caller of this function is responsible of caching the Node's layout results
#[inline(always)]
fn measure_node<Key: NodeKey, Data: Clone + std::fmt::Debug>(
    node_id: Key,
    node: NodeData<Key, Data>,
    layout: &mut Torin<Key, Data>,
    parent_area: &Rect<f32, Measure>,
    available_parent_size: &mut Rect<f32, Measure>, // TODO(marc2332): Does it make sense this to be an area or should just be an origin pointer?
    measurer: &mut Option<impl LayoutMeasurer<Key, Data>>,
    must_cache: bool,
) -> (bool, NodeAreas) {
    let must_run = layout.dirty.contains(&node_id) || layout.results.get(&node_id).is_none();
    if must_run {
        let mut area = Rect::new(available_parent_size.origin, Size2D::default());

        match &node.node.width {
            Size::Pixels(px) => {
                area.size.width += px.get();
            }
            Size::Percentage(per) => {
                area.size.width += parent_area.size.width / 100.0 * per.get();
            }
            Size::DynamicCalculations(calculations) => {
                area.size.width += run_calculations(&calculations, parent_area.size.width);
            }
            _ => {}
        }

        match &node.node.height {
            Size::Pixels(px) => {
                area.size.height += px.get();
            }
            Size::Percentage(per) => {
                area.size.height += parent_area.size.height / 100.0 * per.get();
            }
            Size::DynamicCalculations(calculations) => {
                area.size.height += run_calculations(&calculations, parent_area.size.height)
            }
            _ => {}
        }

        // Custom measure
        if let Some(measurer) = measurer {
            let custom_measure = measurer.measure(&node, &area, parent_area, available_parent_size);
            if let Some(res) = custom_measure {
                area = res;
            }
        }

        let horizontal_padding = node.node.padding.1.get() + node.node.padding.3.get();
        let vertical_padding = node.node.padding.0.get() + node.node.padding.2.get();

        // Node's inner area
        let mut inner_area = {
            let mut inner_area = area;
            match node.node.width {
                Size::Inner => inner_area.size.width = available_parent_size.width(),
                _ => {}
            }
            match node.node.width {
                Size::Inner => inner_area.size.height = available_parent_size.height(),
                _ => {}
            }
            inner_area
        };

        // Apply padding
        inner_area.origin.x += node.node.padding.3.get();
        inner_area.origin.y += node.node.padding.0.get();
        inner_area.size.width -= horizontal_padding;
        inner_area.size.height -= vertical_padding;

        // Node's available inner area
        let mut available_area = inner_area;

        // Apply scroll
        available_area.origin.x += node.node.scroll_x.get();
        available_area.origin.y += node.node.scroll_y.get();

        let mut measurement_mode = MeasureMode::ParentIsNotCached {
            area: &mut area,
            inner_area: &mut inner_area,
            vertical_padding,
            horizontal_padding,
        };

        measure_inner_nodes(
            &node,
            layout,
            &mut available_area,
            measurer,
            must_cache,
            &mut measurement_mode,
        );

        (must_cache, NodeAreas { area, inner_area })
    } else {
        let areas = layout.get_size(node_id).unwrap().clone();

        let mut available_area = areas.inner_area;

        let mut measurement_mode = MeasureMode::ParentIsCached {
            inner_area: &areas.inner_area,
        };

        measure_inner_nodes(
            &node,
            layout,
            &mut available_area,
            measurer,
            must_cache,
            &mut measurement_mode,
        );

        (false, areas)
    }
}

enum MeasureMode<'a> {
    ParentIsCached {
        inner_area: &'a Rect<f32, Measure>,
    },
    ParentIsNotCached {
        area: &'a mut Rect<f32, Measure>,
        inner_area: &'a mut Rect<f32, Measure>,
        vertical_padding: f32,
        horizontal_padding: f32,
    },
}

impl<'a> MeasureMode<'a> {
    pub fn inner_area(&'a self) -> &'a Rect<f32, Measure> {
        match self {
            Self::ParentIsCached { inner_area } => inner_area,
            Self::ParentIsNotCached { inner_area, .. } => inner_area,
        }
    }
}

/// Measure the inner Nodes of a Node
#[inline(always)]
fn measure_inner_nodes<Key: NodeKey, Data: Clone + std::fmt::Debug>(
    node: &NodeData<Key, Data>,
    layout: &mut Torin<Key, Data>,
    available_area: &mut Rect<f32, Measure>,
    measurer: &mut Option<impl LayoutMeasurer<Key, Data>>,
    must_cache: bool,
    mode: &mut MeasureMode,
) {
    let mut inner_area = *mode.inner_area();

    // Center display

    if node.node.display == Display::Center {
        let child_id = node.children.first();

        if let Some(child_id) = child_id {
            let child_data = layout.get(*child_id);

            let (_, child_areas) = measure_node(
                *child_id,
                child_data,
                layout,
                &inner_area,
                available_area,
                measurer,
                false,
            );

            // TODO: Should I also reduce the width and heights?
            if node.node.direction == Direction::Horizontal {
                let new_origin_x = (inner_area.width() / 2.0) - (child_areas.area.width() / 2.0);
                available_area.origin.x = inner_area.min_x() + new_origin_x;
            } else {
                let new_origin_y = (inner_area.height() / 2.0) - (child_areas.area.height() / 2.0);
                available_area.origin.y = inner_area.min_y() + new_origin_y;
            }
        }
    }

    // Normal display

    for child_id in &node.children {
        let child_data = layout.get(*child_id);

        let (child_revalidated, child_areas) = measure_node(
            *child_id,
            child_data,
            layout,
            &inner_area,
            available_area,
            measurer,
            must_cache,
        );

        if node.node.direction == Direction::Horizontal {
            // Move the available area
            available_area.origin.x = child_areas.area.max_x();
            available_area.size.width -= child_areas.area.size.width;

            if let MeasureMode::ParentIsNotCached {
                area,
                vertical_padding,
                horizontal_padding,
                ..
            } = mode
            {
                // Keep the biggest height
                if node.node.height == Size::Inner {
                    area.size.height =
                        area.size.height.max(child_areas.area.size.height) + *vertical_padding;
                    // Keep the inner area in sync
                    inner_area.size.height = area.size.height - *vertical_padding;
                }

                // Accumulate width
                if node.node.width == Size::Inner {
                    area.size.width += child_areas.area.size.width + *horizontal_padding;
                    // Keep the inner area in sync
                    inner_area.size.width = area.size.width - *horizontal_padding;
                }
            }
        } else {
            // Move the available area
            available_area.origin.y = child_areas.area.max_y();
            available_area.size.height -= child_areas.area.size.height;

            if let MeasureMode::ParentIsNotCached {
                area,
                vertical_padding,
                horizontal_padding,
                ..
            } = mode
            {
                // Keep the biggest height
                if node.node.width == Size::Inner {
                    area.size.width =
                        area.size.width.max(child_areas.area.size.width) + *horizontal_padding;
                    // Keep the inner area in sync
                    inner_area.size.width = area.size.width - *horizontal_padding;
                }

                // Accumulate height
                if node.node.height == Size::Inner {
                    area.size.height += child_areas.area.size.height + *vertical_padding;
                    // Keep the inner area in sync
                    inner_area.size.height = area.size.height - *vertical_padding;
                }
            }
        }

        if child_revalidated && must_cache {
            layout.save(*child_id, child_areas);
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Vertical
    }
}

#[derive(PartialEq)]
pub struct Measure;

#[derive(PartialEq, Clone, Debug)]
pub enum Size {
    Inner,
    Percentage(Length<f32, Measure>),
    Pixels(Length<f32, Measure>),
    DynamicCalculations(Vec<DynamicCalculation>),
}

impl Default for Size {
    fn default() -> Self {
        Self::Inner
    }
}

pub type Paddings = (
    Length<f32, Measure>,
    Length<f32, Measure>,
    Length<f32, Measure>,
    Length<f32, Measure>,
);

#[derive(PartialEq, Clone, Debug)]
pub enum Display {
    Normal,
    Center,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    pub width: Size,
    pub height: Size,

    pub display: Display,

    pub padding: Paddings,

    pub scroll_x: Length<f32, Measure>,
    pub scroll_y: Length<f32, Measure>,

    /// Direction in which it's inner Nodes will be stacked
    pub direction: Direction,
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    /// Create a Node with the default values
    pub fn new() -> Self {
        Self {
            width: Size::default(),
            height: Size::default(),
            direction: Direction::Vertical,
            scroll_x: Length::new(0.0),
            scroll_y: Length::new(0.0),
            padding: (
                Length::new(0.0),
                Length::new(0.0),
                Length::new(0.0),
                Length::new(0.0),
            ),
            display: Display::Normal,
        }
    }

    /// Construct a new Node given a size and a direction
    pub fn from_size_and_direction(width: Size, height: Size, direction: Direction) -> Self {
        Self {
            width,
            height,
            direction,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and a scroll
    pub fn from_size_and_scroll(
        width: Size,
        height: Size,
        scroll_x: Length<f32, Measure>,
        scroll_y: Length<f32, Measure>,
    ) -> Self {
        Self {
            width,
            height,
            scroll_x,
            scroll_y,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and padding
    pub fn from_size_and_padding(width: Size, height: Size, padding: Paddings) -> Self {
        Self {
            width,
            height,
            padding,
            ..Default::default()
        }
    }

    /// Construct a new Node given a size and a display
    pub fn from_size_and_display_and_direction(
        width: Size,
        height: Size,
        display: Display,
        direction: Direction,
    ) -> Self {
        Self {
            width,
            height,
            display,
            direction,
            ..Default::default()
        }
    }
}

pub trait LayoutMeasurer<Key: NodeKey, Data: Clone + std::fmt::Debug> {
    fn measure(
        &mut self,
        node: &NodeData<Key, Data>,
        area: &Rect<f32, Measure>,
        parent_area: &Rect<f32, Measure>,
        available_parent_size: &Rect<f32, Measure>,
    ) -> Option<Rect<f32, Measure>>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DynamicCalculation {
    Sub,
    Mul,
    Div,
    Add,
    Percentage(f32),
    Pixels(f32),
}

impl std::fmt::Display for DynamicCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicCalculation::Sub => f.write_str("-"),
            DynamicCalculation::Mul => f.write_str("*"),
            DynamicCalculation::Div => f.write_str("/"),
            DynamicCalculation::Add => f.write_str("+"),
            DynamicCalculation::Percentage(p) => f.write_fmt(format_args!("{p}%")),
            DynamicCalculation::Pixels(s) => f.write_fmt(format_args!("{s}")),
        }
    }
}

/// Calculate some chained operations with a given value.
/// This value could be for example the width of a node's parent area.
pub fn run_calculations(calcs: &Vec<DynamicCalculation>, value: f32) -> f32 {
    let mut prev_number: Option<f32> = None;
    let mut prev_op: Option<DynamicCalculation> = None;

    let mut calc_with_op = |val: f32, prev_op: Option<DynamicCalculation>| {
        if let Some(op) = prev_op {
            match op {
                DynamicCalculation::Sub => {
                    prev_number = Some(prev_number.unwrap() - val);
                }
                DynamicCalculation::Add => {
                    prev_number = Some(prev_number.unwrap() + val);
                }
                DynamicCalculation::Mul => {
                    prev_number = Some(prev_number.unwrap() * val);
                }
                DynamicCalculation::Div => {
                    prev_number = Some(prev_number.unwrap() / val);
                }
                _ => {}
            }
        } else {
            prev_number = Some(val);
        }
    };

    for calc in calcs {
        match calc {
            DynamicCalculation::Percentage(per) => {
                let val = (value / 100.0 * per).round();

                calc_with_op(val, prev_op);

                prev_op = None;
            }
            DynamicCalculation::Pixels(val) => {
                calc_with_op(*val, prev_op);
                prev_op = None;
            }
            _ => prev_op = Some(*calc),
        }
    }

    prev_number.unwrap()
}
