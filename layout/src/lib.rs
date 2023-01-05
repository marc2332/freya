use dioxus_native_core::{node::NodeType, NodeId};
use freya_common::{LayoutMemorizer, NodeArea, NodeLayoutInfo, NodeReferenceLayout};
use freya_layers::{DOMNode, Layers};
use freya_node_state::{
    CursorMode, CursorReference, DirectionMode, DisplayMode, NodeState, SizeMode,
};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use std::sync::{Arc, Mutex};

mod area_calc;
mod ops_calc;

use area_calc::calculate_area;
pub use ops_calc::run_calculations;

type NodeResolver<T> = fn(&NodeId, &mut T) -> Option<DOMNode>;

/// Measure the areas of a node's inner children
#[allow(clippy::too_many_arguments)]
fn measure_node_children<T>(
    dom_node: &DOMNode,
    node_area: &mut NodeArea,
    layers: &mut Layers,
    remaining_inner_area: &mut NodeArea,
    inner_area: NodeArea,
    resolver_options: &mut T,
    node_resolver: NodeResolver<T>,
    inner_height: &mut f32,
    inner_width: &mut f32,
    inherited_relative_layer: i16,
    font_collection: &mut FontCollection,
    layout_memorizer: &Arc<Mutex<LayoutMemorizer>>,
    must_memorize_layout: bool,
) {
    match &dom_node.get_type() {
        NodeType::Element { tag, .. } => {
            if let Some(children) = &dom_node.get_children() {
                for child in children {
                    let child_node = node_resolver(child, resolver_options);

                    if let Some(child_node) = child_node {
                        let child_node_area = measure_node_layout::<T>(
                            &child_node,
                            *remaining_inner_area,
                            inner_area,
                            resolver_options,
                            layers,
                            node_resolver,
                            inherited_relative_layer,
                            font_collection,
                            layout_memorizer,
                            must_memorize_layout,
                        );

                        match dom_node.get_state().size.direction {
                            DirectionMode::Vertical => {
                                remaining_inner_area.height -= child_node_area.height;
                                remaining_inner_area.y = child_node_area.y + child_node_area.height;

                                // Accumulate all heights
                                *inner_height += child_node_area.height;

                                // Only save the biggest width
                                if *inner_width < child_node_area.width {
                                    *inner_width = child_node_area.width;
                                }
                            }
                            DirectionMode::Horizontal => {
                                remaining_inner_area.width -= child_node_area.width;
                                remaining_inner_area.x = child_node_area.x + child_node_area.width;

                                // Accumulate all widths
                                *inner_width += child_node_area.width;

                                // Only save the biggest height
                                if *inner_height < child_node_area.height {
                                    *inner_height = child_node_area.height;
                                }
                            }
                            DirectionMode::Both => {
                                remaining_inner_area.height -= child_node_area.height;
                                remaining_inner_area.width -= child_node_area.width;
                                remaining_inner_area.y = child_node_area.y + child_node_area.height;
                                remaining_inner_area.x = child_node_area.x + child_node_area.width;

                                // Accumulate all heights and widths
                                *inner_height += child_node_area.height;
                                *inner_width += child_node_area.width;
                            }
                        }

                        if child_node_area.width > remaining_inner_area.width
                            || remaining_inner_area.width == 0.0
                        {
                            remaining_inner_area.width = child_node_area.width;
                        }

                        if child_node_area.height > remaining_inner_area.height
                            || remaining_inner_area.height == 0.0
                        {
                            remaining_inner_area.height = child_node_area.height;
                        }
                    }
                }
                if tag == "paragraph"
                    && CursorMode::Editable == dom_node.get_state().cursor_settings.mode
                {
                    let font_size = dom_node.get_state().font_style.font_size;
                    let font_family = &dom_node.get_state().font_style.font_family;
                    let align = dom_node.get_state().font_style.align;
                    let max_lines = dom_node.get_state().font_style.max_lines;
                    let font_style = dom_node.get_state().font_style.font_style;

                    let mut paragraph_style = ParagraphStyle::default();
                    paragraph_style.set_text_align(align);
                    paragraph_style.set_max_lines(max_lines);
                    paragraph_style.set_replace_tab_characters(true);

                    let mut paragraph_builder =
                        ParagraphBuilder::new(&paragraph_style, font_collection.clone());

                    paragraph_builder.push_style(
                        TextStyle::new()
                            .set_font_style(font_style)
                            .set_font_size(font_size)
                            .set_font_families(&[font_family]),
                    );

                    let texts = get_inner_texts(children, node_resolver, resolver_options);

                    for node_text in texts {
                        paragraph_builder.push_style(
                            TextStyle::new()
                                .set_font_style(node_text.0.font_style.font_style)
                                .set_height_override(true)
                                .set_height(node_text.0.font_style.line_height)
                                .set_color(node_text.0.font_style.color)
                                .set_font_size(node_text.0.font_style.font_size)
                                .set_font_families(&[node_text.0.font_style.font_family.clone()]),
                        );
                        paragraph_builder.add_text(node_text.1.clone());
                    }

                    let mut paragraph = paragraph_builder.build();
                    paragraph.layout(node_area.width);

                    if let Some((cursor_ref, cursor_id, positions)) = get_cursor(dom_node) {
                        // Calculate the new cursor position
                        let char_position = paragraph.get_glyph_position_at_coordinate(positions);

                        // Notify the cursor reference listener
                        cursor_ref
                            .agent
                            .send((char_position.position as usize, cursor_id))
                            .ok();
                    }
                }
            }
        }
        NodeType::Text { text } => {
            let line_height = dom_node.get_state().font_style.line_height;
            let font_size = dom_node.get_state().font_style.font_size;
            let font_family = &dom_node.get_state().font_style.font_family;
            let align = dom_node.get_state().font_style.align;
            let max_lines = dom_node.get_state().font_style.max_lines;
            let font_style = dom_node.get_state().font_style.font_style;

            let mut paragraph_style = ParagraphStyle::default();
            paragraph_style.set_text_align(align);
            paragraph_style.set_max_lines(max_lines);
            paragraph_style.set_replace_tab_characters(true);

            let mut paragraph_builder =
                ParagraphBuilder::new(&paragraph_style, font_collection.clone());

            paragraph_builder.push_style(
                TextStyle::new()
                    .set_font_style(font_style)
                    .set_font_size(font_size)
                    .set_font_families(&[font_family]),
            );

            paragraph_builder.add_text(text);

            let mut paragraph = paragraph_builder.build();
            paragraph.layout(node_area.width);

            let lines_count = paragraph.line_number() as f32;
            node_area.width = paragraph.longest_line();
            node_area.height = (line_height * font_size) * lines_count;
        }
        NodeType::Placeholder => {}
    }
}

fn get_inner_texts<T>(
    children: &[NodeId],
    node_resolver: NodeResolver<T>,
    resolver_options: &mut T,
) -> Vec<(NodeState, String)> {
    children
        .iter()
        .filter_map(|child_id| {
            let child = node_resolver(child_id, resolver_options)?;
            if let NodeType::Element { tag, .. } = child.get_type() {
                if tag != "text" {
                    return None;
                }
                let children = child.children?;
                let child_text_id = children.get(0)?;
                let child_text = node_resolver(child_text_id, resolver_options)?;

                if let NodeType::Text { text } = &child_text.get_type() {
                    Some((child.node.state, text.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<(NodeState, String)>>()
}

/// Get the info related to a cursor refer
fn get_cursor(node_data: &DOMNode) -> Option<(&CursorReference, usize, (f32, f32))> {
    let cursor_ref = node_data.get_state().references.cursor_ref.as_ref()?;
    let positions = { *cursor_ref.positions.lock().unwrap().as_ref()? };
    let current_cursor_id = { *cursor_ref.id.lock().unwrap().as_ref()? };
    let cursor_id = node_data.get_state().cursor_settings.id.as_ref()?;

    if current_cursor_id == *cursor_id {
        Some((cursor_ref, *cursor_id, positions))
    } else {
        None
    }
}

/// Measure an area of a given Node
#[allow(clippy::too_many_arguments)]
pub fn measure_node_layout<T>(
    node_data: &DOMNode,
    remaining_area: NodeArea,
    parent_area: NodeArea,
    resolver_options: &mut T,
    layers: &mut Layers,
    node_resolver: NodeResolver<T>,
    inherited_relative_layer: i16,
    font_collection: &mut FontCollection,
    layout_memorizer: &Arc<Mutex<LayoutMemorizer>>,
    must_memorize_layout: bool,
) -> NodeArea {
    // Caculate the corresponding layer of this node
    let (node_layer, inherited_relative_layer) =
        layers.calculate_layer(node_data, inherited_relative_layer);

    let is_parent_dirty = node_data
        .parent_id
        .map(|p| layout_memorizer.lock().unwrap().is_dirty(&p))
        .unwrap_or(false);

    // If parent is dirty, mark this node as dirty too
    if is_parent_dirty {
        layout_memorizer
            .lock()
            .unwrap()
            .mark_as_dirty(*node_data.get_id());
    }

    let is_dirty = layout_memorizer
        .lock()
        .unwrap()
        .is_dirty(node_data.get_id());

    let is_cached = layout_memorizer
        .lock()
        .unwrap()
        .is_node_layout_memorized(node_data.get_id());

    // If this node is dirty and parent is not dirty, mark this node dirty
    if is_dirty && !is_parent_dirty {
        if let Some(p) = node_data.parent_id {
            layout_memorizer.lock().unwrap().mark_as_dirty(p)
        }
    }

    let padding = node_data.get_state().size.padding;
    let must_recalculate = is_dirty || !is_cached;

    let (mut node_area, mut remaining_inner_area, inner_area, (mut inner_width, mut inner_height)) =
        if must_recalculate {
            let node_area = calculate_area(node_data, remaining_area, parent_area);

            // Returns a tuple, the first element is the layer in which the current node must be added
            // and the second indicates the layer that it's children must inherit

            let horizontal_padding = padding.1 + padding.3;
            let vertical_padding = padding.0 + padding.2;

            // Area that is available consideing the parent area
            let mut remaining_inner_area = NodeArea {
                x: node_area.x + padding.3,
                y: node_area.y + padding.0,
                width: node_area.width - horizontal_padding,
                height: node_area.height - vertical_padding,
            };

            // Visible area occupied by the child elements
            let inner_area = remaining_inner_area;

            // Transform the x and y axis with the node's scroll attributes
            remaining_inner_area.y += node_data.get_state().scroll.scroll_y;
            remaining_inner_area.x += node_data.get_state().scroll.scroll_x;

            if must_memorize_layout {
                // Memorize these layouts
                layout_memorizer.lock().unwrap().add_node_layout(
                    *node_data.get_id(),
                    NodeLayoutInfo {
                        area: node_area,
                        remaining_inner_area,
                        inner_area,
                        inner_sizes: (horizontal_padding, vertical_padding),
                    },
                );
            }

            (
                node_area,
                remaining_inner_area,
                inner_area,
                (horizontal_padding, vertical_padding),
            )
        } else {
            // Get the memorized layouts
            let NodeLayoutInfo {
                area,
                remaining_inner_area,
                inner_area,
                inner_sizes,
            } = layout_memorizer
                .lock()
                .unwrap()
                .get_node_layout(node_data.get_id())
                .unwrap();
            (area, remaining_inner_area, inner_area, inner_sizes)
        };

    // Calculate the children layouts  for the first time without the size and axis adjusted.
    if DisplayMode::Center == node_data.get_state().style.display {
        measure_node_children(
            node_data,
            &mut node_area,
            layers,
            &mut remaining_inner_area,
            inner_area,
            resolver_options,
            node_resolver,
            &mut inner_height,
            &mut inner_width,
            inherited_relative_layer,
            font_collection,
            layout_memorizer,
            false, // By specifying `false` in the argument `must_memorize_layout` it will not cache any inner children layout in this first iteration
        );

        let space_left_vertically = (inner_area.height - inner_height) / 2.0;
        let space_left_horizontally = (inner_area.width - inner_width) / 2.0;

        match node_data.get_state().size.direction {
            DirectionMode::Vertical => {
                remaining_inner_area.y = inner_area.y + space_left_vertically + padding.0;
                remaining_inner_area.height = inner_area.height - space_left_vertically - padding.2;
            }
            DirectionMode::Horizontal => {
                remaining_inner_area.x = inner_area.x + space_left_horizontally + padding.1;
                remaining_inner_area.width = inner_area.width - space_left_horizontally - padding.3;
            }
            DirectionMode::Both => {
                remaining_inner_area.x = inner_area.x + space_left_horizontally + padding.1;
                remaining_inner_area.y = inner_area.y + space_left_vertically + padding.0;

                remaining_inner_area.width = inner_area.width - space_left_horizontally - padding.3;
                remaining_inner_area.height = inner_area.height - space_left_vertically - padding.2;
            }
        }
    }

    measure_node_children(
        node_data,
        &mut node_area,
        layers,
        &mut remaining_inner_area,
        inner_area,
        resolver_options,
        node_resolver,
        &mut inner_height,
        &mut inner_width,
        inherited_relative_layer,
        font_collection,
        layout_memorizer,
        must_memorize_layout,
    );

    // Unmark the node as dirty as it's layout has been calculted
    if must_recalculate && must_memorize_layout {
        layout_memorizer
            .lock()
            .unwrap()
            .remove_as_dirty(node_data.get_id());
    }

    match &node_data.get_type() {
        NodeType::Text { .. } => {}
        _ => {
            if let SizeMode::Auto = node_data.get_state().size.width {
                node_area.width = remaining_inner_area.x - node_area.x + padding.1;
            }
            if let SizeMode::Auto = node_data.get_state().size.height {
                node_area.height = remaining_inner_area.y - node_area.y + padding.0;
            }
        }
    }

    // Registers the element in the Layers handler
    if must_memorize_layout {
        layers.add_element(node_data, &node_area, node_layer);
    }

    // Notify the node's reference about the new size layout
    if let Some(reference) = &node_data.get_state().references.node_ref {
        reference
            .send(NodeReferenceLayout {
                x: node_area.x,
                y: node_area.y,
                width: node_area.width,
                height: node_area.height,
                inner_height,
                inner_width,
            })
            .ok();
    }

    node_area
}
