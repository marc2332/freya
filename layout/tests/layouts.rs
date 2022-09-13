use dioxus_core::ElementId;
use dioxus_native_core::real_dom::{Node, NodeType};
use freya_layers::{Layers, NodeArea, NodeData};
use freya_layout::calculate_node;
use freya_node_state::node::{DirectionMode, NodeState, Size, SizeMode};
use lazy_static::lazy_static;

lazy_static! {
    static ref TEST_NODE: Node<NodeState> = Node {
        id: ElementId(0),
        parent: None,
        state: NodeState::default(),
        node_type: NodeType::Element {
            tag: "rect".to_string(),
            namespace: None,
            children: Vec::new()
        },
        height: 0,
    };
}

#[test]
fn percentage() {
    let result = calculate_node(
        &NodeData {
            size: Size {
                width: SizeMode::Percentage(50.0),
                height: SizeMode::Percentage(25.0),
                ..Size::expanded()
            },
            node: TEST_NODE.clone(),
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

    assert_eq!(result.height, 75.0);
    assert_eq!(result.width, 100.0);
}

#[test]
fn manual() {
    let result = calculate_node(
        &NodeData {
            size: Size {
                width: SizeMode::Manual(250.0),
                height: SizeMode::Manual(150.0),
                ..Size::expanded()
            },
            node: TEST_NODE.clone(),
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
            size: Size {
                width: SizeMode::Auto,
                height: SizeMode::Auto,
                direction: DirectionMode::Both,
                ..Size::expanded()
            },
            node: Node {
                id: ElementId(0),
                parent: None,
                state: NodeState::default(),
                node_type: NodeType::Element {
                    tag: "rect".to_string(),
                    namespace: None,
                    children: vec![ElementId(1)],
                },
                height: 0,
            },
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
                size: Size {
                    width: SizeMode::Manual(170.0),
                    height: SizeMode::Manual(25.0),
                    ..Size::expanded()
                },
                node: Node {
                    id: ElementId(1),
                    parent: None,
                    state: NodeState::default(),
                    node_type: NodeType::Element {
                        tag: "rect".to_string(),
                        namespace: None,
                        children: Vec::new(),
                    },
                    height: 0,
                },
            })
        },
        0,
    );

    assert_eq!(result.height, 25.0);
    assert_eq!(result.width, 170.0);
}

#[test]
fn x_y() {
    let result = calculate_node(
        &NodeData {
            size: Size {
                width: SizeMode::Auto,
                height: SizeMode::Auto,
                ..Size::expanded()
            },
            node: TEST_NODE.clone(),
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
                size: Size {
                    width: SizeMode::Manual(170.0),
                    height: SizeMode::Manual(25.0),
                    ..Size::expanded()
                },
                node: Node {
                    id: ElementId(1),
                    parent: None,
                    state: NodeState::default(),
                    node_type: NodeType::Element {
                        tag: "rect".to_string(),
                        namespace: None,
                        children: Vec::new(),
                    },
                    height: 0,
                },
            })
        },
        0,
    );

    assert_eq!(result.x, 15.0);
    assert_eq!(result.y, 25.0);
}
