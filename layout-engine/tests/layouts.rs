use dioxus_core::ElementId;
use dioxus_native_core::real_dom::{Node, NodeType};
use layers_engine::{Layers, NodeArea, NodeData};
use layout_engine::calculate_node;
use state::node::{SizeMode, NodeState};

#[test]
fn percentage() {
    let result = calculate_node(
        &NodeData {
            width: SizeMode::Percentage(100.0),
            height: SizeMode::Percentage(100.0),
            padding: (0.0, 0.0, 0.0, 0.0),
            node: Some(Node {
                id: ElementId(0),
                parent: None,
                state: NodeState::default(),
                node_type: NodeType::Element { tag: "rect".to_string(), namespace: None, children: Vec::new() },
                height: 0,
            }),
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| None,
        0,
    );

    assert_eq!(result.height, 300.0);
    assert_eq!(result.width, 200.0);
}

#[test]
fn manual() {
    let result = calculate_node(
        &NodeData {
            width: SizeMode::Manual(250.0),
            height: SizeMode::Manual(150.0),
            padding: (0.0, 0.0, 0.0, 0.0),
            node: Some(Node {
                id: ElementId(0),
                parent: None,
                state: NodeState::default(),
                node_type: NodeType::Element { tag: "rect".to_string(), namespace: None, children: Vec::new() },
                height: 0,
            }),
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| None,
        0,
    );

    assert_eq!(result.height, 150.0);
    assert_eq!(result.width, 250.0);
}

#[test]
fn auto() {
    let result = calculate_node(
        &NodeData {
            width: SizeMode::Percentage(100.0),
            height: SizeMode::Percentage(100.0),
            padding: (0.0, 0.0, 0.0, 0.0),
            node: Some(Node {
                id: ElementId(0),
                parent: None,
                state: NodeState::default(),
                node_type: NodeType::Element { tag: "rect".to_string(), namespace: None, children: Vec::new() },
                height: 0,
            }),
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| {
            Some(NodeData {
                width: SizeMode::Manual(170.0),
                height: SizeMode::Percentage(25.0),
                padding: (0.0, 0.0, 0.0, 0.0),
                node: None,
            })
        },
        0,
    );

    assert_eq!(result.height, 300.0);
    assert_eq!(result.width, 200.0);
}

#[test]
fn x_y() {
    let result = calculate_node(
        &NodeData {
            width: SizeMode::Auto,
            height: SizeMode::Auto,
            padding: (0.0, 0.0, 0.0, 0.0),
            node: Some(Node {
                id: ElementId(0),
                parent: None,
                state: NodeState::default(),
                node_type: NodeType::Element { tag: "rect".to_string(), namespace: None, children: Vec::new() },
                height: 0,
            }),
        },
        NodeArea {
            x: 15.0,
            y: 25.0,
            height: 300.0,
            width: 200.0,
        },
        NodeArea {
            x: 15.0,
            y: 25.0,
            height: 300.0,
            width: 200.0,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| {
            Some(NodeData {
                width: SizeMode::Manual(170.0),
                height: SizeMode::Percentage(25.0),
                padding: (0.0, 0.0, 0.0, 0.0),
                node: None,
            })
        },
        0,
    );

    assert_eq!(result.x, 15.0);
    assert_eq!(result.y, 25.0);
}
