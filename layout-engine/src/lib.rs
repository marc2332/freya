use dioxus::core::ElementId;
use dioxus_native_core::real_dom::{Node, NodeType};
use state::node::{NodeState, SizeMode};

#[derive(Default, Clone, Debug)]
pub struct NodeData {
    pub width: SizeMode,
    pub height: SizeMode,
    pub padding: (i32, i32, i32, i32),
    pub node: Option<Node<NodeState>>,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

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
            viewport.width = ((parent_viewport.width as f32) / 100.0 * (per as f32)) as i32;
        }
        SizeMode::Auto => {}
    }

    match node.height {
        SizeMode::Manual(h) => {
            viewport.height = h;
        }
        SizeMode::Percentage(per) => {
            viewport.height = ((parent_viewport.height as f32) / 100.0 * (per as f32)) as i32;
        }
        SizeMode::Auto => {
            if let Some(node) = &node.node {
                if let NodeType::Element { tag, .. } = &node.node_type {
                    if tag == "p" {
                        viewport.y += 10;
                        viewport.height = 5;
                    }
                }
            }
        }
    }

    let x = {
        if viewport.x < parent_viewport.x {
            parent_viewport.x
        } else {
            viewport.x
        }
    };
    let y = {
        if viewport.y < parent_viewport.y {
            parent_viewport.y
        } else {
            viewport.y
        }
    };

    let height = {
        if viewport.y < parent_viewport.y {
            viewport.height - (parent_viewport.y - viewport.y)
        } else {
            viewport.height
        }
    };

    Viewport {
        x,
        y,
        height,
        ..viewport
    }
}

pub fn calculate_node<T>(
    node: &NodeData,
    left_viewport: Viewport,
    parent_viewport: Viewport,
    render_options: &mut T,
    node_resolver: fn(&ElementId, &mut T) -> Option<NodeData>,
    render_hook: fn(&NodeData, &Viewport, &Viewport, &mut T),
) -> Viewport {
    let mut node_viewport = calculate_viewport(node, left_viewport, parent_viewport);
    let mut is_text = false;

    render_hook(node, &node_viewport, &parent_viewport, render_options);

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

    inner_viewport.y += node.node.as_ref().unwrap().state.size.overflow;

    if let Some(dom_node) = &node.node {
        match &dom_node.node_type {
            NodeType::Element { children, .. } => {
                for child in children {
                    let child_node = node_resolver(child, render_options);

                    if let Some(child_node) = child_node {
                        let box_viewport = calculate_node::<T>(
                            &child_node,
                            inner_viewport,
                            out_viewport,
                            render_options,
                            node_resolver,
                            render_hook,
                        );

                        inner_viewport.y = box_viewport.y + box_viewport.height;
                        inner_viewport.height -= box_viewport.height;

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
