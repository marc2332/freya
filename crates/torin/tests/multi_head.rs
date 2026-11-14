use std::{
    any::Any,
    rc::Rc,
};

use torin::{
    prelude::*,
    test_utils::TestingTree,
};

#[derive(Default)]
struct VisitMeasurer {
    visited: Vec<usize>,
}

impl LayoutMeasurer<usize> for VisitMeasurer {
    fn measure(
        &mut self,
        _node_id: usize,
        _node: &Node,
        _size: &Size2D,
    ) -> Option<(Size2D, Rc<dyn Any>)> {
        None
    }

    fn should_hook_measurement(&mut self, _node_id: usize) -> bool {
        false
    }

    fn should_measure_inner_children(&mut self, node_id: usize) -> bool {
        self.visited.push(node_id);
        true
    }
}

fn fixed_node(width: f32, height: f32) -> Node {
    Node::from_size_and_direction(Size::px(width), Size::px(height), Direction::Vertical)
}

fn panel_tree() -> TestingTree {
    let mut tree = TestingTree::default();
    tree.add(0, None, vec![1, 4, 7], fixed_node(400., 400.));
    for panel in [1, 4, 7] {
        tree.add(
            panel,
            Some(0),
            vec![panel + 1, panel + 2],
            Node {
                padding: Gaps::new_all(5.),
                margin: Gaps::new_all(3.),
                ..fixed_node(100., 90.)
            },
        );
        tree.add(panel + 1, Some(panel), vec![], fixed_node(20., 20.));
        tree.add(panel + 2, Some(panel), vec![], fixed_node(30., 20.));
    }
    tree
}

fn viewport() -> Area {
    Area::new(Point2D::new(17., 23.), Size2D::new(500., 500.))
}

fn assert_matches_fresh(layout: &Torin<usize>, tree: &TestingTree, area: Area) {
    let mut fresh = Torin::new();
    fresh.measure(0, area, &mut None::<NoopMeasurer>, tree);
    assert_eq!(layout.results.len(), fresh.results.len());
    for (node_id, expected) in &fresh.results {
        let actual = layout.get(node_id).unwrap();
        assert_eq!(actual, expected, "Layout differs for node {node_id}");
        assert_eq!(
            actual.inner_sizes, expected.inner_sizes,
            "Content sizes differ for node {node_id}"
        );
    }
}

#[test]
fn independent_panels_skip_their_common_ancestor_and_clean_panel() {
    let mut tree = panel_tree();
    let mut layout = Torin::new();
    let mut measurer = Some(VisitMeasurer::default());
    layout.measure(0, viewport(), &mut measurer, &tree);
    measurer.as_mut().unwrap().visited.clear();

    for child in [2, 8] {
        tree.set_node(child, fixed_node(40., 35.));
        layout.invalidate(child);
    }
    layout.find_best_root(&tree);
    let RootNodeCandidate::Multiple(mut roots) = layout.get_root_candidate() else {
        panic!("Expected independent measurement roots");
    };
    roots.sort_unstable();
    assert_eq!(roots, vec![1, 7]);
    layout.measure(0, viewport(), &mut measurer, &tree);

    let visited = &measurer.as_ref().unwrap().visited;
    for skipped in [0, 4, 5, 6] {
        assert!(!visited.contains(&skipped));
    }
    for measured in [1, 2, 3, 7, 8, 9] {
        assert_eq!(visited.iter().filter(|id| **id == measured).count(), 1);
    }
    assert_matches_fresh(&layout, &tree, viewport());
    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::None);
    assert!(layout.get_dirty_nodes().is_empty());
}

#[test]
fn ancestor_mutation_absorbs_descendant_heads() {
    let mut tree = panel_tree();
    let mut layout = Torin::new();
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    tree.set_node(1, fixed_node(120., 110.));
    tree.set_node(2, fixed_node(40., 35.));
    tree.set_node(8, fixed_node(40., 35.));
    for node_id in [8, 2, 1] {
        layout.invalidate(node_id);
    }
    layout.find_best_root(&tree);
    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::Valid(0));
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    assert_matches_fresh(&layout, &tree, viewport());
}

#[test]
fn content_sized_panel_promotes_measurement_to_its_parent() {
    let mut tree = panel_tree();
    tree.set_node(
        1,
        Node {
            height: Size::auto(),
            ..fixed_node(100., 90.)
        },
    );
    let mut layout = Torin::new();
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    for node_id in [2, 8] {
        tree.set_node(node_id, fixed_node(40., 35.));
        layout.invalidate(node_id);
    }
    layout.find_best_root(&tree);
    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::Valid(0));
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    assert_matches_fresh(&layout, &tree, viewport());
}

#[test]
fn offset_and_descendant_mutations_do_not_translate_twice() {
    let mut tree = panel_tree();
    let mut layout = Torin::new();
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    let mut panel = tree.read_node(&1, |node, _| node.clone()).unwrap();
    panel.offset_x = Length::new(13.);
    panel.offset_y = Length::new(19.);
    tree.set_node(1, panel);
    layout.invalidate_with_reason(1, DirtyReason::InnerLayout);
    for node_id in [2, 8] {
        tree.set_node(node_id, fixed_node(40., 35.));
        layout.invalidate(node_id);
    }
    layout.find_best_root(&tree);
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    assert_matches_fresh(&layout, &tree, viewport());
}

#[test]
fn single_child_heads_refresh_parent_content_sizes() {
    let mut tree = panel_tree();
    for panel in [1, 7] {
        tree.remove(panel + 2);
    }
    let mut layout = Torin::new();
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    for node_id in [2, 8] {
        tree.set_node(node_id, fixed_node(40., 35.));
        layout.invalidate(node_id);
    }
    layout.find_best_root(&tree);
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    assert_matches_fresh(&layout, &tree, viewport());
}

#[test]
fn removing_a_pending_head_preserves_other_heads() {
    let mut tree = panel_tree();
    let mut layout = Torin::new();
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    layout.invalidate(2);
    layout.invalidate(8);
    layout.find_best_root(&tree);
    layout.raw_remove(1);
    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::Valid(7));

    layout.remove(1, &tree, true);
    tree.remove(1);
    layout.find_best_root(&tree);
    layout.measure(0, viewport(), &mut None::<NoopMeasurer>, &tree);
    assert_matches_fresh(&layout, &tree, viewport());
    layout.reset();
    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::None);
}
