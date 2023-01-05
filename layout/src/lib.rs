use dioxus_native_core::{node::NodeType, NodeId};
use freya_common::{LayoutMemorizer, NodeArea, NodeLayoutInfo, NodeReferenceLayout};
use freya_layers::{Layers, DOMNode};
use freya_node_state::{
    CalcType, CursorMode, CursorReference, DirectionMode, DisplayMode, NodeState, SizeMode,
};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use std::sync::{Arc, Mutex};

pub fn run_calculations(calcs: &Vec<CalcType>, parent_area_value: f32) -> f32 {
    let mut prev_number: Option<f32> = None;
    let mut prev_op: Option<CalcType> = None;

    let mut calc_with_op = |val: f32, prev_op: Option<CalcType>| {
        if let Some(op) = prev_op {
            match op {
                CalcType::Sub => {
                    prev_number = Some(prev_number.unwrap() - val);
                }
                CalcType::Add => {
                    prev_number = Some(prev_number.unwrap() + val);
                }
                CalcType::Mul => {
                    prev_number = Some(prev_number.unwrap() * val);
                }
                CalcType::Div => {
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
            CalcType::Percentage(per) => {
                let val = (parent_area_value / 100.0 * per).round();

                calc_with_op(val, prev_op);

                prev_op = None;
            }
            CalcType::Manual(val) => {
                calc_with_op(*val, prev_op);
                prev_op = None;
            }
            _ => prev_op = Some(*calc),
        }
    }

    prev_number.unwrap()
}

/// Calculate the are of a node considering it's parent area
fn calculate_area(node_data: &DOMNode, mut area: NodeArea, parent_area: NodeArea) -> NodeArea {
    let calculate = |value: &SizeMode, area_value: f32, parent_area_value: f32| -> f32 {
        match value {
            &SizeMode::Manual(v) => v,
            SizeMode::Percentage(per) => (parent_area_value / 100.0 * per).round(),
            SizeMode::Auto => area_value,
            SizeMode::Calculation(calcs) => run_calculations(calcs, parent_area_value),
        }
    };

    let calculate_min = |value: &SizeMode, area_value: f32, parent_area_value: f32| -> f32 {
        match value {
            &SizeMode::Manual(v) => {
                if v > area_value {
                    v
                } else {
                    area_value
                }
            }
            SizeMode::Percentage(per) => {
                let by_per = (parent_area_value / 100.0 * per).round();
                if by_per > area_value {
                    by_per
                } else {
                    area_value
                }
            }
            SizeMode::Auto => area_value,
            SizeMode::Calculation(calcs) => {
                let by_calcs = run_calculations(calcs, parent_area_value);
                if by_calcs > area_value {
                    by_calcs
                } else {
                    area_value
                }
            }
        }
    };

    let calculate_max = |value: &SizeMode, area_value: f32, parent_area_value: f32| -> f32 {
        match value {
            &SizeMode::Manual(v) => {
                if v > area_value {
                    area_value
                } else {
                    v
                }
            }
            SizeMode::Percentage(per) => {
                let by_per = (parent_area_value / 100.0 * per).round();
                if by_per > area_value {
                    area_value
                } else {
                    by_per
                }
            }
            SizeMode::Auto => area_value,
            SizeMode::Calculation(calcs) => {
                let by_calcs = run_calculations(calcs, parent_area_value);
                if by_calcs > area_value {
                    area_value
                } else {
                    by_calcs
                }
            }
        }
    };

    area.width = calculate(
        &node_data.node.state.size.width,
        area.width,
        parent_area.width,
    );
    area.height = calculate(
        &node_data.node.state.size.height,
        area.height,
        parent_area.height,
    );

    if SizeMode::Auto == node_data.node.state.size.height {
        if let NodeType::Element { tag, .. } = &node_data.node.node_data.node_type {
            if tag == "label" {
                area.height = 18.0;
            }
        }
    }

    area.height = calculate_min(
        &node_data.node.state.size.min_height,
        area.height,
        parent_area.height,
    );
    area.width = calculate_min(
        &node_data.node.state.size.min_width,
        area.width,
        parent_area.width,
    );

    area.height = calculate_max(
        &node_data.node.state.size.max_height,
        area.height,
        parent_area.height,
    );
    area.width = calculate_max(
        &node_data.node.state.size.max_width,
        area.width,
        parent_area.width,
    );

    area
}

type NodeResolver<T> = fn(&NodeId, &mut T) -> Option<DOMNode>;

/// Measure the areas of a node's inner children
#[allow(clippy::too_many_arguments)]
fn measure_node_children<T>(
    node_data: &DOMNode,
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
    match &node_data.node.node_data.node_type {
        NodeType::Element { tag, .. } => {
            if let Some(children) = &node_data.children {
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

                        match node_data.node.state.size.direction {
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
                    && CursorMode::Editable == node_data.node.state.cursor_settings.mode
                {
                    let font_size = node_data.node.state.font_style.font_size;
                    let font_family = &node_data.node.state.font_style.font_family;
                    let align = node_data.node.state.font_style.align;
                    let max_lines = node_data.node.state.font_style.max_lines;
                    let font_style = node_data.node.state.font_style.font_style;

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

                    if let Some((cursor_ref, cursor_id, positions)) = get_cursor(node_data) {
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
            let line_height = node_data.node.state.font_style.line_height;
            let font_size = node_data.node.state.font_style.font_size;
            let font_family = &node_data.node.state.font_style.font_family;
            let align = node_data.node.state.font_style.align;
            let max_lines = node_data.node.state.font_style.max_lines;
            let font_style = node_data.node.state.font_style.font_style;

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

                if let NodeType::Text { text } =  &child_text.get_type() {
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
    let cursor_ref = node_data.node.state.references.cursor_ref.as_ref()?;
    let positions = { *cursor_ref.positions.lock().unwrap().as_ref()? };
    let current_cursor_id = { *cursor_ref.id.lock().unwrap().as_ref()? };
    let cursor_id = node_data.node.state.cursor_settings.id.as_ref()?;

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
            .mark_as_dirty(node_data.node.node_data.node_id);
    }

    let is_dirty = layout_memorizer
        .lock()
        .unwrap()
        .is_dirty(&node_data.node.node_data.node_id);
        
    let is_cached = layout_memorizer
        .lock()
        .unwrap()
        .is_node_layout_memorized(&node_data.node.node_data.node_id);

    // If this node is dirty and parent is not dirty, mark this node dirty
    if is_dirty && !is_parent_dirty {
        if let Some(p) = node_data.parent_id {
            layout_memorizer.lock().unwrap().mark_as_dirty(p)
        }
    }

    let padding = node_data.node.state.size.padding;
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
            remaining_inner_area.y += node_data.node.state.scroll.scroll_y;
            remaining_inner_area.x += node_data.node.state.scroll.scroll_x;

            if must_memorize_layout {
                // Memorize these layouts
                layout_memorizer.lock().unwrap().add_node_layout(
                    node_data.node.node_data.node_id,
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
                .get_node_layout(&node_data.node.node_data.node_id)
                .unwrap();
            (area, remaining_inner_area, inner_area, inner_sizes)
        };

    // Re calculate the children layouts after the parent has properly adjusted it's size and axis according to it's children
    if DisplayMode::Center == node_data.node.state.style.display {
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

        match node_data.node.state.size.direction {
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

    if must_recalculate && must_memorize_layout {
        layout_memorizer
            .lock()
            .unwrap()
            .remove_as_dirty(&node_data.node.node_data.node_id);
    }

    match &node_data.node.node_data.node_type {
        NodeType::Text { .. } => {}
        _ => {
            if let SizeMode::Auto = node_data.node.state.size.width {
                node_area.width = remaining_inner_area.x - node_area.x + padding.1;
            }
            if let SizeMode::Auto = node_data.node.state.size.height {
                node_area.height = remaining_inner_area.y - node_area.y + padding.0;
            }
        }
    }

    if must_memorize_layout {
        // Registers the element in the Layers handler
        layers.add_element(node_data, &node_area, node_layer);
    }

    // Asynchronously notify the Node's reference about the new size layout
    if let Some(reference) = &node_data.node.state.references.node_ref {
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
