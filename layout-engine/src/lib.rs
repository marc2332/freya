use dioxus::core::ElementId;
use dioxus_native_core::real_dom::NodeType;
use layers_engine::{Layers, NodeData, Viewport};
use state::node::{DirectionMode, SizeMode};

fn calculate_viewport(
    node: &NodeData,
    mut viewport: Viewport,
    parent_viewport: Viewport,
) -> Viewport {
    match node.width {
        SizeMode::Manual(w) => {
            viewport.width = w;
        }
        SizeMode::Percentage(per) => {
            viewport.width = ((parent_viewport.width as f32) / 100.0 * (per as f32)).round() as i32;
        }
        SizeMode::Auto => {}
    }

    match node.height {
        SizeMode::Manual(h) => {
            viewport.height = h;
        }
        SizeMode::Percentage(per) => {
            viewport.height =
                ((parent_viewport.height as f32) / 100.0 * (per as f32)).round() as i32;
        }
        SizeMode::Auto => {
            if let Some(node) = &node.node {
                if let NodeType::Element { tag, .. } = &node.node_type {
                    if tag == "text" {
                        viewport.height = 5;
                    }
                }
            }
        }
    }

    viewport
}

pub fn calculate_node<T>(
    node: &NodeData,
    left_viewport: Viewport,
    parent_viewport: Viewport,
    resolver_options: &mut T,
    layers: &mut Layers,
    node_resolver: fn(&ElementId, &mut T) -> Option<NodeData>,
    layer_num: i16,
) -> Viewport {
    let mut node_viewport = calculate_viewport(node, left_viewport, parent_viewport);
    let mut is_text = false;

    let layer_num = layers.add_element(node, &node_viewport, &parent_viewport, layer_num);

    let padding = node.padding;
    let horizontal_padding = padding.1 + padding.3;
    let vertical_padding = padding.0 + padding.2;

    let mut inner_viewport = Viewport {
        x: node_viewport.x + padding.3,
        y: node_viewport.y + padding.0,
        width: node_viewport.width - horizontal_padding,
        height: node_viewport.height - vertical_padding,
    };
    let out_viewport = inner_viewport.clone();

    inner_viewport.y += node.node.as_ref().unwrap().state.size.scroll_y;
    inner_viewport.x += node.node.as_ref().unwrap().state.size.scroll_x;

    if let Some(dom_node) = &node.node {
        match &dom_node.node_type {
            NodeType::Element { children, .. } => {
                for child in children {
                    let child_node = node_resolver(child, resolver_options);

                    if let Some(child_node) = child_node {
                        let box_viewport = calculate_node::<T>(
                            &child_node,
                            inner_viewport,
                            out_viewport,
                            resolver_options,
                            layers,
                            node_resolver,
                            layer_num,
                        );

                        let state = &node.node.as_ref().unwrap().state;

                        if state.size.direction == DirectionMode::Vertical {
                            inner_viewport.y = box_viewport.y + box_viewport.height;
                            inner_viewport.height -= box_viewport.height;
                        } else {
                            inner_viewport.x = box_viewport.x + box_viewport.width;
                            inner_viewport.width -= box_viewport.width;
                        }

                        if box_viewport.width > inner_viewport.width || inner_viewport.width == 0 {
                            inner_viewport.width = box_viewport.width;
                        }
                    }
                }
            }
            NodeType::Text { .. } => {
                node_viewport.height += 10;
                is_text = true;
            }
            NodeType::Placeholder => {}
        }

        if !is_text {
            if let SizeMode::Auto = node.width {
                node_viewport.width = inner_viewport.x - node_viewport.x;
            }

            if let SizeMode::Auto = node.height {
                node_viewport.height = inner_viewport.y - node_viewport.y;
            }
        }
    }

    node_viewport
}
