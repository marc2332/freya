use core::{
    cmp::Eq,
    default::Default,
    option::{
        Option,
        Option::{
            None,
            Some,
        },
    },
};

pub use euclid::Rect;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use rustc_hash::FxHashMap as HashMap;

use crate::{
    custom_measurer::LayoutMeasurer,
    dom_adapter::{
        DOMAdapter,
        LayoutNode,
        NodeKey,
    },
    geometry::{
        Area,
        Size2D,
    },
    node::Node,
    prelude::{
        AlignAxis,
        Alignment,
        AlignmentDirection,
        AreaModel,
        DirectionMode,
        LayoutMetadata,
        Length,
        NodeData,
        Torin,
    },
};

/// Some layout strategies require two-phase measurements
/// Example: Alignments or content-fit.
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Initial,
    Final,
}

pub struct MeasureContext<'a, Key, Data, L, D>
where
    Key: NodeKey,
    Data: NodeData,
    L: LayoutMeasurer<Key, Data>,
    D: DOMAdapter<Key>,
{
    pub layout: &'a mut Torin<Key, Data>,
    pub measurer: &'a mut Option<L>,
    pub dom_adapter: &'a mut D,
    pub layout_metadata: LayoutMetadata,
}

impl<Key, Data, L, D> MeasureContext<'_, Key, Data, L, D>
where
    Key: NodeKey,
    Data: NodeData,
    L: LayoutMeasurer<Key, Data>,
    D: DOMAdapter<Key>,
{
    /// Measure a Node.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn measure_node(
        &mut self,
        // ID for this Node
        node_id: Key,
        // Data of this Node
        node: &Node,
        // Area occupied by it's parent
        parent_area: &Area,
        // Area that is available to use by the children of the parent
        available_parent_area: &Area,
        // Whether to cache the measurements of this Node's children
        must_cache_children: bool,
        // Parent Node is dirty.
        parent_is_dirty: bool,
        // Current phase of measurement
        phase: Phase,
    ) -> (bool, LayoutNode<Data>) {
        // 1. If parent is dirty
        // 2. If this Node has been marked as dirty
        // 3. If there is no know cached data about this Node.
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
            // When a Node is measured by a custom measurer function the inner children will be skipped
            let (measure_inner_children, node_data) = if let Some(measurer) = self.measurer {
                let most_fitting_width = *node
                    .width
                    .most_fitting_size(&area_size.width, &available_parent_area.size.width);
                let most_fitting_height = *node
                    .height
                    .most_fitting_size(&area_size.height, &available_parent_area.size.height);

                let most_fitting_area_size = Size2D::new(most_fitting_width, most_fitting_height);
                let res = measurer.measure(node_id, node, &most_fitting_area_size);

                // Compute the width and height again using the new custom area sizes
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
                    (false, Some(node_data))
                } else {
                    (true, None)
                }
            } else {
                (true, None)
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
                    must_cache_children,
                    &mut area,
                    &mut inner_area,
                    true,
                );
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
                    must_cache_children,
                    &mut area,
                    &mut inner_area,
                    false,
                );

                // In case of any layout listener, notify it with the new areas.
                if node.has_layout_references {
                    if let Some(measurer) = self.measurer {
                        measurer.notify_layout_references(node_id, layout_node.area, inner_sizes);
                    }
                }
            }

            (false, layout_node)
        }
    }

    /// Measure the children layouts of a Node
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn measure_children(
        &mut self,
        parent_node_id: &Key,
        parent_node: &Node,
        // Area available inside the Node
        available_area: &mut Area,
        // Accumulated sizes in both axis in the Node
        inner_sizes: &mut Size2D,
        // Whether to cache the measurements of this Node's children
        must_cache_children: bool,
        // Parent area.
        area: &mut Area,
        // Inner area of the parent.
        inner_area: &mut Area,
        // Parent Node is dirty.
        parent_is_dirty: bool,
    ) {
        let children = self.dom_adapter.children_of(parent_node_id);

        let mut initial_phase_flex_grows = HashMap::<Key, Length>::default();
        let mut initial_phase_sizes = HashMap::<Key, Size2D>::default();
        let mut initial_phase_inner_sizes = Size2D::default();

        // Used to calculate the spacing and some alignments
        let (non_absolute_children_len, first_child, last_child) = if parent_node.spacing.get() > 0.
        {
            let mut last_child = None;
            let mut first_child = None;
            let len = children
                .iter()
                .filter(|child_id| {
                    let Some(child_data) = self.dom_adapter.get_node(child_id) else {
                        return false;
                    };
                    let is_stacked = !child_data.position.is_absolute();
                    if is_stacked {
                        last_child = Some(**child_id);

                        if first_child.is_none() {
                            first_child = Some(**child_id)
                        }
                    }
                    is_stacked
                })
                .count();
            (len, first_child, last_child)
        } else {
            (
                children.len(),
                children.first().cloned(),
                children.last().cloned(),
            )
        };

        let needs_initial_phase = parent_node.cross_alignment.is_not_start()
            || parent_node.main_alignment.is_not_start()
            || parent_node.content.is_fit()
            || parent_node.content.is_flex();

        let mut initial_phase_area = *area;
        let mut initial_phase_inner_area = *inner_area;
        let mut initial_phase_available_area = *available_area;

        // Initial phase: Measure the size and position of the children if the parent has a
        // non-start cross alignment, non-start main aligment of a fit-content.
        if needs_initial_phase {
            //  Measure the children
            for child_id in children.iter() {
                let Some(child_data) = self.dom_adapter.get_node(child_id) else {
                    continue;
                };

                // No need to consider this Node for a two-phasing
                // measurements as it will float on its own.
                if child_data.position.is_absolute() {
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
                    parent_is_dirty,
                    Phase::Initial,
                );

                child_areas.area.adjust_size(&child_data);

                // Stack this child into the parent
                Self::stack_child(
                    &mut initial_phase_available_area,
                    parent_node,
                    &child_data,
                    &mut initial_phase_area,
                    &mut initial_phase_inner_area,
                    &mut initial_phase_inner_sizes,
                    &child_areas.area,
                    is_last_child,
                    Phase::Initial,
                );

                if parent_node.cross_alignment.is_not_start()
                    || parent_node.main_alignment.is_spaced()
                {
                    initial_phase_sizes.insert(*child_id, child_areas.area.size);
                }

                if parent_node.content.is_flex() {
                    match parent_node.direction {
                        DirectionMode::Vertical => {
                            if let Some(ff) = child_data.height.flex_grow() {
                                initial_phase_flex_grows.insert(*child_id, ff);
                            }
                        }
                        DirectionMode::Horizontal => {
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
            .cloned()
            .reduce(|acc, v| acc + v)
            .unwrap_or_default()
            .max(Length::new(1.0));

        let flex_axis = AlignAxis::new(&parent_node.direction, AlignmentDirection::Main);

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
            if parent_node.main_alignment.is_not_start() {
                // Adjust the available and inner areas of the Main axis
                Self::shrink_area_to_fit_when_unbounded(
                    available_area,
                    &initial_phase_area,
                    &mut initial_phase_inner_area,
                    parent_node,
                    AlignmentDirection::Main,
                );

                // Align the Main axis
                Self::align_content(
                    available_area,
                    &initial_phase_inner_area,
                    &initial_phase_inner_sizes_with_flex,
                    &parent_node.main_alignment,
                    &parent_node.direction,
                    AlignmentDirection::Main,
                );
            }

            if parent_node.cross_alignment.is_not_start() || parent_node.content.is_fit() {
                // Adjust the available and inner areas of the Cross axis
                Self::shrink_area_to_fit_when_unbounded(
                    available_area,
                    &initial_phase_area,
                    &mut initial_phase_inner_area,
                    parent_node,
                    AlignmentDirection::Cross,
                );
            }
        }

        let initial_available_area = *available_area;

        // Final phase: measure the children with all the axis and sizes adjusted
        for child_id in children {
            let Some(child_data) = self.dom_adapter.get_node(&child_id) else {
                continue;
            };

            let is_first_child = first_child == Some(child_id);
            let is_last_child = last_child == Some(child_id);

            let mut adapted_available_area = *available_area;

            if parent_node.content.is_flex() {
                let flex_grow = initial_phase_flex_grows.get(&child_id);

                if let Some(flex_grow) = flex_grow {
                    let flex_grow_per = flex_grow.get() / flex_grows.get() * 100.;

                    match flex_axis {
                        AlignAxis::Height => {
                            let size = flex_available_height / 100. * flex_grow_per;
                            adapted_available_area.size.height = size;
                        }
                        AlignAxis::Width => {
                            let size = flex_available_width / 100. * flex_grow_per;
                            adapted_available_area.size.width = size;
                        }
                    }
                }
            }

            // Only the stacked children will be aligned
            if parent_node.main_alignment.is_spaced() && !child_data.position.is_absolute() {
                // Align the Main axis if necessary
                Self::align_position(
                    AlignmentDirection::Main,
                    &mut adapted_available_area,
                    &initial_available_area,
                    &initial_phase_inner_sizes_with_flex,
                    &parent_node.main_alignment,
                    &parent_node.direction,
                    non_absolute_children_len,
                    is_first_child,
                );
            }

            if parent_node.cross_alignment.is_not_start() {
                let initial_phase_size = initial_phase_sizes.get(&child_id);

                if let Some(initial_phase_size) = initial_phase_size {
                    // Align the Cross axis if necessary
                    Self::align_content(
                        &mut adapted_available_area,
                        available_area,
                        initial_phase_size,
                        &parent_node.cross_alignment,
                        &parent_node.direction,
                        AlignmentDirection::Cross,
                    );
                }
            }

            // Final measurement
            let (child_revalidated, mut child_areas) = self.measure_node(
                child_id,
                &child_data,
                inner_area,
                &adapted_available_area,
                must_cache_children,
                parent_is_dirty,
                Phase::Final,
            );

            // Adjust the size of the area if needed
            child_areas.area.adjust_size(&child_data);

            // Stack this child into the parent
            if !child_data.position.is_absolute() {
                Self::stack_child(
                    available_area,
                    parent_node,
                    &child_data,
                    area,
                    inner_area,
                    inner_sizes,
                    &child_areas.area,
                    is_last_child,
                    Phase::Final,
                );
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
        contents_size: &Size2D,
        alignment: &Alignment,
        direction: &DirectionMode,
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
        alignment_direction: AlignmentDirection,
        available_area: &mut Area,
        initial_available_area: &Area,
        inner_sizes: &Size2D,
        alignment: &Alignment,
        direction: &DirectionMode,
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

    /// Stack a child Node into its parent
    #[allow(clippy::too_many_arguments)]
    fn stack_child(
        available_area: &mut Area,
        parent_node: &Node,
        child_node: &Node,
        parent_area: &mut Area,
        inner_area: &mut Area,
        inner_sizes: &mut Size2D,
        child_area: &Area,
        is_last_sibiling: bool,
        phase: Phase,
    ) {
        // Only apply the spacing to elements after `i > 0` and `i < len - 1`
        let spacing = (!is_last_sibiling)
            .then_some(parent_node.spacing)
            .unwrap_or_default();

        match parent_node.direction {
            DirectionMode::Horizontal => {
                // Move the available area
                available_area.origin.x = child_area.max_x() + spacing.get();
                available_area.size.width -= child_area.size.width + spacing.get();

                inner_sizes.height = child_area.height().max(inner_sizes.height);
                inner_sizes.width += spacing.get();
                if !child_node.width.is_flex() || phase == Phase::Final {
                    inner_sizes.width += child_area.width();
                }

                // Keep the biggest height
                if parent_node.height.inner_sized() {
                    parent_area.size.height = parent_area.size.height.max(
                        child_area.size.height
                            + parent_node.padding.vertical()
                            + parent_node.margin.vertical(),
                    );
                    // Keep the inner area in sync
                    inner_area.size.height = parent_area.size.height
                        - parent_node.padding.vertical()
                        - parent_node.margin.vertical();
                }

                // Accumulate width
                if parent_node.width.inner_sized() {
                    parent_area.size.width += child_area.size.width + spacing.get();
                }
            }
            DirectionMode::Vertical => {
                // Move the available area
                available_area.origin.y = child_area.max_y() + spacing.get();
                available_area.size.height -= child_area.size.height + spacing.get();

                inner_sizes.width = child_area.width().max(inner_sizes.width);
                inner_sizes.height += spacing.get();
                if !child_node.height.is_flex() || phase == Phase::Final {
                    inner_sizes.height += child_area.height();
                }

                // Keep the biggest width
                if parent_node.width.inner_sized() {
                    parent_area.size.width = parent_area.size.width.max(
                        child_area.size.width
                            + parent_node.padding.horizontal()
                            + parent_node.margin.horizontal(),
                    );
                    // Keep the inner area in sync
                    inner_area.size.width = parent_area.size.width
                        - parent_node.padding.horizontal()
                        - parent_node.margin.horizontal();
                }

                // Accumulate height
                if parent_node.height.inner_sized() {
                    parent_area.size.height += child_area.size.height + spacing.get();
                }
            }
        }
    }

    /// Shrink the available area and inner area of a parent node when for example height is set to "auto",
    /// direction is vertical and main_alignment is set to "center" or "end" or the content is set to "fit".
    /// The intended usage is to call this after the first measurement and before the second,
    /// this way the second measurement will align the content relatively to the parent element instead
    /// of overflowing due to being aligned relatively to the upper parent element
    fn shrink_area_to_fit_when_unbounded(
        available_area: &mut Area,
        parent_area: &Area,
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
            DirectionMode::Vertical => (
                parent_node.main_alignment.is_not_start(),
                parent_node.cross_alignment.is_not_start() || parent_node.content.is_fit(),
            ),
            DirectionMode::Horizontal => (
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
                    area_origin: parent_area.origin.y,
                    area_size: parent_area.size.height,
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
                    area_origin: parent_area.origin.x,
                    area_size: parent_area.size.width,
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
