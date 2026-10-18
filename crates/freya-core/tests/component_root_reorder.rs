use freya::helpers::*;
use freya_core::{
    integration::*,
    prelude::*,
};

fn replay_component_root_reorder(component_key: u8, reordered_keys: [u8; 3]) {
    fn changing_root(reordered: &bool) -> Element {
        let initial_value = use_state(|| *reordered);
        assert!(!initial_value());
        rect().key(*reordered).into()
    }

    fn app() -> Element {
        let reordered = use_consume::<State<bool>>();
        let (component_key, reordered_keys) = consume_context::<(u8, [u8; 3])>();
        let keys = if reordered() {
            reordered_keys
        } else {
            [0, 1, 2]
        };
        rect()
            .children(keys.into_iter().map(|key| {
                if key == component_key {
                    from_fn(key, reordered(), changing_root)
                } else {
                    rect().key(key).into()
                }
            }))
            .into()
    }

    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    runner.provide_root_context(|| (component_key, reordered_keys));
    let mut reordered = runner.provide_root_context(|| State::create(false));
    tree.apply_mutations(runner.sync_and_update(), 1.0);
    let parent_node_id = runner.scopes[&ScopeId::ROOT]
        .borrow()
        .nodes
        .get(&[0])
        .unwrap()
        .node_id;
    let component_scope_id = runner.scopes[&ScopeId::ROOT]
        .borrow()
        .nodes
        .get(&[0, component_key as u32])
        .unwrap()
        .scope_id
        .unwrap();
    let old_children = tree.children[&parent_node_id].clone();

    for next in [true, false, true] {
        reordered.set(next);
        tree.apply_mutations(runner.sync_and_update(), 1.0);
        tree.verify_tree_integrity();
        assert_eq!(tree.elements.len(), runner.node_to_scope.len());
        assert_eq!(
            runner.scopes[&ScopeId::ROOT]
                .borrow()
                .nodes
                .get(&[0, component_key as u32])
                .unwrap()
                .scope_id,
            Some(component_scope_id),
        );

        let root_node_id = runner.scopes[&component_scope_id]
            .borrow()
            .nodes
            .get(&[0])
            .unwrap()
            .node_id;
        let keys = if next { reordered_keys } else { [0, 1, 2] };
        let expected = keys.map(|key| {
            if key == component_key {
                root_node_id
            } else {
                old_children[key as usize]
            }
        });
        assert_eq!(tree.children[&parent_node_id], expected);
    }
}

#[test]
fn unchanged_middle_component_replaces_root_during_sibling_reorder() {
    replay_component_root_reorder(1, [2, 1, 0]);
}

#[test]
fn stationary_component_root_replacement_preserves_reordered_sibling_order() {
    replay_component_root_reorder(0, [0, 2, 1]);
}
