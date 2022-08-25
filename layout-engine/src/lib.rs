use dioxus::core::ElementId;
use dioxus_native_core::real_dom::NodeType;
use layers_engine::{Layers, NodeArea, NodeData};
use state::node::{DirectionMode, SizeMode};

fn calculate_area(node: &NodeData, mut area: NodeArea, parent_area: NodeArea) -> NodeArea {
    match node.width {
        SizeMode::Manual(w) => {
            area.width = w;
        }
        SizeMode::Percentage(per) => {
            area.width = ((parent_area.width as f32) / 100.0 * (per as f32)).round() as i32;
        }
        SizeMode::Auto => {}
    }

    match node.height {
        SizeMode::Manual(h) => {
            area.height = h;
        }
        SizeMode::Percentage(per) => {
            area.height = ((parent_area.height as f32) / 100.0 * (per as f32)).round() as i32;
        }
        SizeMode::Auto => {
            if let Some(node) = &node.node {
                if let NodeType::Element { tag, .. } = &node.node_type {
                    if tag == "text" {
                        area.height = 5;
                    }
                }
            }
        }
    }

    area
}

pub fn calculate_node<T>(
    node: &NodeData,
    left_area: NodeArea,
    parent_area: NodeArea,
    resolver_options: &mut T,
    layers: &mut Layers,
    node_resolver: fn(&ElementId, &mut T) -> Option<NodeData>,
    layer_num: i16,
) -> NodeArea {
    let mut node_area = calculate_area(node, left_area, parent_area);
    let mut is_text = false;

    let layer_num = layers.add_element(node, &node_area, &parent_area, layer_num);

    let padding = node.padding;
    let horizontal_padding = padding.1 + padding.3;
    let vertical_padding = padding.0 + padding.2;

    let mut inner_area = NodeArea {
        x: node_area.x + padding.3,
        y: node_area.y + padding.0,
        width: node_area.width - horizontal_padding,
        height: node_area.height - vertical_padding,
    };
    let out_area = inner_area.clone();

    inner_area.y += node.node.as_ref().unwrap().state.size.scroll_y;
    inner_area.x += node.node.as_ref().unwrap().state.size.scroll_x;

    if let Some(dom_node) = &node.node {
        match &dom_node.node_type {
            NodeType::Element { children, .. } => {
                for child in children {
                    let child_node = node_resolver(child, resolver_options);

                    if let Some(child_node) = child_node {
                        let box_area = calculate_node::<T>(
                            &child_node,
                            inner_area,
                            out_area,
                            resolver_options,
                            layers,
                            node_resolver,
                            layer_num,
                        );

                        let state = &node.node.as_ref().unwrap().state;

                        if state.size.direction == DirectionMode::Vertical {
                            inner_area.y = box_area.y + box_area.height;
                            inner_area.height -= box_area.height;
                        } else {
                            inner_area.x = box_area.x + box_area.width;
                            inner_area.width -= box_area.width;
                        }

                        if box_area.width > inner_area.width || inner_area.width == 0 {
                            inner_area.width = box_area.width;
                        }
                    }
                }
            }
            NodeType::Text { .. } => {
                node_area.height += 10;
                is_text = true;
            }
            NodeType::Placeholder => {}
        }

        if !is_text {
            if let SizeMode::Auto = node.width {
                node_area.width = inner_area.x - node_area.x;
            }

            if let SizeMode::Auto = node.height {
                node_area.height = inner_area.y - node_area.y;
            }
        }
    }

    node_area
}
