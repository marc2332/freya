use freya::prelude::*;
use freya_core::{
    layers::Layers,
    node_id::NodeId,
};
use freya_testing::prelude::*;
use rustc_hash::FxHashMap;

#[test]
fn insertions_and_moved_subtrees_update_only_their_layer_entries() {
    let root = NodeId::ROOT;
    let first = NodeId::from(2);
    let second = NodeId::from(3);
    let first_child = NodeId::from(4);
    let second_child = NodeId::from(5);
    let inserted = NodeId::from(6);
    let parents = FxHashMap::from_iter([
        (first, root),
        (second, root),
        (inserted, root),
        (first_child, first),
        (second_child, second),
    ]);
    let mut children = FxHashMap::from_iter([
        (root, vec![first, second]),
        (first, vec![first_child]),
        (second, vec![second_child]),
    ]);
    let mut layers = Layers::default();

    layers.update_order(
        &FxHashMap::from_iter([
            (root, 0),
            (first, 1),
            (second, 1),
            (first_child, 2),
            (second_child, 2),
        ]),
        &parents,
        &children,
    );
    assert_eq!(layers[&1], [first, second]);
    assert_eq!(layers[&2], [first_child, second_child]);

    children.insert(root, vec![inserted, first, second]);
    layers.update_order(&FxHashMap::from_iter([(inserted, 1)]), &parents, &children);
    assert_eq!(layers[&1], [inserted, first, second]);

    children.insert(root, vec![second, inserted, first]);
    layers.update_order(
        &FxHashMap::from_iter([(second, 1), (second_child, 2)]),
        &parents,
        &children,
    );
    assert_eq!(layers[&0], [root]);
    assert_eq!(layers[&1], [second, inserted, first]);
    assert_eq!(layers[&2], [second_child, first_child]);

    layers.remove_node_from_layer(&inserted, 1);
    assert_eq!(layers[&1], [second, first]);
}

#[test]
fn painting_follows_keyed_sibling_order() {
    fn app() -> impl IntoElement {
        let mut step = use_state(|| 0usize);
        let children: Vec<usize> = match step() {
            0 => vec![0, 1],
            1 => vec![2, 0, 1],
            _ => vec![1, 2, 0],
        };

        rect()
            .expanded()
            .on_mouse_up(move |_| *step.write() += 1)
            .children(children.into_iter().map(|index| {
                rect()
                    .key(index)
                    .position(Position::new_global().top(20.).left(20.))
                    .width(Size::px(100.))
                    .height(Size::px(100.))
                    .background(match index {
                        0 => Color::RED,
                        1 => Color::BLUE,
                        _ => Color::GREEN,
                    })
            }))
    }

    let mut runner = launch_test(app);
    runner.sync_and_update();
    let initial = runner.render();

    runner.click_cursor((250., 250.));
    assert_eq!(initial.as_bytes(), runner.render().as_bytes());

    runner.click_cursor((250., 250.));
    assert_ne!(initial.as_bytes(), runner.render().as_bytes());
}
