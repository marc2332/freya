use dioxus_native_core::state::State;
use dioxus_native_core_macro::State;

use layout_engine::{calculate_node, NodeData, Viewport};
use state::node::SizeMode;

#[derive(Clone, Default, Debug, State)]
struct DummyState {}

#[test]
fn percentage() {
    let result = calculate_node(
        &NodeData::<DummyState> {
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
        |_, _| None,
        |_, _, _| {},
    );

    assert_eq!(result.height, 300);
    assert_eq!(result.width, 200);
}

#[test]
fn manual() {
    let result = calculate_node(
        &NodeData::<DummyState> {
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
        |_, _| None,
        |_, _, _| {},
    );

    assert_eq!(result.height, 150);
    assert_eq!(result.width, 250);
}

#[test]
fn auto() {
    let result = calculate_node(
        &NodeData::<DummyState> {
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
        |_, _| {
            Some(NodeData {
                width: SizeMode::Manual(170),
                height: SizeMode::Percentage(25),
                padding: (0, 0, 0, 0),
                node: None,
            })
        },
        |_, _, _| {},
    );

    assert_eq!(result.height, 300);
    assert_eq!(result.width, 200);
}

#[test]
fn x_y() {
    let result = calculate_node(
        &NodeData::<DummyState> {
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
        |_, _| {
            Some(NodeData {
                width: SizeMode::Manual(170),
                height: SizeMode::Percentage(25),
                padding: (0, 0, 0, 0),
                node: None,
            })
        },
        |_, _, _| {},
    );

    assert_eq!(result.x, 15);
    assert_eq!(result.y, 25);
}
