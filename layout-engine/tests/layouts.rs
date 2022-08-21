use layers_engine::{Layers, NodeData, Viewport};
use layout_engine::calculate_node;
use state::node::SizeMode;

#[test]
fn percentage() {
    let result = calculate_node(
        &NodeData {
            width: SizeMode::Percentage(100),
            height: SizeMode::Percentage(100),
            padding: (0, 0, 0, 0),
            node: None,
        },
        Viewport {
            x: 0,
            y: 0,
            height: 300,
            width: 200,
        },
        Viewport {
            x: 0,
            y: 0,
            height: 300,
            width: 200,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| None,
        0,
    );

    assert_eq!(result.height, 300);
    assert_eq!(result.width, 200);
}

#[test]
fn manual() {
    let result = calculate_node(
        &NodeData {
            width: SizeMode::Manual(250),
            height: SizeMode::Manual(150),
            padding: (0, 0, 0, 0),
            node: None,
        },
        Viewport {
            x: 0,
            y: 0,
            height: 300,
            width: 200,
        },
        Viewport {
            x: 0,
            y: 0,
            height: 300,
            width: 200,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| None,
        0,
    );

    assert_eq!(result.height, 150);
    assert_eq!(result.width, 250);
}

#[test]
fn auto() {
    let result = calculate_node(
        &NodeData {
            width: SizeMode::Auto,
            height: SizeMode::Auto,
            padding: (0, 0, 0, 0),
            node: None,
        },
        Viewport {
            x: 0,
            y: 0,
            height: 300,
            width: 200,
        },
        Viewport {
            x: 0,
            y: 0,
            height: 300,
            width: 200,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| {
            Some(NodeData {
                width: SizeMode::Manual(170),
                height: SizeMode::Percentage(25),
                padding: (0, 0, 0, 0),
                node: None,
            })
        },
        0,
    );

    assert_eq!(result.height, 300);
    assert_eq!(result.width, 200);
}

#[test]
fn x_y() {
    let result = calculate_node(
        &NodeData {
            width: SizeMode::Auto,
            height: SizeMode::Auto,
            padding: (0, 0, 0, 0),
            node: None,
        },
        Viewport {
            x: 15,
            y: 25,
            height: 300,
            width: 200,
        },
        Viewport {
            x: 15,
            y: 25,
            height: 300,
            width: 200,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| {
            Some(NodeData {
                width: SizeMode::Manual(170),
                height: SizeMode::Percentage(25),
                padding: (0, 0, 0, 0),
                node: None,
            })
        },
        0,
    );

    assert_eq!(result.x, 15);
    assert_eq!(result.y, 25);
}
