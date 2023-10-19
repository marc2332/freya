pub use euclid::Rect;

use crate::{
    custom_measurer::LayoutMeasurer,
    dom_adapter::{DOMAdapter, NodeAreas, NodeKey},
    geometry::{Area, Size2D},
    measure_mode::MeasureMode,
    node::Node,
    prelude::{AlignmentDirection, AreaModel, Torin},
    size::Size,
};

/// Measure this node and all it's children
/// The caller of this function is responsible of caching the Node's layout results
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn measure_node<Key: NodeKey>(
    node_id: Key,
    node: &Node,
    layout: &mut Torin<Key>,
    parent_area: &Area,
    available_parent_area: &Area,
    measurer: &mut Option<impl LayoutMeasurer<Key>>,
    must_cache: bool,
    dom_adapter: &mut impl DOMAdapter<Key>,
) -> (bool, NodeAreas) {
    let must_run = layout.dirty.contains(&node_id) || layout.results.get(&node_id).is_none();
    if must_run {
        let horizontal_padding = node.padding.horizontal();
        let vertical_padding = node.padding.vertical();

        let mut area = Rect::new(
            available_parent_area.origin,
            Size2D::new(horizontal_padding, vertical_padding),
        );

        area.origin.x += node.margin.left();
        area.origin.y += node.margin.top();

        area.size.width = node.width.min_max(
            area.size.width,
            parent_area.size.width,
            node.margin.left(),
            node.margin.horizontal(),
            &node.minimum_width,
            &node.maximum_width,
        );
        area.size.height = node.height.min_max(
            area.size.height,
            parent_area.size.height,
            node.margin.top(),
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
                        node.margin.left(),
                        node.margin.horizontal(),
                        &node.minimum_width,
                        &node.maximum_width,
                    );
                }
                if Size::Inner == node.height {
                    area.size.height = node.height.min_max(
                        new_area.height(),
                        parent_area.size.height,
                        node.margin.top(),
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

/// Measure the inner Nodes of a Node
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn measure_inner_nodes<Key: NodeKey>(
    node_id: &Key,
    node: &Node,
    layout: &mut Torin<Key>,
    available_area: &mut Area,
    inner_sizes: &mut Size2D,
    measurer: &mut Option<impl LayoutMeasurer<Key>>,
    must_cache: bool,
    mode: &mut MeasureMode,
    dom_adapter: &mut impl DOMAdapter<Key>,
) {
    let mut measure_children = |mode: &mut MeasureMode,
                                available_area: &mut Area,
                                inner_sizes: &mut Size2D,
                                must_cache: bool| {
        let children = dom_adapter.children_of(node_id);

        for child_id in children {
            let inner_area = *mode.inner_area();

            let child_data = dom_adapter.get_node(&child_id).unwrap();

            let mut adapted_available_area = *available_area;

            if node.cross_alignment.is_not_start() {
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
                );

                // 2. Align the Cross axis
                adapted_available_area.align_content(
                    available_area,
                    &child_areas.area.size,
                    &node.cross_alignment,
                    &node.direction,
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
                must_cache,
                dom_adapter,
            );

            // Stack the child node
            mode.stack_node(node, available_area, &child_areas.area, inner_sizes);

            if child_revalidated && must_cache {
                layout.cache_node(child_id, child_areas);
            }
        }
    };

    {
        let mut alignment_mode = mode.to_owned();
        let mut alignment_mode = alignment_mode.to_mut();
        let mut inner_sizes = *inner_sizes;

        if node.main_alignment.is_not_start() || node.cross_alignment.is_not_start() {
            // 1. First measure: Main axis is not aligned
            measure_children(
                &mut alignment_mode,
                &mut available_area.clone(),
                &mut inner_sizes,
                false,
            );
        }

        if node.cross_alignment.is_not_start() {
            // 2. Adjust the available and inner areas of the Cross axis
            alignment_mode.fit_bounds_when_unspecified_and_aligned(
                node,
                AlignmentDirection::Cross,
                available_area,
            );
        }

        if node.main_alignment.is_not_start() {
            // 3. Adjust the available and inner areas of the Main axis
            alignment_mode.fit_bounds_when_unspecified_and_aligned(
                node,
                AlignmentDirection::Main,
                available_area,
            );

            // 4. Align the Main axis
            available_area.align_content(
                alignment_mode.inner_area(),
                &inner_sizes,
                &node.main_alignment,
                &node.direction,
                AlignmentDirection::Main,
            );
        }
    }

    // 5. Second measure
    measure_children(mode, available_area, inner_sizes, must_cache);
}
