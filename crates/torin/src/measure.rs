pub use euclid::Rect;
use rustc_hash::FxHashMap;

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

/// Some layout strategies require two-phase measurements
/// Example: Alignments or content-fit.
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Initial,
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
            let phase_measure_inner_children = if phase == Phase::Initial {
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
            for child_id in children.iter() {
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
            || node.content.is_flex();

        let mut initial_phase_area = *node_area;
        let mut initial_phase_inner_area = *inner_area;
        let mut initial_phase_available_area = *available_area;
        let mut initial_phase_flex_grows = FxHashMap::default();
        let mut initial_phase_sizes = FxHashMap::default();
        let mut initial_phase_lines = vec![(0 as usize, Size2D::default())];
        let mut initial_phase_inner_sizes = Size2D::default();

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

                let inner_area = initial_phase_inner_area;

                let (_, mut child_areas) = self.measure_node(
                    *child_id,
                    &child_data,
                    &inner_area,
                    &initial_phase_available_area,
                    false,
                    node_is_dirty,
                    Phase::Initial,
                );

                child_areas.area.adjust_size(&child_data);
                let (line_len, line_size) = initial_phase_lines.last_mut().unwrap();

                // Stack this child into the parent
                let is_last_sibling_in_line = Self::stack_child(
                    node,
                    &child_data,
                    &mut initial_phase_available_area,
                    &mut initial_phase_area,
                    &mut initial_phase_inner_area,
                    &mut initial_phase_inner_sizes,
                    line_size,
                    &child_areas.area,
                    is_last_child,
                    Phase::Initial,
                );
                *line_len += 1;
                if is_last_sibling_in_line && !is_last_child {
                    initial_phase_lines.push((0, Size2D::default()));
                }

                if node.cross_alignment.is_not_start() || node.main_alignment.is_spaced() {
                    initial_phase_sizes.insert(*child_id, child_areas.area.size);
                }

                if node.content.is_flex() {
                    match node.direction {
                        Direction::Vertical => {
                            if let Some(ff) = child_data.height.flex_grow() {
                                initial_phase_flex_grows.insert(*child_id, ff);
                            }
                        }
                        Direction::Horizontal => {
                            if let Some(ff) = child_data.width.flex_grow() {
                                initial_phase_flex_grows.insert(*child_id, ff);
                            }
                        }
                    }
                }
            }
        }

        let initial_available_area = *available_area;

        let flex_grows = initial_phase_flex_grows
            .values()
            .copied()
            .reduce(|acc, v| acc + v)
            .unwrap_or_default()
            .max(Length::new(1.0));

        let flex_axis = AlignAxis::new(&node.direction, AlignmentDirection::Main);

        let flex_available_width = initial_available_area.width() - initial_phase_inner_sizes.width;
        let flex_available_height =
            initial_available_area.height() - initial_phase_inner_sizes.height;

        let initial_phase_inner_sizes_with_flex =
            initial_phase_flex_grows
                .values()
                .fold(initial_phase_inner_sizes, |mut acc, f| {
                    let flex_grow_per = f.get() / flex_grows.get() * 100.;

                    match flex_axis {
                        AlignAxis::Height => {
                            let size = flex_available_height / 100. * flex_grow_per;
                            acc.height += size;
                        }
                        AlignAxis::Width => {
                            let size = flex_available_width / 100. * flex_grow_per;
                            acc.width += size;
                        }
                    }

                    acc
                });

        if needs_initial_phase {
            if node.main_alignment.is_not_start() {
                // Adjust the available and inner areas of the Main axis
                Self::shrink_area_to_fit_when_unbounded(
                    available_area,
                    &initial_phase_area,
                    &mut initial_phase_inner_area,
                    node,
                    AlignmentDirection::Main,
                );
            }

            if node.cross_alignment.is_not_start() || node.content.is_fit() {
                // Adjust the available and inner areas of the Cross axis
                Self::shrink_area_to_fit_when_unbounded(
                    available_area,
                    &initial_phase_area,
                    &mut initial_phase_inner_area,
                    node,
                    AlignmentDirection::Cross,
                );
                // Align the Cross axis (all children)
                Self::align_content(
                    available_area,
                    &initial_phase_inner_area,
                    initial_phase_inner_sizes_with_flex,
                    &node.cross_alignment,
                    &node.direction,
                    AlignmentDirection::Cross,
                );
            }
        }

        let initial_available_area = *available_area;

        // Final phase: measure the children with all the axis and sizes adjusted
        let mut curr_line = 0;
        let mut line_index = 0;
        let mut line_origin = available_area.origin;
        let mut current_line_size = Size2D::zero();
        for child_id in children {
            let Some(child_data) = self.dom_adapter.get_node(&child_id) else {
                continue;
            };

            let is_last_child = last_child == Some(child_id);

            if node.content.is_flex() {
                let flex_grow = initial_phase_flex_grows.get(&child_id);

                if let Some(flex_grow) = flex_grow {
                    let flex_grow_per = flex_grow.get() / flex_grows.get() * 100.;

                    match flex_axis {
                        AlignAxis::Height => {
                            let size = flex_available_height / 100. * flex_grow_per;
                            available_area.size.height = size;
                        }
                        AlignAxis::Width => {
                            let size = flex_available_width / 100. * flex_grow_per;
                            available_area.size.width = size;
                        }
                    }
                }
            }

            // Only the stacked children will be aligned
            if node.main_alignment.is_spaced() && child_data.position.is_stacked() {
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

            // Align the Cross direction (child in line)
            if node.cross_alignment.is_not_start() {
                let initial_phase_size = initial_phase_sizes.get(&child_id);

                if let Some(initial_phase_size) = initial_phase_size {
                    if line_index == 0 {
                        Self::align_position(
                            available_area,
                            &initial_available_area,
                            initial_phase_inner_sizes_with_flex,
                            &node.cross_alignment,
                            &node.direction,
                            AlignmentDirection::Cross,
                            initial_phase_lines.len(),
                            curr_line == 0,
                        );
                    }
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
            if child_data.position.is_stacked() && line_index == 0 {
                let read_available_area = available_area.clone();
                Self::align_content(
                    available_area,
                    &read_available_area,
                    initial_phase_lines[curr_line].1,
                    &node.main_alignment,
                    &node.direction,
                    AlignmentDirection::Main,
                );
            }

            // Final measurement
            let (child_revalidated, mut child_areas) = self.measure_node(
                child_id,
                &child_data,
                inner_area,
                &available_area,
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
                    &mut current_line_size,
                    &child_areas.area,
                    is_last_child,
                    Phase::Final,
                );
                line_index += 1;
                if line_index == initial_phase_lines[curr_line].0 {
                    match node.direction {
                        Direction::Vertical => {
                            line_origin.x += initial_phase_lines[curr_line].1.width
                        }
                        Direction::Horizontal => {
                            line_origin.y += initial_phase_lines[curr_line].1.height
                        }
                    }
                    curr_line += 1;
                    line_index = 0;
                    current_line_size = Size2D::default();
                }
            }

            // Cache the child layout if it was mutated and children must be cached
            if child_revalidated && must_cache_children {
                // Finally cache this node areas into Torin
                self.layout.cache_node(child_id, child_areas);
            }
        }
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
    ///   this value is updated accordingly with the last sibling in each line.
    ///
    /// - `inner_area`: Kept in sync with `node_area` but excludes padding and margin. It reflects
    ///   the actual space available for child layout inside the parent.
    ///
    /// - `inner_sizes`: Accumulates the total width and height occupied by children.
    ///
    /// - `line_size`: Accumulates the width and height of children in the same line. A line is a row
    ///    or column, depending on the direction of the node. A wrapping node can have multiple lines.
    ///
    /// Returns:
    /// Whether the child is the last sibling in its line
    ///
    #[allow(clippy::too_many_arguments)]
    fn stack_child(
        node: &Node,
        child_node: &Node,
        available_area: &mut Area,
        node_area: &mut Area,
        inner_area: &mut Area,
        inner_sizes: &mut Size2D,
        line_size: &mut Size2D,
        child_area: &Area,
        is_last_sibling: bool,
        phase: Phase,
    ) -> bool {
        let is_last_sibling_in_line;
        match node.direction {
            Direction::Horizontal => {
                is_last_sibling_in_line = is_last_sibling
                    || (node.wrap_content.is_wrap()
                        && child_area.size.width * 2.0 + node.spacing.get()
                            > available_area.size.width);

                // Don't apply spacing to last child in the line
                let spacing = (!is_last_sibling_in_line)
                    .then_some(node.spacing)
                    .unwrap_or_default();

                // update size of current line
                line_size.height = line_size.height.max(child_area.height());
                line_size.width += spacing.get();
                // we only know child's correct flex sizing in the final phase
                if !child_node.width.is_flex() || phase == Phase::Final {
                    line_size.width += child_area.size.width;
                }

                // if last in line, update inner size
                if is_last_sibling_in_line {
                    inner_sizes.height += line_size.height;
                    inner_sizes.width = inner_sizes.width.max(line_size.width);

                    if node.height.inner_sized() {
                        node_area.size.height = node_area.size.height.max(
                            inner_sizes.height + node.padding.vertical() + node.margin.vertical(),
                        );
                        // Keep the inner area in sync
                        inner_area.size.height = node_area.size.height
                            - node.padding.vertical()
                            - node.margin.vertical();
                    }

                    if node.width.inner_sized() {
                        node_area.size.width =
                            inner_sizes.width + node.padding.horizontal() + node.margin.horizontal()
                    }

                    // move available area for next sibling
                    available_area.origin.x = inner_area.origin.x;
                    available_area.origin.y += line_size.height;
                    available_area.size.width = inner_area.size.width;
                } else {
                    // move available area for next sibling
                    available_area.origin.x = child_area.max_x() + spacing.get();
                    available_area.size.width -= child_area.size.width + spacing.get();
                }
            }
            Direction::Vertical => {
                is_last_sibling_in_line = is_last_sibling
                    || (node.wrap_content.is_wrap()
                        && child_area.size.height * 2.0 + node.spacing.get()
                            > available_area.size.height);

                // Don't apply spacing to last child in the line
                let spacing = (!is_last_sibling_in_line)
                    .then_some(node.spacing)
                    .unwrap_or_default();

                // update size of current line
                line_size.width = child_area.width().max(line_size.width);
                line_size.height += spacing.get();
                // we only know child's correct flex sizing in the final phase
                if !child_node.height.is_flex() || phase == Phase::Final {
                    line_size.height += child_area.size.height;
                }

                // if last in line, update inner size
                if is_last_sibling_in_line {
                    inner_sizes.width += line_size.width;
                    inner_sizes.height = inner_sizes.height.max(line_size.height);

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

                    if node.height.inner_sized() {
                        node_area.size.height =
                            inner_sizes.height + node.padding.vertical() + node.margin.vertical()
                    }
                }

                // Move the available area
                if is_last_sibling_in_line {
                    available_area.origin.y = inner_area.origin.y;
                    available_area.size.height = inner_area.size.height;
                } else {
                    available_area.origin.y = child_area.max_y() + spacing.get();
                    available_area.size.height -= child_area.size.height + spacing.get();
                }
            }
        }
        is_last_sibling_in_line
    }

    /// Shrink the available area and inner area of a parent node when for example height is set to "auto",
    /// direction is vertical and main_alignment is set to "center" or "end" or the content is set to "fit".
    /// The intended usage is to call this after the first measurement and before the second,
    /// this way the second measurement will align the content relatively to the parent element instead
    /// of overflowing due to being aligned relatively to the upper parent element
    fn shrink_area_to_fit_when_unbounded(
        available_area: &mut Area,
        node_area: &Area,
        inner_area: &mut Area,
        parent_node: &Node,
        alignment_direction: AlignmentDirection,
    ) {
        struct NodeData<'a> {
            pub inner_origin: &'a mut f32,
            pub inner_size: &'a mut f32,
            pub area_origin: f32,
            pub area_size: f32,
            pub one_side_padding: f32,
            pub two_sides_padding: f32,
            pub one_side_margin: f32,
            pub two_sides_margin: f32,
            pub available_size: &'a mut f32,
        }

        let axis = AlignAxis::new(&parent_node.direction, alignment_direction);
        let (is_vertical_not_start, is_horizontal_not_start) = match parent_node.direction {
            Direction::Vertical => (
                parent_node.main_alignment.is_not_start(),
                parent_node.cross_alignment.is_not_start() || parent_node.content.is_fit(),
            ),
            Direction::Horizontal => (
                parent_node.cross_alignment.is_not_start() || parent_node.content.is_fit(),
                parent_node.main_alignment.is_not_start(),
            ),
        };
        let NodeData {
            inner_origin,
            inner_size,
            area_origin,
            area_size,
            one_side_padding,
            two_sides_padding,
            one_side_margin,
            two_sides_margin,
            available_size,
        } = match axis {
            AlignAxis::Height if parent_node.height.inner_sized() && is_vertical_not_start => {
                NodeData {
                    inner_origin: &mut inner_area.origin.y,
                    inner_size: &mut inner_area.size.height,
                    area_origin: node_area.origin.y,
                    area_size: node_area.size.height,
                    one_side_padding: parent_node.padding.top(),
                    two_sides_padding: parent_node.padding.vertical(),
                    one_side_margin: parent_node.margin.top(),
                    two_sides_margin: parent_node.margin.vertical(),
                    available_size: &mut available_area.size.height,
                }
            }
            AlignAxis::Width if parent_node.width.inner_sized() && is_horizontal_not_start => {
                NodeData {
                    inner_origin: &mut inner_area.origin.x,
                    inner_size: &mut inner_area.size.width,
                    area_origin: node_area.origin.x,
                    area_size: node_area.size.width,
                    one_side_padding: parent_node.padding.left(),
                    two_sides_padding: parent_node.padding.horizontal(),
                    one_side_margin: parent_node.margin.left(),
                    two_sides_margin: parent_node.margin.horizontal(),
                    available_size: &mut available_area.size.width,
                }
            }
            _ => return,
        };

        // Set the origin of the inner area to the origin of the area plus the padding and margin for the given axis
        *inner_origin = area_origin + one_side_padding + one_side_margin;
        // Set the size of the inner area to the size of the area minus the padding and margin for the given axis
        *inner_size = area_size - two_sides_padding - two_sides_margin;
        // Set the same available size as the inner area for the given axis
        *available_size = *inner_size;
    }
}
