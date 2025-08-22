use crate::{
    custom_measurer::LayoutMeasurer,
    dom_adapter::{DOMAdapter, LayoutNode, NodeKey},
    geometry::{Area, Size2D},
    node::Node,
    prelude::{
        AlignAxis, Alignment, AlignmentDirection, AreaModel, Direction, LayoutMetadata, Length,
        Torin,
    },
    size::Size,
};
pub use euclid::Rect;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::HashMap;
use std::mem;

/// Some layout strategies require two-phase measurements
/// Example: Alignments or content-fit.
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Initial,
    InitialDeferred,
    Final,
}

pub struct MeasureContext<'a, Key, L, D>
where
    Key: NodeKey,
    L: LayoutMeasurer<Key>,
    D: DOMAdapter<Key>,
{
    pub layout: &'a mut Torin<Key>,
    pub measurer: &'a mut Option<L>,
    pub dom_adapter: &'a mut D,
    pub layout_metadata: LayoutMetadata,
}

impl<Key, L, D> MeasureContext<'_, Key, L, D>
where
    Key: NodeKey,
    L: LayoutMeasurer<Key>,
    D: DOMAdapter<Key>,
{
    /// Measure a Node.
    #[allow(clippy::too_many_arguments, clippy::missing_panics_doc)]
    pub fn measure_node(
        &mut self,
        // ID for this Node
        node_id: Key,
        // Data of this Node
        node: &Node,
        // Area occupied by its parent
        parent_area: &Area,
        // Area that is available to use by the children of the parent
        available_parent_area: &Area,
        // Whether to cache the measurements of this Node's children
        must_cache_children: bool,
        // Parent Node is dirty.
        parent_is_dirty: bool,
        // Current phase of measurement
        phase: Phase,
    ) -> (bool, LayoutNode) {
        // 1. If parent is dirty
        // 2. If this Node has been marked as dirty
        // 3. If there is no known cached data about this Node.
        let must_revalidate = parent_is_dirty
            || self.layout.dirty.contains_key(&node_id)
            || !self.layout.results.contains_key(&node_id);
        if must_revalidate {
            // Create the initial Node area size
            let mut area_size = Size2D::new(node.padding.horizontal(), node.padding.vertical());

            // Compute the width and height given the size, the minimum size, the maximum size and margins
            area_size.width = node.width.min_max(
                area_size.width,
                parent_area.size.width,
                available_parent_area.size.width,
                node.margin.left(),
                node.margin.horizontal(),
                &node.minimum_width,
                &node.maximum_width,
                self.layout_metadata.root_area.width(),
                phase,
            );
            area_size.height = node.height.min_max(
                area_size.height,
                parent_area.size.height,
                available_parent_area.size.height,
                node.margin.top(),
                node.margin.vertical(),
                &node.minimum_height,
                &node.maximum_height,
                self.layout_metadata.root_area.height(),
                phase,
            );

            // If available, run a custom layout measure function
            // This is useful when you use third-party libraries (e.g. rust-skia, cosmic-text) to measure text layouts
            let node_data = if let Some(measurer) = self.measurer {
                if measurer.should_measure(node_id) {
                    let available_width =
                        Size::Pixels(Length::new(available_parent_area.size.width)).min_max(
                            area_size.width,
                            parent_area.size.width,
                            available_parent_area.size.width,
                            node.margin.left(),
                            node.margin.horizontal(),
                            &node.minimum_width,
                            &node.maximum_width,
                            self.layout_metadata.root_area.width(),
                            phase,
                        );
                    let available_height =
                        Size::Pixels(Length::new(available_parent_area.size.height)).min_max(
                            area_size.height,
                            parent_area.size.height,
                            available_parent_area.size.height,
                            node.margin.top(),
                            node.margin.vertical(),
                            &node.minimum_height,
                            &node.maximum_height,
                            self.layout_metadata.root_area.height(),
                            phase,
                        );
                    let most_fitting_width = *node
                        .width
                        .most_fitting_size(&area_size.width, &available_width);
                    let most_fitting_height = *node
                        .height
                        .most_fitting_size(&area_size.height, &available_height);

                    let most_fitting_area_size =
                        Size2D::new(most_fitting_width, most_fitting_height);
                    let res = measurer.measure(node_id, node, &most_fitting_area_size);

                    // Compute the width and height again using the new custom area sizes
                    #[allow(clippy::float_cmp)]
                    if let Some((custom_size, node_data)) = res {
                        if node.width.inner_sized() {
                            area_size.width = node.width.min_max(
                                custom_size.width,
                                parent_area.size.width,
                                available_parent_area.size.width,
                                node.margin.left(),
                                node.margin.horizontal(),
                                &node.minimum_width,
                                &node.maximum_width,
                                self.layout_metadata.root_area.width(),
                                phase,
                            );
                        }
                        if node.height.inner_sized() {
                            area_size.height = node.height.min_max(
                                custom_size.height,
                                parent_area.size.height,
                                available_parent_area.size.height,
                                node.margin.top(),
                                node.margin.vertical(),
                                &node.minimum_height,
                                &node.maximum_height,
                                self.layout_metadata.root_area.height(),
                                phase,
                            );
                        }

                        // Do not measure inner children
                        Some(node_data)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            let measure_inner_children = if let Some(measurer) = self.measurer {
                measurer.should_measure_inner_children(node_id)
            } else {
                true
            };

            // There is no need to measure inner children in the initial phase if this Node size
            // isn't decided by his children
            let phase_measure_inner_children =
                if phase == Phase::Initial || phase == Phase::InitialDeferred {
                    node.width.inner_sized() || node.height.inner_sized()
                } else {
                    true
                };

            // Compute the inner size of the Node, which is basically the size inside the margins and paddings
            let inner_size = {
                let mut inner_size = area_size;

                // When having an unsized bound we set it to whatever is still available in the parent's area
                if node.width.inner_sized() {
                    inner_size.width = node.width.min_max(
                        available_parent_area.width(),
                        parent_area.size.width,
                        available_parent_area.width(),
                        node.margin.left(),
                        node.margin.horizontal(),
                        &node.minimum_width,
                        &node.maximum_width,
                        self.layout_metadata.root_area.width(),
                        phase,
                    );
                }
                if node.height.inner_sized() {
                    inner_size.height = node.height.min_max(
                        available_parent_area.height(),
                        parent_area.size.height,
                        available_parent_area.height(),
                        node.margin.top(),
                        node.margin.vertical(),
                        &node.minimum_height,
                        &node.maximum_height,
                        self.layout_metadata.root_area.height(),
                        phase,
                    );
                }
                inner_size
            };

            // Create the areas
            let area_origin = node.position.get_origin(
                available_parent_area,
                parent_area,
                &area_size,
                &self.layout_metadata.root_area,
            );
            let mut area = Rect::new(area_origin, area_size);
            let mut inner_area = Rect::new(area_origin, inner_size)
                .without_gaps(&node.padding)
                .without_gaps(&node.margin);

            let mut inner_sizes = Size2D::default();

            if measure_inner_children && phase_measure_inner_children {
                // Create an area containing the available space inside the inner area
                let mut available_area = inner_area;

                available_area.move_with_offsets(&node.offset_x, &node.offset_y);

                // Measure the layout of this Node's children
                self.measure_children(
                    &node_id,
                    node,
                    &mut available_area,
                    &mut inner_sizes,
                    &mut area,
                    &mut inner_area,
                    must_cache_children,
                    true,
                );

                // Re apply min max values after measuring with inner sized
                // Margins are set to 0 because area.size already contains the margins
                if node.width.inner_sized() {
                    area.size.width = node.width.min_max(
                        area.size.width,
                        parent_area.size.width,
                        available_parent_area.size.width,
                        0.,
                        0.,
                        &node.minimum_width,
                        &node.maximum_width,
                        self.layout_metadata.root_area.width(),
                        phase,
                    );
                }
                if node.height.inner_sized() {
                    area.size.height = node.height.min_max(
                        area.size.height,
                        parent_area.size.height,
                        available_parent_area.size.height,
                        0.,
                        0.,
                        &node.minimum_height,
                        &node.maximum_height,
                        self.layout_metadata.root_area.height(),
                        phase,
                    );
                }
            }

            inner_sizes.width += node.padding.horizontal();
            inner_sizes.height += node.padding.vertical();

            let layout_node = LayoutNode {
                area,
                margin: node.margin,
                inner_area,
                data: node_data,
            };

            // In case of any layout listener, notify it with the new areas.
            if node.has_layout_references {
                if let Some(measurer) = self.measurer {
                    measurer.notify_layout_references(node_id, layout_node.area, inner_sizes);
                }
            }

            (must_cache_children, layout_node)
        } else {
            let layout_node = self.layout.get(node_id).unwrap().clone();

            let mut inner_sizes = Size2D::default();
            let mut available_area = layout_node.inner_area;
            let mut area = layout_node.area;
            let mut inner_area = layout_node.inner_area;

            available_area.move_with_offsets(&node.offset_x, &node.offset_y);

            let measure_inner_children = if let Some(measurer) = self.measurer {
                measurer.should_measure_inner_children(node_id)
            } else {
                true
            };

            if measure_inner_children {
                self.measure_children(
                    &node_id,
                    node,
                    &mut available_area,
                    &mut inner_sizes,
                    &mut area,
                    &mut inner_area,
                    must_cache_children,
                    false,
                );
            }

            (false, layout_node)
        }
    }

    /// Updates the layout for current node based on the measured children.
    #[allow(clippy::too_many_arguments)]
    pub fn measure_children(
        &mut self,
        node_id: &Key,
        node: &Node,
        // Area available for children inside the Node
        available_area: &mut Area,
        // Accumulated sizes in both axis in the Node
        inner_sizes: &mut Size2D,
        // Total area of the node.
        node_area: &mut Area,
        // Inner area of the node.
        inner_area: &mut Area,
        // Whether to cache the measurements of this Node's children
        must_cache_children: bool,
        // Parent Node is dirty.
        node_is_dirty: bool,
    ) {
        let children = self.dom_adapter.children_of(node_id);

        // Used to calculate the spacing and some alignments
        let last_child = if node.spacing.get() > 0. {
            let mut last_child = None;
            for child_id in &children {
                let Some(child_data) = self.dom_adapter.get_node(child_id) else {
                    continue;
                };
                if child_data.position.is_stacked() {
                    last_child = Some(*child_id);
                }
            }
            last_child
        } else {
            children.last().copied()
        };

        let needs_initial_phase = node.cross_alignment.is_not_start()
            || node.main_alignment.is_not_start()
            || node.content.is_fit()
            || node.content.is_flex()
            || node.wrap_content.is_wrap();

        let initial_available_area = *available_area;

        let mut initial_phase_area = *node_area;
        let mut initial_phase_inner_area = *inner_area;
        let mut initial_phase_available_area = *available_area;
        let mut initial_phase_flex_grows: Vec<Length> = Vec::new();
        let mut initial_phase_sizes = FxHashMap::default();
        let mut initial_phase_lines = vec![(0, Size2D::default())];
        let mut initial_phase_inner_sizes = Size2D::default();
        let mut initial_phase_defer = Vec::new();
        let mut defer_size = 0.;

        // Initial phase: Measure the size and position of the children if the parent has a
        // non-start cross alignment, non-start main alignment or a fit-content.
        if needs_initial_phase {
            //  Measure the children
            for child_id in &children {
                let Some(child_data) = self.dom_adapter.get_node(child_id) else {
                    continue;
                };

                // No need to consider this Node for a two-phasing
                // measurements as it will float on its own.
                if !child_data.position.is_stacked() {
                    continue;
                }

                let is_last_child = last_child == Some(*child_id);

                let (_, mut child_areas) = self.measure_node(
                    *child_id,
                    &child_data,
                    &initial_phase_inner_area,
                    &initial_phase_available_area,
                    false,
                    node_is_dirty,
                    Phase::Initial,
                );

                child_areas.area.adjust_size(&child_data);

                let new_line = node.wrap_content.is_wrap()
                    && Self::should_wrap(
                        node,
                        child_areas.area.size,
                        &initial_phase_available_area,
                        &initial_phase_lines,
                    );

                if new_line {
                    for child_id in mem::take(&mut initial_phase_defer) {
                        if let Some(child_data) = self.dom_adapter.get_node(child_id) {
                            self.deferred_measure_child(
                                node,
                                node_is_dirty,
                                child_id,
                                &child_data,
                                &mut initial_phase_inner_area,
                                &initial_phase_available_area,
                                &mut initial_phase_flex_grows,
                                defer_size,
                                &mut initial_phase_sizes,
                                &mut initial_phase_lines,
                            );
                        }
                    }
                    defer_size = 0.;
                    Self::wrap_new_line(
                        node,
                        &mut initial_phase_available_area,
                        &initial_available_area,
                        &mut initial_phase_lines,
                    );
                    initial_phase_flex_grows = Vec::new();
                }

                // Stack this child into the parent
                Self::stack_child(
                    node,
                    &child_data,
                    &mut initial_phase_available_area,
                    &mut initial_phase_area,
                    &mut initial_phase_inner_area,
                    &mut initial_phase_inner_sizes,
                    &mut initial_phase_lines,
                    &child_areas.area,
                    new_line,
                    is_last_child,
                    Phase::Initial,
                );

                if node.cross_alignment.is_not_start()
                    || node.main_alignment.is_spaced()
                    || new_line
                {
                    initial_phase_sizes.insert(*child_id, child_areas.area.size);
                }

                match node.direction {
                    Direction::Vertical => {
                        if let Some(ff) = child_data.height.flex_grow() {
                            initial_phase_flex_grows.push(ff);
                        }
                        if node.content.is_flex() && child_data.height.is_flex()
                            || node.content.is_fit() && child_data.width.is_fill_minimum()
                        {
                            defer_size += child_areas.area.height();
                            initial_phase_defer.push(child_id);
                        }
                    }
                    Direction::Horizontal => {
                        if let Some(ff) = child_data.width.flex_grow() {
                            initial_phase_flex_grows.push(ff);
                        }
                        if node.content.is_flex() && child_data.width.is_flex()
                            || node.content.is_fit() && child_data.height.is_fill_minimum()
                        {
                            defer_size += child_areas.area.width();
                            initial_phase_defer.push(child_id);
                        }
                    }
                }
            }
            for child_id in initial_phase_defer {
                if let Some(child_data) = self.dom_adapter.get_node(child_id) {
                    self.deferred_measure_child(
                        node,
                        node_is_dirty,
                        child_id,
                        &child_data,
                        &mut initial_phase_inner_area,
                        &initial_phase_available_area,
                        &mut initial_phase_flex_grows,
                        defer_size,
                        &mut initial_phase_sizes,
                        &mut initial_phase_lines,
                    );
                }
            }
            if node.height.inner_sized() {
                available_area.size.height = initial_phase_inner_sizes
                    .height
                    .min(available_area.size.height);
            }
            if node.width.inner_sized() {
                available_area.size.width = initial_phase_inner_sizes
                    .width
                    .min(available_area.size.width);
            }
        }

        if node.cross_alignment.is_not_start() {
            // Align the Cross axis (all lines)
            Self::align_content(
                available_area,
                &initial_phase_inner_area,
                initial_phase_inner_sizes,
                &node.cross_alignment,
                &node.direction,
                AlignmentDirection::Cross,
            );
        }

        let initial_available_area = *available_area;

        // Final phase: measure the children with all the axis and sizes adjusted
        let mut curr_line = 0;
        let mut line_index = 0;
        let mut line_origin = available_area.origin;
        let mut lines = vec![(0, Size2D::default())];
        for child_id in children {
            let Some(child_data) = self.dom_adapter.get_node(&child_id) else {
                continue;
            };

            let align_axis = AlignAxis::new(&node.direction, AlignmentDirection::Main);
            let initial_phase_size = initial_phase_sizes.get(&child_id);
            let is_last_child = last_child == Some(child_id);

            let new_line = if let Some(initial_phase_size) = initial_phase_size {
                node.wrap_content.is_wrap()
                    && child_data.position.is_stacked()
                    && Self::should_wrap(node, *initial_phase_size, available_area, &lines)
            } else {
                false
            };

            if new_line {
                Self::wrap_new_line(node, available_area, &initial_available_area, &mut lines);
                line_origin = available_area.origin;
            }

            if child_data.position.is_stacked() {
                // Only the stacked children will be aligned
                if node.main_alignment.is_spaced() {
                    // Align the Main axis if necessary
                    Self::align_position(
                        available_area,
                        &initial_available_area,
                        initial_phase_lines[curr_line].1,
                        &node.main_alignment,
                        &node.direction,
                        AlignmentDirection::Main,
                        initial_phase_lines[curr_line].0,
                        line_index == 0,
                    );
                }

                if node.cross_alignment.is_not_start() {
                    if let Some(initial_phase_size) = initial_phase_size {
                        if line_index == 0 {
                            Self::align_position(
                                available_area,
                                &initial_available_area,
                                initial_phase_inner_sizes,
                                &node.cross_alignment,
                                &node.direction,
                                AlignmentDirection::Cross,
                                initial_phase_lines.len(),
                                curr_line == 0,
                            );
                            match align_axis {
                                AlignAxis::Height => line_origin.x = available_area.origin.x,
                                AlignAxis::Width => line_origin.y = available_area.origin.y,
                            }
                        }
                        // Align the Cross direction (child in line)
                        Self::align_content(
                            available_area,
                            &Area::new(line_origin, initial_phase_lines[curr_line].1),
                            *initial_phase_size,
                            &node.cross_alignment,
                            &node.direction,
                            AlignmentDirection::Cross,
                        );
                    }
                }

                // Align the Main direction (new line)
                if node.main_alignment.is_not_start() && line_index == 0 {
                    Self::align_content(
                        available_area,
                        &available_area.clone(),
                        initial_phase_lines[curr_line].1,
                        &node.main_alignment,
                        &node.direction,
                        AlignmentDirection::Main,
                    );
                    match align_axis {
                        AlignAxis::Height => line_origin.y = available_area.origin.y,
                        AlignAxis::Width => line_origin.x = available_area.origin.x,
                    }
                }
            }

            let mut available_area_in_line = *available_area;
            if needs_initial_phase && child_data.position.is_stacked() {
                if let Some(initial_phase_size) = initial_phase_size {
                    let origin_offset = available_area.origin - line_origin;
                    let line_available = &initial_phase_lines[curr_line].1;

                    let (width, height) = match node.direction {
                        Direction::Vertical => (
                            line_available.width - origin_offset.x,
                            if child_data.height.is_flex() {
                                initial_phase_size.height
                            } else {
                                line_available.height - origin_offset.y
                            },
                        ),
                        Direction::Horizontal => (
                            if child_data.width.is_flex() {
                                initial_phase_size.width
                            } else {
                                line_available.width - origin_offset.x
                            },
                            line_available.height - origin_offset.y,
                        ),
                    };

                    available_area_in_line =
                        Area::new(available_area.origin, Size2D::new(width, height));
                }
            }

            // Final measurement
            let (child_revalidated, mut child_areas) = self.measure_node(
                child_id,
                &child_data,
                inner_area,
                &available_area_in_line,
                must_cache_children,
                node_is_dirty,
                Phase::Final,
            );

            // Adjust the size of the area if needed
            child_areas.area.adjust_size(&child_data);

            // Stack this child into the parent
            if child_data.position.is_stacked() {
                Self::stack_child(
                    node,
                    &child_data,
                    available_area,
                    node_area,
                    inner_area,
                    inner_sizes,
                    &mut lines,
                    &child_areas.area,
                    new_line,
                    is_last_child,
                    Phase::Final,
                );
                line_index += 1;
                if !initial_phase_lines.is_empty() && line_index == initial_phase_lines[curr_line].0
                {
                    curr_line += 1;
                    line_index = 0;
                }
            }

            // Cache the child layout if it was mutated and children must be cached
            if child_revalidated && must_cache_children {
                // Finally cache this node areas into Torin
                self.layout.cache_node(child_id, child_areas);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn deferred_measure_child(
        &mut self,
        node: &Node,
        node_is_dirty: bool,
        child_id: &Key,
        child_data: &Node,
        initial_phase_inner_area: &mut Area,
        initial_phase_available_area: &Area,
        initial_phase_flex_grows: &mut [Length],
        defer_size: f32,
        initial_phase_sizes: &mut HashMap<Key, Size2D, FxBuildHasher>,
        initial_phase_lines: &mut [(usize, Size2D)],
    ) {
        let mut deferred_available_area = *initial_phase_available_area;
        match node.direction {
            Direction::Vertical => {
                deferred_available_area.size.height += defer_size;
            }
            Direction::Horizontal => {
                deferred_available_area.size.width += defer_size;
            }
        }

        let mut deferred_height = None;
        let mut deferred_width = None;

        if node.content.is_flex() {
            let flex_grow = match node.direction {
                Direction::Vertical => child_data.height.flex_grow(),
                Direction::Horizontal => child_data.width.flex_grow(),
            };

            if let Some(flex_grow) = flex_grow {
                let (flex_grows, flex_available) = Self::calculate_available_flex_size(
                    initial_phase_flex_grows,
                    &node.direction,
                    &deferred_available_area,
                    &mut initial_phase_lines.last_mut().unwrap().1,
                );
                let flex_grow_per = flex_grow.get() / flex_grows.get() * 100.;

                match node.direction {
                    Direction::Vertical => {
                        let size = flex_available / 100. * flex_grow_per;
                        deferred_height = Some(size.get());
                    }
                    Direction::Horizontal => {
                        let size = flex_available / 100. * flex_grow_per;
                        deferred_width = Some(size.get());
                    }
                }
            }
        }

        if node.content.is_fit() {
            match node.direction {
                Direction::Vertical => {
                    if child_data.width.is_fill_minimum() {
                        deferred_width = Some(initial_phase_lines.last().unwrap().1.width);
                    }
                }
                Direction::Horizontal => {
                    if child_data.height.is_fill_minimum() {
                        deferred_height = Some(initial_phase_lines.last().unwrap().1.height);
                    }
                }
            }
        }

        let available_width = deferred_width.unwrap_or(deferred_available_area.width());
        let available_height = deferred_height.unwrap_or(deferred_available_area.height());

        let (_, mut child_areas) = self.measure_node(
            *child_id,
            child_data,
            initial_phase_inner_area,
            &Area::new(
                deferred_available_area.origin,
                Size2D::new(available_width, available_height),
            ),
            false,
            node_is_dirty,
            Phase::InitialDeferred,
        );

        child_areas.area.adjust_size(child_data);
        initial_phase_sizes.insert(*child_id, child_areas.area.size);
    }

    fn calculate_available_flex_size(
        initial_flex_grows: &[Length],
        direction: &Direction,
        available_area: &Area,
        line_size: &mut Size2D,
    ) -> (Length, Length) {
        let flex_grows = initial_flex_grows
            .iter()
            .copied()
            .reduce(|acc, v| acc + v)
            .unwrap_or_default()
            .max(Length::new(1.0));

        let flex_available;
        match direction {
            Direction::Vertical => {
                flex_available = Length::new(available_area.height());

                initial_flex_grows.iter().fold(line_size, |acc, f| {
                    let flex_grow_per = f.get() / flex_grows.get() * 100.;

                    let size = flex_available / 100. * flex_grow_per;
                    acc.height += size.get();

                    acc
                });
            }
            Direction::Horizontal => {
                flex_available = Length::new(available_area.width());

                initial_flex_grows.iter().fold(line_size, |acc, f| {
                    let flex_grow_per = f.get() / flex_grows.get() * 100.;

                    let size = flex_available / 100. * flex_grow_per;
                    acc.width += size.get();

                    acc
                });
            }
        }
        (flex_grows, flex_available)
    }

    /// Align the content of this node.
    fn align_content(
        available_area: &mut Area,
        inner_area: &Area,
        contents_size: Size2D,
        alignment: &Alignment,
        direction: &Direction,
        alignment_direction: AlignmentDirection,
    ) {
        let axis = AlignAxis::new(direction, alignment_direction);

        match axis {
            AlignAxis::Height => match alignment {
                Alignment::Center => {
                    let new_origin_y = (inner_area.height() / 2.0) - (contents_size.height / 2.0);
                    available_area.origin.y = inner_area.min_y() + new_origin_y;
                }
                Alignment::End => {
                    available_area.origin.y = inner_area.max_y() - contents_size.height;
                }
                _ => {}
            },
            AlignAxis::Width => match alignment {
                Alignment::Center => {
                    let new_origin_x = (inner_area.width() / 2.0) - (contents_size.width / 2.0);
                    available_area.origin.x = inner_area.min_x() + new_origin_x;
                }
                Alignment::End => {
                    available_area.origin.x = inner_area.max_x() - contents_size.width;
                }
                _ => {}
            },
        }
    }

    /// Align the position of this node.
    #[allow(clippy::too_many_arguments)]
    fn align_position(
        available_area: &mut Area,
        initial_available_area: &Area,
        inner_sizes: Size2D,
        alignment: &Alignment,
        direction: &Direction,
        alignment_direction: AlignmentDirection,
        siblings_len: usize,
        is_first_sibling: bool,
    ) {
        let axis = AlignAxis::new(direction, alignment_direction);

        match axis {
            AlignAxis::Height => match alignment {
                Alignment::SpaceBetween if !is_first_sibling => {
                    let all_gaps_sizes = initial_available_area.height() - inner_sizes.height;
                    let gap_size = all_gaps_sizes / (siblings_len - 1) as f32;
                    available_area.origin.y += gap_size;
                }
                Alignment::SpaceEvenly => {
                    let all_gaps_sizes = initial_available_area.height() - inner_sizes.height;
                    let gap_size = all_gaps_sizes / (siblings_len + 1) as f32;
                    available_area.origin.y += gap_size;
                }
                Alignment::SpaceAround => {
                    let all_gaps_sizes = initial_available_area.height() - inner_sizes.height;
                    let one_gap_size = all_gaps_sizes / siblings_len as f32;
                    let gap_size = if is_first_sibling {
                        one_gap_size / 2.
                    } else {
                        one_gap_size
                    };
                    available_area.origin.y += gap_size;
                }
                _ => {}
            },
            AlignAxis::Width => match alignment {
                Alignment::SpaceBetween if !is_first_sibling => {
                    let all_gaps_sizes = initial_available_area.width() - inner_sizes.width;
                    let gap_size = all_gaps_sizes / (siblings_len - 1) as f32;
                    available_area.origin.x += gap_size;
                }
                Alignment::SpaceEvenly => {
                    let all_gaps_sizes = initial_available_area.width() - inner_sizes.width;
                    let gap_size = all_gaps_sizes / (siblings_len + 1) as f32;
                    available_area.origin.x += gap_size;
                }
                Alignment::SpaceAround => {
                    let all_gaps_sizes = initial_available_area.width() - inner_sizes.width;
                    let one_gap_size = all_gaps_sizes / siblings_len as f32;
                    let gap_size = if is_first_sibling {
                        one_gap_size / 2.
                    } else {
                        one_gap_size
                    };
                    available_area.origin.x += gap_size;
                }
                _ => {}
            },
        }
    }

    /// Updates layout of the current node as a child node is stacked into the current node in
    /// either a horizontal or vertical direction.
    ///
    /// Mutable parameters:
    /// - `available_area`: Shifted forward (in x or y, depending on direction) to reserve space
    ///   for the current child and prepare for the next sibling. Its size is reduced accordingly.
    ///
    /// - `node_area`: Total area used by the node. If its size is determined by its children,
    ///   this value is updated accordingly at the start of a new line, and at the very last sibling
    ///
    /// - `inner_area`: Kept in sync with `node_area` but excludes padding and margin. It reflects
    ///   the actual space available for child layout inside the parent.
    ///
    /// - `inner_sizes`: Accumulates the total width and height occupied by children.
    ///
    /// - `line_sizes`: Accumulates the width and height of children in the same line. A line is a row
    ///    or column, depending on the direction of the node. A wrapping node can have multiple lines.
    #[allow(clippy::too_many_arguments)]
    fn stack_child(
        node: &Node,
        child_node: &Node,
        available_area: &mut Area,
        node_area: &mut Area,
        inner_area: &mut Area,
        inner_sizes: &mut Size2D,
        line_sizes: &mut [(usize, Size2D)],
        child_area: &Area,
        new_line: bool,
        is_last_sibling: bool,
        phase: Phase,
    ) {
        match node.direction {
            Direction::Horizontal => {
                let (cur_line_len, cur_line) = line_sizes.last_mut().unwrap();
                *cur_line_len += 1;

                // Don't apply spacing to last child
                let spacing = (!is_last_sibling)
                    .then_some(node.spacing)
                    .unwrap_or_default();

                // update size of current line
                cur_line.height = cur_line.height.max(child_area.height());
                cur_line.width += spacing.get();
                // we only know child's correct flex sizing in the final phase
                if !child_node.width.is_flex() || phase == Phase::Final {
                    cur_line.width += child_area.size.width;
                }

                // move available area for next sibling
                available_area.origin.x += child_area.size.width + spacing.get();
                available_area.size.width -= child_area.size.width + spacing.get();

                let mut update_inner_sizes = |line: &mut Size2D, inner_sizes: &mut Size2D| {
                    inner_sizes.height += line.height;
                    inner_sizes.width = inner_sizes.width.max(line.width);

                    if node.height.inner_sized() {
                        node_area.size.height =
                            inner_sizes.height + node.padding.vertical() + node.margin.vertical();

                        // Keep the inner area in sync
                        inner_area.size.height = node_area.size.height
                            - node.padding.vertical()
                            - node.margin.vertical();
                    }

                    if node.width.inner_sized() {
                        node_area.size.width = node_area.size.width.max(
                            inner_sizes.width
                                + node.padding.horizontal()
                                + node.margin.horizontal(),
                        );
                        // Keep the inner area in sync
                        inner_area.size.width = node_area.size.width
                            - node.padding.horizontal()
                            - node.margin.horizontal();
                    }
                };

                if is_last_sibling {
                    update_inner_sizes(cur_line, inner_sizes);
                }

                if new_line {
                    inner_sizes.height += node.spacing.get();
                    let amount_lines = line_sizes.len();
                    update_inner_sizes(&mut line_sizes[amount_lines - 2].1, inner_sizes);
                }
            }
            Direction::Vertical => {
                let (cur_line_len, cur_line) = line_sizes.last_mut().unwrap();
                *cur_line_len += 1;

                // Don't apply spacing to last child
                let spacing = (!is_last_sibling)
                    .then_some(node.spacing)
                    .unwrap_or_default();

                // update size of current line
                cur_line.width = cur_line.width.max(child_area.width());
                cur_line.height += spacing.get();
                // we only know child's correct flex sizing in the final phase
                if !child_node.height.is_flex() || phase == Phase::Final {
                    cur_line.height += child_area.size.height;
                }

                // move available area for next sibling
                available_area.origin.y += child_area.size.height + spacing.get();
                available_area.size.height -= child_area.size.height + spacing.get();

                // end of line, update inner size
                let mut update_inner_sizes = |line: &mut Size2D, inner_sizes: &mut Size2D| {
                    inner_sizes.width += line.width;
                    inner_sizes.height = inner_sizes.height.max(line.height);

                    if node.width.inner_sized() {
                        node_area.size.width = inner_sizes.width
                            + node.padding.horizontal()
                            + node.margin.horizontal();
                        // Keep the inner area in sync
                        inner_area.size.width = node_area.size.width
                            - node.padding.horizontal()
                            - node.margin.horizontal();
                    }

                    if node.height.inner_sized() {
                        node_area.size.height = node_area.size.height.max(
                            inner_sizes.height + node.padding.vertical() + node.margin.vertical(),
                        );
                        // Keep the inner area in sync
                        inner_area.size.height = node_area.size.height
                            - node.padding.vertical()
                            - node.margin.vertical();
                    }
                };

                if is_last_sibling {
                    update_inner_sizes(cur_line, inner_sizes);
                }

                if new_line {
                    let amount_lines = line_sizes.len();
                    inner_sizes.width += node.spacing.get();
                    update_inner_sizes(&mut line_sizes[amount_lines - 2].1, inner_sizes);
                }
            }
        }
    }

    fn should_wrap(
        node: &Node,
        child_size: Size2D,
        available_area: &Area,
        line_sizes: &[(usize, Size2D)],
    ) -> bool {
        match node.direction {
            Direction::Vertical => {
                node.wrap_content.is_wrap()
                    && !line_sizes.is_empty()
                    && child_size.height > available_area.size.height
            }
            Direction::Horizontal => {
                node.wrap_content.is_wrap()
                    && !line_sizes.is_empty()
                    && child_size.width > available_area.size.width
            }
        }
    }

    fn wrap_new_line(
        node: &Node,
        available_area: &mut Area,
        initial_available_area: &Area,
        line_sizes: &mut Vec<(usize, Size2D)>,
    ) {
        match node.direction {
            Direction::Vertical => {
                if let Some((_, line_size)) = line_sizes.last_mut() {
                    line_size.height -= node.spacing.get();
                    // move available area for new line
                    available_area.origin.y = initial_available_area.origin.y;
                    available_area.origin.x += line_size.width + node.spacing.get();
                    available_area.size.height = initial_available_area.size.height;
                    available_area.size.width -= line_size.width + node.spacing.get();
                    line_sizes.push((0, Size2D::default()));
                }
            }
            Direction::Horizontal => {
                if let Some((_, line_size)) = line_sizes.last_mut() {
                    line_size.width -= node.spacing.get();
                    // move available area for new line
                    available_area.origin.x = initial_available_area.origin.x;
                    available_area.origin.y += line_size.height + node.spacing.get();
                    available_area.size.width = initial_available_area.size.width;
                    available_area.size.height -= line_size.height + node.spacing.get();
                    line_sizes.push((0, Size2D::default()));
                }
            }
        }
    }
}
