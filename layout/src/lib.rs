use dioxus::core::ElementId;
use dioxus_native_core::real_dom::NodeType;
use freya_elements::NodeLayout;
use freya_layers::{Layers, NodeArea, NodeData};
use freya_node_state::node::{CalcType, DirectionMode, SizeMode};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};

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
            _ => prev_op = Some(calc.clone()),
        }
    }

    prev_number.unwrap()
}

fn calculate_area(node_data: &NodeData, mut area: NodeArea, parent_area: NodeArea) -> NodeArea {
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
        if let NodeType::Element { tag, .. } = &node_data.node.node_type {
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

    area
}

pub fn calculate_node<T>(
    node_data: &NodeData,
    remaining_area: NodeArea,
    parent_area: NodeArea,
    resolver_options: &mut T,
    layers: &mut Layers,
    node_resolver: fn(&ElementId, &mut T) -> Option<NodeData>,
    inherited_relative_layer: i16,
    font_collection: &mut FontCollection,
) -> NodeArea {
    let mut node_area = calculate_area(node_data, remaining_area, parent_area);

    // Returns a tuple, the first element is the layer in which the current node must be added
    // and the second indicates the layer that it's children must inherit
    let (node_layer, inherited_relative_layer) =
        layers.calculate_layer(node_data, inherited_relative_layer);

    let padding = node_data.node.state.size.padding;
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
    let inner_area = remaining_inner_area.clone();

    remaining_inner_area.y += node_data.node.state.size.scroll_y;
    remaining_inner_area.x += node_data.node.state.size.scroll_x;

    let mut inner_height = vertical_padding;
    let mut inner_width = vertical_padding;

    match &node_data.node.node_type {
        NodeType::Element { children, .. } => {
            for child in children {
                let child_node = node_resolver(child, resolver_options);

                if let Some(child_node) = child_node {
                    let child_node_area = calculate_node::<T>(
                        &child_node,
                        remaining_inner_area,
                        inner_area,
                        resolver_options,
                        layers,
                        node_resolver,
                        inherited_relative_layer,
                        font_collection,
                    );

                    match node_data.node.state.size.direction {
                        DirectionMode::Vertical => {
                            remaining_inner_area.height -= child_node_area.height;
                            remaining_inner_area.y = child_node_area.y + child_node_area.height;

                            // Accumulate all heights
                            inner_height += child_node_area.height;

                            // Only save the biggest width
                            if inner_width < child_node_area.width {
                                inner_width = child_node_area.width;
                            }
                        }
                        DirectionMode::Horizontal => {
                            remaining_inner_area.width -= child_node_area.width;
                            remaining_inner_area.x = child_node_area.x + child_node_area.width;

                            // Accumulate all widths
                            inner_width += child_node_area.width;

                            // Only save the biggest height
                            if inner_height < child_node_area.height {
                                inner_height = child_node_area.height;
                            }
                        }
                        DirectionMode::Both => {
                            remaining_inner_area.height -= child_node_area.height;
                            remaining_inner_area.width -= child_node_area.width;
                            remaining_inner_area.y = child_node_area.y + child_node_area.height;
                            remaining_inner_area.x = child_node_area.x + child_node_area.width;

                            // Accumulate all heights and widths
                            inner_height += child_node_area.height;
                            inner_width += child_node_area.width;
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

            if let SizeMode::Auto = node_data.node.state.size.width {
                node_area.width = remaining_inner_area.x - node_area.x + padding.1;
            }

            if let SizeMode::Auto = node_data.node.state.size.height {
                node_area.height = remaining_inner_area.y - node_area.y + padding.0;
            }
        }
        NodeType::Text { text } => {
            let line_height = node_data.node.state.font_style.line_height;
            let font_size = node_data.node.state.font_style.font_size;
            let font_family = &node_data.node.state.font_style.font_family;

            let paragraph_style = ParagraphStyle::default();
            let mut paragraph_builder =
                ParagraphBuilder::new(&paragraph_style, font_collection.clone());

            paragraph_builder.push_style(
                TextStyle::new()
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

    // Registers the element in the Layers handler
    layers.add_element(node_data, &node_area, node_layer);

    // Asynchronously notify the Node's reference about the new size layout
    if let Some(reference) = &node_data.node.state.references.node_ref {
        reference
            .send(NodeLayout {
                x: node_area.x,
                y: node_area.y,
                width: node_area.width,
                height: node_area.height,
                inner_height: inner_height,
                inner_width: inner_width,
            })
            .ok();
    }

    node_area
}
