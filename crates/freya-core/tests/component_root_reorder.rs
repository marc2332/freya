use freya::helpers::*;
use freya_core::{
    integration::*,
    prelude::*,
};

fn replay_component_root_reorder(
    component_key: u8,
    reordered_keys: [u8; 3],
    change_root_type: bool,
) {
    fn changing_root(&(reordered, change_root_type): &(bool, bool)) -> Element {
        let initial_value = use_state(|| reordered);
        assert!(!initial_value());
        if change_root_type {
            if reordered {
                label().into()
            } else {
                rect().into()
            }
        } else {
            rect().key(reordered).into()
        }
    }

    fn app() -> Element {
        let reordered = use_consume::<State<bool>>();
        let (component_key, reordered_keys, change_root_type) =
            consume_context::<(u8, [u8; 3], bool)>();
        let keys = if reordered() {
            reordered_keys
        } else {
            [0, 1, 2]
        };
        rect()
            .children(keys.into_iter().map(|key| {
                if key == component_key {
                    from_fn(key, (reordered(), change_root_type), changing_root)
                } else {
                    rect().key(key).into()
                }
            }))
            .into()
    }

    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    runner.provide_root_context(|| (component_key, reordered_keys, change_root_type));
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
        let keys = if next { reordered_keys } else { [0, 1, 2] };
        let component_index = keys.iter().position(|key| *key == component_key).unwrap();
        assert_eq!(
            runner.scopes[&ScopeId::ROOT]
                .borrow()
                .nodes
                .get(&[0, component_index as u32])
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
        if change_root_type {
            assert_eq!(root_node_id, old_children[component_key as usize]);
        }
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
    replay_component_root_reorder(1, [2, 1, 0], false);
}

#[test]
fn stationary_component_root_replacement_preserves_reordered_sibling_order() {
    replay_component_root_reorder(0, [0, 2, 1], false);
}

#[test]
fn moving_component_replaces_root_during_sibling_reorder() {
    replay_component_root_reorder(0, [2, 1, 0], false);
}

#[test]
fn moving_component_changes_root_type_during_sibling_reorder() {
    replay_component_root_reorder(0, [2, 1, 0], true);
}

#[test]
fn component_changes_root_type_without_sibling_reorder() {
    replay_component_root_reorder(0, [0, 1, 2], true);
}
