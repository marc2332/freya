pub use euclid::Rect;

use crate::{
    custom_measurer::LayoutMeasurer,
    dom_adapter::{DOMAdapter, NodeAreas, NodeKey},
    geometry::{Area, Size2D},
    measure_mode::MeasureMode,
    node::Node,
    prelude::{AlignmentDirection, AreaModel, LayoutMetadata, Torin},
    size::Size,
};

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
) -> (bool, NodeAreas) {
    let must_revalidate = invalidated_tree
        || layout.dirty.contains(&node_id)
        || !layout.results.contains_key(&node_id);
    if must_revalidate {
        // 1. Create the initial Node area size
        let mut area_size = Size2D::new(node.padding.horizontal(), node.padding.vertical());

        // 2. Compute the width and height given the size, the minimum size, the maximum size and margins
        area_size.width = node.width.min_max(
            area_size.width,
            parent_area.size.width,
            available_parent_area.size.width,
            node.margin.left(),
            node.margin.horizontal(),
            &node.minimum_width,
            &node.maximum_width,
            layout_metadata.root_area.width(),
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
        );

        // 3. If available, run a custom layout measure function
        // This is useful when you use third-party libraries (e.g. rust-skia, cosmic-text) to measure text layouts
        // When a Node is measured by a custom measurer function the inner children will be skipped
        let measure_inner_children = if let Some(measurer) = measurer {
            let custom_size = measurer.measure(node_id, node, parent_area, available_parent_area);

            // 3.1. Compute the width and height again using the new custom area sizes
            if let Some(custom_size) = custom_size {
                if Size::Inner == node.width {
                    area_size.width = node.width.min_max(
                        custom_size.width,
                        parent_area.size.width,
                        available_parent_area.size.width,
                        node.margin.left(),
                        node.margin.horizontal(),
                        &node.minimum_width,
                        &node.maximum_width,
                        layout_metadata.root_area.width(),
                    );
                }
                if Size::Inner == node.height {
                    area_size.height = node.height.min_max(
                        custom_size.height,
                        parent_area.size.height,
                        available_parent_area.size.height,
                        node.margin.top(),
                        node.margin.vertical(),
                        &node.minimum_height,
                        &node.maximum_height,
                        layout_metadata.root_area.height(),
                    );
                }
            }

            // Do not measure inner children
            custom_size.is_none()
        } else {
            true
        };

        // 4. Compute the inner size of the Node, which is basically the size inside the margins and paddings
        let inner_size = {
            let mut inner_size = area_size;

            // 4.1. When having an unsized bound we set it to whatever is still available in the parent's area
            if Size::Inner == node.width {
                inner_size.width = node.width.min_max(
                    available_parent_area.width(),
                    parent_area.size.width,
                    available_parent_area.width(),
                    node.margin.left(),
                    node.margin.horizontal(),
                    &node.minimum_width,
                    &node.maximum_width,
                    layout_metadata.root_area.width(),
                );
            }
            if Size::Inner == node.height {
                inner_size.height = node.height.min_max(
                    available_parent_area.height(),
                    parent_area.size.height,
                    available_parent_area.height(),
                    node.margin.top(),
                    node.margin.vertical(),
                    &node.minimum_height,
                    &node.maximum_height,
                    layout_metadata.root_area.height(),
                );
            }
            inner_size
        };

        // 5. Create the areas
        let area_origin = node
            .position
            .get_origin(available_parent_area, parent_area, &area_size);
        let mut area = Rect::new(area_origin, area_size);
        let mut inner_area = Rect::new(area_origin, inner_size)
            .after_gaps(&node.padding)
            .after_gaps(&node.margin);

        let mut inner_sizes = Size2D::default();

        if measure_inner_children {
            // 6. Create an area containing the available space inside the inner area
            let mut available_area = inner_area;

            // 6.1. Adjust the available area with the node offsets (mainly used by scrollviews)
            available_area.move_with_offsets(&node.offset_x, &node.offset_y);

            let mut measurement_mode = MeasureMode::ParentIsNotCached {
                area: &mut area,
                inner_area: &mut inner_area,
            };

            // 7. Measure the layout of this Node's children
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

        available_area.move_with_offsets(&node.offset_x, &node.offset_y);

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
            must_cache_inner_nodes,
            &mut measurement_mode,
            dom_adapter,
            layout_metadata,
            false,
        );

        (false, areas)
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
    let mut measure_children = |mode: &mut MeasureMode,
                                available_area: &mut Area,
                                inner_sizes: &mut Size2D,
                                must_cache_inner_nodes: bool| {
        let children = dom_adapter.children_of(parent_node_id);

        for child_id in children {
            let inner_area = *mode.inner_area();

            let child_data = dom_adapter.get_node(&child_id).unwrap();

            let mut adapted_available_area = *available_area;

            if parent_node.cross_alignment.is_not_start() {
                // 1. First measure: Cross axis is not aligned
                let (_, child_areas) = measure_node(
                    child_id,
                    &child_data,
                    layout,
                    &inner_area,
                    available_area,
                    measurer,
                    false,
                    dom_adapter,
                    layout_metadata,
                    invalidated_tree,
                );

                // 2. Align the Cross axis
                adapted_available_area.align_content(
                    available_area,
                    &child_areas.area.size,
                    &parent_node.cross_alignment,
                    &parent_node.direction,
                    AlignmentDirection::Cross,
                );
            }

            // 3. Second measure
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
                layout.cache_node(child_id, child_areas);
            }
        }
    };

    {
        // This is no the final measure, hence we make a temporary measurement mode
        // so the affected values are not reused by the final measurement
        let mut alignment_mode = mode.to_owned();
        let mut alignment_mode = alignment_mode.to_mut();
        let mut inner_sizes = *inner_sizes;

        if parent_node.main_alignment.is_not_start() || parent_node.cross_alignment.is_not_start() {
            // 1. First measure: Main axis is not aligned
            measure_children(
                &mut alignment_mode,
                &mut available_area.clone(),
                &mut inner_sizes,
                false,
            );
        }

        if parent_node.cross_alignment.is_not_start() {
            // 2. Adjust the available and inner areas of the Cross axis
            alignment_mode.fit_bounds_when_unspecified_and_aligned(
                parent_node,
                AlignmentDirection::Cross,
                available_area,
            );
        }

        if parent_node.main_alignment.is_not_start() {
            // 3. Adjust the available and inner areas of the Main axis
            alignment_mode.fit_bounds_when_unspecified_and_aligned(
                parent_node,
                AlignmentDirection::Main,
                available_area,
            );

            // 4. Align the Main axis
            available_area.align_content(
                alignment_mode.inner_area(),
                &inner_sizes,
                &parent_node.main_alignment,
                &parent_node.direction,
                AlignmentDirection::Main,
            );
        }
    }

    // 5. Second measure
    measure_children(mode, available_area, inner_sizes, must_cache_inner_nodes);
}
