use std::{
    any::Any,
    rc::Rc,
};

use torin::{
    prelude::*,
    test_utils::TestingTree,
};

#[derive(Default)]
struct ChangeMeasurer {
    changes: Vec<LayoutChange<usize>>,
    offset: f32,
    hide_children: bool,
}

impl LayoutMeasurer<usize> for ChangeMeasurer {
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

    fn should_measure_inner_children(&mut self, _node_id: usize) -> bool {
        true
    }

    fn should_post_measure(&mut self, node_id: usize) -> bool {
        node_id == 0
    }

    fn post_measure(
        &mut self,
        _node_id: usize,
        _node_layout: &LayoutNode,
        children: &[usize],
        _layout: &Torin<usize>,
    ) -> PostMeasure<usize> {
        if self.hide_children {
            PostMeasure {
                hidden_children: children.to_vec(),
                ..PostMeasure::default()
            }
        } else {
            PostMeasure {
                offsets: children
                    .iter()
                    .map(|child| (*child, Length::new(self.offset), Length::new(0.)))
                    .collect(),
                ..PostMeasure::default()
            }
        }
    }

    fn notify_layout_change(&mut self, change: LayoutChange<usize>) {
        self.changes.push(change);
    }
}

#[test]
fn cached_layout_changes() {
    let mut layout = Torin::<usize>::new();
    let mut measurer = Some(ChangeMeasurer::default());
    let initial = LayoutNode::default();
    let mut inner = initial.clone();
    inner.inner_area.origin.x = 10.;
    let mut data = inner.clone();
    data.data = Some(Rc::new(42));
    let mut moved = data.clone();
    moved.area.origin.x = 10.;
    let mut hidden = moved.clone();
    hidden.hidden = true;

    for (node, expected) in [
        (initial.clone(), Some(LayoutChange::Changed(1))),
        (initial, None),
        (inner, Some(LayoutChange::ContentChanged(1))),
        (data.clone(), Some(LayoutChange::ContentChanged(1))),
        (data, None),
        (moved, Some(LayoutChange::Changed(1))),
        (hidden, Some(LayoutChange::Changed(1))),
    ] {
        layout.cache_node(1, node, &mut measurer);
        let changes = std::mem::take(&mut measurer.as_mut().unwrap().changes);
        assert_eq!(changes.as_slice(), expected.as_slice());
    }

    let node = LayoutNode::default();
    layout.cache_node(2, node.clone(), &mut None::<NoopMeasurer>);
    assert_eq!(layout.get(&2), Some(&node));
}

#[test]
fn post_measure_layout_changes() {
    let mut layout = Torin::<usize>::new();
    let mut measurer = Some(ChangeMeasurer::default());
    let mut tree = TestingTree::default();
    for (node_id, parent, children) in [
        (0, None, vec![1]),
        (1, Some(0), vec![2]),
        (2, Some(1), vec![]),
    ] {
        tree.add(
            node_id,
            parent,
            children,
            Node::from_size_and_direction(
                Size::Pixels(Length::new(100.)),
                Size::Pixels(Length::new(100.)),
                Direction::Vertical,
            ),
        );
    }
    let area = Area::from_size(Size2D::new(200., 200.));
    layout.measure(0, area, &mut measurer, &tree);
    measurer.as_mut().unwrap().changes.clear();

    for (offset, hide_children, changed) in
        [(0., false, false), (10., false, true), (10., true, true)]
    {
        measurer.as_mut().unwrap().offset = offset;
        measurer.as_mut().unwrap().hide_children = hide_children;
        layout.invalidate(0);
        layout.measure(0, area, &mut measurer, &tree);
        let changes = std::mem::take(&mut measurer.as_mut().unwrap().changes);
        if !changed {
            assert!(changes.is_empty());
        }
        for node_id in [1, 2] {
            assert_eq!(changes.contains(&LayoutChange::Changed(node_id)), changed);
            let node = layout.get(&node_id).unwrap();
            assert_eq!(node.hidden, hide_children);
            if !hide_children {
                assert_eq!(node.area.origin.x, offset);
            }
        }
    }
}
