use tiny_diff::{
    Diff,
    DiffNode,
};

#[derive(Clone)]
struct Node {
    key: u8,
    path: Vec<u32>,
    children: Vec<Node>,
    kind: bool,
    value: usize,
}

impl DiffNode for Node {
    type Key = u8;
    type Modifications = (usize, usize);

    fn key(&self) -> &u8 {
        &self.key
    }

    fn path(&self) -> &[u32] {
        &self.path
    }

    fn children(&self) -> &[Self] {
        &self.children
    }

    fn compatible(&self, previous: &Self) -> bool {
        self.kind == previous.kind
    }

    fn modifications(&self, previous: &Self) -> Option<Self::Modifications> {
        (self.value != previous.value).then_some((previous.value, self.value))
    }
}

fn leaf(key: u8) -> Node {
    Node {
        key,
        path: Vec::new(),
        children: Vec::new(),
        kind: false,
        value: 0,
    }
}

fn branch(key: u8, children: Vec<Node>) -> Node {
    Node {
        children,
        ..leaf(key)
    }
}

fn assign_paths(node: &mut Node, path: Vec<u32>) {
    node.path = path;
    for (index, child) in node.children.iter_mut().enumerate() {
        let mut child_path = node.path.clone();
        child_path.push(index as u32);
        assign_paths(child, child_path);
    }
}

fn tree(keys: &[u8]) -> Node {
    let mut root = branch(255, keys.iter().copied().map(leaf).collect());
    assign_paths(&mut root, Vec::new());
    root
}

fn compare(mut current: Node, mut previous: Node) -> Diff<(usize, usize)> {
    assign_paths(&mut current, Vec::new());
    assign_paths(&mut previous, Vec::new());
    let mut changes = Diff::default();
    current.diff(Some(&previous), &mut changes);
    changes
}

fn paths(paths: &[Box<[u32]>]) -> Vec<Vec<u32>> {
    let mut paths: Vec<_> = paths.iter().map(|path| path.to_vec()).collect();
    paths.sort();
    paths
}

#[test]
fn unchanged_and_modified() {
    let previous = tree(&[1, 2]);
    let changes = compare(previous.clone(), previous.clone());
    assert!(changes.added.is_empty());
    assert!(changes.removed.is_empty());
    assert!(changes.moved.is_empty());
    assert!(changes.modified.is_empty());

    let mut current = previous.clone();
    current.children[1].value = 4;
    let changes = compare(current, previous);
    assert_eq!(changes.modified, vec![(Box::<[u32]>::from([1]), (0, 4))]);
}

#[test]
fn mounting_adds_descendants_in_preorder() {
    let mut current = branch(0, vec![branch(1, vec![leaf(2)]), leaf(3)]);
    assign_paths(&mut current, Vec::new());
    let mut changes = Diff::default();
    current.diff(None, &mut changes);
    assert_eq!(
        changes.added,
        vec![
            Box::<[u32]>::from([]),
            Box::<[u32]>::from([0]),
            Box::<[u32]>::from([0, 0]),
            Box::<[u32]>::from([1])
        ]
    );
    assert!(changes.removed.is_empty());
}

#[test]
fn removing_a_branch_only_removes_its_root() {
    let changes = compare(tree(&[]), branch(255, vec![branch(1, vec![leaf(2)])]));
    assert_eq!(paths(&changes.removed), vec![vec![0]]);
    assert!(changes.added.is_empty());
}

#[test]
fn incompatible_kind_replaces_the_entire_subtree() {
    let previous = branch(0, vec![leaf(1)]);
    let mut current = branch(0, vec![leaf(1), leaf(2)]);
    current.kind = true;
    let changes = compare(current, previous);
    assert_eq!(paths(&changes.removed), vec![Vec::<u32>::new()]);
    assert_eq!(paths(&changes.added), vec![vec![], vec![0], vec![1]]);
    assert!(changes.moved.is_empty());
    assert!(changes.modified.is_empty());
}

#[test]
fn replacing_parent_replaces_matching_descendants() {
    let previous = branch(0, vec![branch(1, vec![leaf(2)])]);
    let current = branch(3, vec![branch(1, vec![leaf(2)])]);
    let changes = compare(current, previous);
    assert_eq!(paths(&changes.removed), vec![vec![], vec![0], vec![0, 0]]);
    assert_eq!(paths(&changes.added), vec![vec![], vec![0], vec![0, 0]]);
    assert!(changes.moved.is_empty());
    assert!(changes.modified.is_empty());
}

#[test]
fn nested_moves_use_new_parent_paths_and_old_removal_paths() {
    let previous = branch(0, vec![branch(1, vec![leaf(3), leaf(4), leaf(5)]), leaf(2)]);
    let current = branch(0, vec![leaf(2), branch(1, vec![leaf(4), leaf(6), leaf(3)])]);
    let changes = compare(current, previous);
    assert_eq!(changes.moved.get(&[][..]), Some(&vec![(1, 0), (0, 1)]));
    assert_eq!(changes.moved.get(&[1][..]), Some(&vec![(1, 0), (0, 2)]));
    assert_eq!(paths(&changes.added), vec![vec![1, 1]]);
    assert_eq!(paths(&changes.removed), vec![vec![0, 2]]);
}

#[test]
fn moved_nodes_can_also_be_modified() {
    let previous = tree(&[1, 2]);
    let mut current = tree(&[2, 1]);
    current.children[0].value = 7;
    let changes = compare(current, previous);
    assert_eq!(changes.modified, vec![(Box::<[u32]>::from([0]), (0, 7))]);
    assert_eq!(changes.moved.get(&[][..]), Some(&vec![(1, 0), (0, 1)]));
}

#[test]
fn duplicate_fallback_handles_partial_and_exhausted_queues() {
    for new_keys in [&[2, 1][..], &[2, 1, 1, 1, 1][..]] {
        let mut previous = tree(&[1, 1, 1, 2, 3]);
        for (index, child) in previous.children.iter_mut().enumerate() {
            child.value = 10 + index;
        }
        let changes = compare(tree(new_keys), previous);
        assert_eq!(changes.modified[0], (Box::<[u32]>::from([0]), (13, 0)));
        assert_eq!(changes.modified[1], (Box::<[u32]>::from([1]), (10, 0)));
        if new_keys.len() == 2 {
            assert_eq!(paths(&changes.removed), vec![vec![1], vec![2], vec![4]]);
            assert!(changes.added.is_empty());
        } else {
            assert_eq!(changes.modified[2], (Box::<[u32]>::from([2]), (11, 0)));
            assert_eq!(changes.modified[3], (Box::<[u32]>::from([3]), (12, 0)));
            assert_eq!(paths(&changes.removed), vec![vec![4]]);
            assert_eq!(paths(&changes.added), vec![vec![4]]);
        }
    }
}

#[test]
fn diff_appends_movements_to_an_existing_parent_entry() {
    let previous = tree(&[1, 2]);
    let current = tree(&[2, 1]);
    let mut changes = Diff::default();
    changes.moved.insert(Box::from([]), vec![(10, 11)]);
    current.diff(Some(&previous), &mut changes);
    assert_eq!(
        changes.moved.get(&[][..]),
        Some(&vec![(10, 11), (1, 0), (0, 1)])
    );
}

#[test]
fn default_does_not_require_default_modifications() {
    struct NoDefault;
    let changes = Diff::<NoDefault>::default();
    assert!(changes.modified.is_empty());
}

fn sequences() -> Vec<Vec<u8>> {
    let mut sequences = vec![Vec::new()];
    let mut level = vec![Vec::new()];
    for _ in 0..4 {
        let mut next = Vec::new();
        for prefix in level {
            for key in 0..3 {
                let mut sequence = prefix.clone();
                sequence.push(key);
                next.push(sequence);
            }
        }
        sequences.extend(next.clone());
        level = next;
    }
    sequences
}

#[test]
fn exhaustive_sibling_matching_preserves_fifo_identity() {
    let sequences = sequences();
    for old_keys in &sequences {
        for new_keys in &sequences {
            let mut previous = tree(old_keys);
            let mut current = tree(new_keys);
            for (index, child) in previous.children.iter_mut().enumerate() {
                child.value = index;
            }
            for (index, child) in current.children.iter_mut().enumerate() {
                child.value = 100 + index;
            }
            let changes = compare(current, previous);
            let mut consumed = vec![false; old_keys.len()];
            let mut added = Vec::new();
            let mut modified = Vec::new();
            let mut moved = Vec::new();
            for (new_index, new_key) in new_keys.iter().enumerate() {
                let matched = old_keys
                    .iter()
                    .enumerate()
                    .find(|(old_index, old_key)| !consumed[*old_index] && *old_key == new_key);
                if let Some((old_index, _)) = matched {
                    consumed[old_index] = true;
                    modified.push((
                        Box::<[u32]>::from([new_index as u32]),
                        (old_index, 100 + new_index),
                    ));
                    if old_index != new_index {
                        moved.push((old_index as u32, new_index as u32));
                    }
                } else {
                    added.push(vec![new_index as u32]);
                }
            }
            let removed: Vec<_> = consumed
                .iter()
                .enumerate()
                .filter_map(|(index, consumed)| (!consumed).then_some(vec![index as u32]))
                .collect();
            assert_eq!(paths(&changes.added), added, "{old_keys:?} -> {new_keys:?}");
            assert_eq!(
                paths(&changes.removed),
                removed,
                "{old_keys:?} -> {new_keys:?}"
            );
            assert_eq!(changes.modified, modified, "{old_keys:?} -> {new_keys:?}");
            assert_eq!(
                changes
                    .moved
                    .get(&[][..])
                    .map(Vec::as_slice)
                    .unwrap_or_default(),
                moved
            );
            assert_eq!(changes.moved.len(), usize::from(!moved.is_empty()));
        }
    }
}
