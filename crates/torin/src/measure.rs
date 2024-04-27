pub use euclid::Rect;
use rustc_hash::FxHashMap;

use crate::{
    custom_measurer::LayoutMeasurer,
    dom_adapter::{DOMAdapter, LayoutNode, NodeKey},
    geometry::{Area, Size2D},
    measure_mode::MeasureMode,
    node::Node,
    prelude::{AlignmentDirection, AreaModel, LayoutMetadata, Torin},
};

/// Some layout strategies require two-phase measurements
/// Example: Alignments or content-fit.
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Initial,
    Final,
}

/// Measure a Node layout
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn measure_node<Key: NodeKey>(
    node_id: Key,
    node: &Node,
    layout: &mut Torin<Key>,
    // Area occupied by it's parent
    parent_area: &Area,
    // Area that is available to use by the children of the parent
    available_parent_area: &Area,
    measurer: &mut Option<impl LayoutMeasurer<Key>>,
    // Whether to cache the measurements of this Node's children
    must_cache_inner_nodes: bool,
    // Adapter for the provided DOM
    dom_adapter: &mut impl DOMAdapter<Key>,

    layout_metadata: &LayoutMetadata,

    invalidated_tree: bool,

    phase: Phase,
) -> (bool, LayoutNode) {
    let must_revalidate = invalidated_tree
        || layout.dirty.contains(&node_id)
        || !layout.results.contains_key(&node_id);
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
            layout_metadata.root_area.width(),
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
            layout_metadata.root_area.height(),
            phase,
        );

        // If available, run a custom layout measure function
        // This is useful when you use third-party libraries (e.g. rust-skia, cosmic-text) to measure text layouts
        // When a Node is measured by a custom measurer function the inner children will be skipped
        let (measure_inner_children, node_data) = if let Some(measurer) = measurer {
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
                        layout_metadata.root_area.width(),
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
                        layout_metadata.root_area.height(),
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
                    layout_metadata.root_area.width(),
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
                    layout_metadata.root_area.height(),
                    phase,
                );
            }
            inner_size
        };

        // Create the areas
        let area_origin = node
            .position
            .get_origin(available_parent_area, parent_area, &area_size);
        let mut area = Rect::new(area_origin, area_size);
        let mut inner_area = Rect::new(area_origin, inner_size)
            .after_gaps(&node.padding)
            .after_gaps(&node.margin);

        let mut inner_sizes = Size2D::default();

        if measure_inner_children && phase_measure_inner_children {
            // Create an area containing the available space inside the inner area
            let mut available_area = inner_area;

            // Adjust the available area with the node offsets (mainly used by scrollviews)
            available_area.move_with_offsets(&node.offset_x, &node.offset_y);

            let mut measurement_mode = MeasureMode::ParentIsNotCached {
                area: &mut area,
                inner_area: &mut inner_area,
            };

            // Measure the layout of this Node's children
            measure_inner_nodes(
                &node_id,
                node,
                layout,
                &mut available_area,
                &mut inner_sizes,
                measurer,
                must_cache_inner_nodes,
                &mut measurement_mode,
                dom_adapter,
                layout_metadata,
                true,
            );
        }

        (
            must_cache_inner_nodes,
            LayoutNode {
                area,
                margin: node.margin,
                inner_area,
                inner_sizes,
                data: node_data,
            },
        )
    } else {
        let layout_node = layout.get(node_id).unwrap().clone();

        let mut inner_sizes = layout_node.inner_sizes;
        let mut available_area = layout_node.inner_area;

        available_area.move_with_offsets(&node.offset_x, &node.offset_y);

        let mut measurement_mode = MeasureMode::ParentIsCached {
            inner_area: &layout_node.inner_area,
        };

        let measure_inner_children = if let Some(measurer) = measurer {
            measurer.should_measure_inner_children(node_id)
        } else {
            true
        };

        if measure_inner_children {
            measure_inner_nodes(
                &node_id,
                node,
                layout,
                &mut available_area,
                &mut inner_sizes,
                measurer,
                must_cache_inner_nodes,
                &mut measurement_mode,
                dom_adapter,
                layout_metadata,
                false,
            );
        }

        (false, layout_node)
    }
}

/// Measure the children layouts of a Node
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn measure_inner_nodes<Key: NodeKey>(
    parent_node_id: &Key,
    parent_node: &Node,
    layout: &mut Torin<Key>,
    // Area available inside the Node
    available_area: &mut Area,
    // Accumulated sizes in both axis in the Node
    inner_sizes: &mut Size2D,
    measurer: &mut Option<impl LayoutMeasurer<Key>>,
    // Whether to cache the measurements of this Node's children
    must_cache_inner_nodes: bool,
    mode: &mut MeasureMode,
    // Adapter for the provided DOM
    dom_adapter: &mut impl DOMAdapter<Key>,

    layout_metadata: &LayoutMetadata,

    invalidated_tree: bool,
) {
    let children = dom_adapter.children_of(parent_node_id);

    let mut initial_phase_sizes = FxHashMap::default();

    // Initial phase: Measure the size and position of the children if the parent has a
    // non-start cross alignment, non-start main aligment of a fit-content.
    if parent_node.cross_alignment.is_not_start()
        || parent_node.main_alignment.is_not_start()
        || parent_node.content.is_fit()
    {
        let mut initial_phase_mode = mode.to_owned();
        let mut initial_phase_mode = initial_phase_mode.to_mut();
        let mut initial_phase_inner_sizes = *inner_sizes;
        let mut initial_phase_available_area = *available_area;

        // 1. Measure the children
        for child_id in &children {
            let Some(child_data) = dom_adapter.get_node(child_id) else {
                continue;
            };

            if child_data.position.is_absolute() {
                continue;
            }

            let inner_area = *initial_phase_mode.inner_area();

            let (_, child_areas) = measure_node(
                *child_id,
                &child_data,
                layout,
                &inner_area,
                &initial_phase_available_area,
                measurer,
                false,
                dom_adapter,
                layout_metadata,
                invalidated_tree,
                Phase::Initial,
            );

            initial_phase_mode.stack_into_node(
                parent_node,
                &mut initial_phase_available_area,
                &child_areas.area,
                &mut initial_phase_inner_sizes,
                &child_data,
            );

            if parent_node.cross_alignment.is_not_start() {
                initial_phase_sizes.insert(*child_id, child_areas.area.size);
            }
        }

        if parent_node.main_alignment.is_not_start() {
            // 2. Adjust the available and inner areas of the Main axis
            initial_phase_mode.fit_bounds_when_unspecified(
                parent_node,
                AlignmentDirection::Main,
                available_area,
            );

            // 3. Align the Main axis
            available_area.align_content(
                initial_phase_mode.inner_area(),
                &initial_phase_inner_sizes,
                &parent_node.main_alignment,
                &parent_node.direction,
                AlignmentDirection::Main,
            );
        }

        if parent_node.cross_alignment.is_not_start() || parent_node.content.is_fit() {
            // 4. Adjust the available and inner areas of the Cross axis
            initial_phase_mode.fit_bounds_when_unspecified(
                parent_node,
                AlignmentDirection::Cross,
                available_area,
            );
        }
    }

    // Final phase: measure the children with all the axis and sizes adjusted
    for child_id in children {
        let Some(child_data) = dom_adapter.get_node(&child_id) else {
            continue;
        };

        let mut adapted_available_area = *available_area;
        if parent_node.cross_alignment.is_not_start() {
            let initial_phase_size = initial_phase_sizes.get(&child_id);

            if let Some(initial_phase_size) = initial_phase_size {
                // 1. Align the Cross axis if necessary
                adapted_available_area.align_content(
                    available_area,
                    initial_phase_size,
                    &parent_node.cross_alignment,
                    &parent_node.direction,
                    AlignmentDirection::Cross,
                );
            }
        }

        let inner_area = *mode.inner_area();

        // Final measurement
        let (child_revalidated, child_areas) = measure_node(
            child_id,
            &child_data,
            layout,
            &inner_area,
            &adapted_available_area,
            measurer,
            must_cache_inner_nodes,
            dom_adapter,
            layout_metadata,
            invalidated_tree,
            Phase::Final,
        );

        // Stack the child into its parent
        mode.stack_into_node(
            parent_node,
            available_area,
            &child_areas.area,
            inner_sizes,
            &child_data,
        );

        // Cache the child layout if it was mutated and inner nodes must be cache
        if child_revalidated && must_cache_inner_nodes {
            if let Some(measurer) = measurer {
                if child_data.has_layout_references {
                    measurer.notify_layout_references(child_id, &child_areas);
                }
            }
            layout.cache_node(child_id, child_areas);
        }
    }
}
