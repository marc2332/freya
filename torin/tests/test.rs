use std::collections::{HashMap, HashSet};

use euclid::*;
use torin::*;

struct TestingMeasurer;

impl LayoutMeasurer<usize> for TestingMeasurer {
    fn measure(
        &mut self,
        _node_id: usize,
        _node: &NodeData,
        _area: &Rect<f32, Measure>,
        _parent_size: &Rect<f32, Measure>,
        _available_parent_size: &Rect<f32, Measure>,
    ) -> Option<Rect<f32, Measure>> {
        None
    }
}

#[derive(Default)]
struct TreeMapper {
    mapper: HashMap<usize, (Option<usize>, Vec<usize>, u16)>,
}

impl TreeMapper {
    fn add(&mut self, node_id: usize, parent: Option<usize>, children: Vec<usize>) {
        let depth = parent.map(|p| self.mapper.get(&p).unwrap().2).unwrap_or(0) + 1;
        self.mapper.insert(node_id, (parent, children, depth));
    }

    fn remove(&mut self, node_id: usize) {
        let node = self.mapper.get(&node_id).unwrap().clone();

        if let Some((_, parent_children, _)) = node.0.map(|p| self.mapper.get_mut(&p)).flatten() {
            parent_children.retain(|c| *c != node_id);
        }

        self.mapper.remove(&node_id);

        for child in node.1 {
            self.remove(child);
        }
    }
}

impl NodeResolver<usize> for TreeMapper {
    fn children_of(&self, node_id: &usize) -> Vec<usize> {
        self.mapper
            .get(node_id)
            .map(|c| c.1.clone())
            .unwrap_or_default()
    }

    fn parent_of(&self, node_id: &usize) -> Option<usize> {
        self.mapper.get(node_id).map(|c| c.0).flatten()
    }

    fn height(&self, node_id: &usize) -> Option<u16> {
        self.mapper.get(node_id).map(|c| c.2)
    }
}

fn test_utils() -> (Torin<usize>, Option<TestingMeasurer>) {
    let layout = Torin::<usize>::new();
    let measurer = Some(TestingMeasurer);

    (layout, measurer)
}

#[test]
pub fn root_100per_children_50per50per() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1, 2]);
    tree_mapper.add(1, Some(0), vec![]);
    tree_mapper.add(2, Some(0), vec![]);

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Horizontal,
        ),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.has_pending_measurements(&tree_mapper);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 500.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 500.0), Size2D::new(1000.0, 500.0)),
    );
}

#[test]
pub fn root_200px_children_50per50per() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1, 2]);
    tree_mapper.add(1, Some(0), vec![]);
    tree_mapper.add(2, Some(0), vec![]);

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Horizontal,
        ),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Horizontal,
        ),
    );

    layout.has_pending_measurements(&tree_mapper);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );
}

#[test]
pub fn layout_dirty_nodes() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1]);
    tree_mapper.add(1, None, vec![2]);
    tree_mapper.add(2, Some(1), vec![]);

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    // CASE 1
    // - Root is fixed
    // - Child A is fixed
    // - Child A[0] is fixed

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(50.0, 50.0)),
    );

    layout.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(10.0)),
            Size::Pixels(Length::new(10.0)),
            DirectionMode::Vertical,
        ),
    );

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2]));

    // CASE 2
    // Same as Case 1 but we make Child A depend on Child A[0]'s size

    layout.set_node(
        1,
        Node::from_size_and_direction(
            Size::Inner,
            Size::Pixels(Length::new(10.0)),
            DirectionMode::Vertical,
        ),
    );

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2, 1]));

    // CASE 3
    // Same as Case 2, but triggers a change in Child A[0]

    layout.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2, 1]));

    // CASE 4
    // Same as Case 3, but triggers a change in the root

    layout.set_node(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(150.0)),
            Size::Pixels(Length::new(150.0)),
            DirectionMode::Vertical,
        ),
    );

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2, 1, 0]));
}

#[test]
pub fn direction() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1, 2]);
    tree_mapper.add(1, None, vec![2]);
    tree_mapper.add(2, Some(1), vec![]);

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    // Change the direction from vertical to horizontal

    layout.set_node(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Horizontal,
        ),
    );

    layout.has_pending_measurements(&tree_mapper);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn scroll() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1, 2]);
    tree_mapper.add(1, Some(0), vec![]);
    tree_mapper.add(2, Some(0), vec![]);

    layout.add(
        0,
        Node::from_size_and_scroll(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Length::new(50.0),
            Length::new(0.0),
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.has_pending_measurements(&tree_mapper);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(50.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(50.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn padding() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1]);
    tree_mapper.add(1, Some(0), vec![]);

    layout.add(
        0,
        Node::from_size_and_padding(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Paddings::new(5.0, 10.0, 15.0, 20.0),
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(20.0, 5.0), Size2D::new(170.0, 180.0)),
    );
}

#[test]
pub fn caching() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1]);
    tree_mapper.add(1, Some(0), vec![]);

    layout.add(
        0,
        Node::from_size_and_padding(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Paddings::new(5.0, 0.0, 0.0, 0.0),
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(200.0, 195.0)),
    );

    layout.set_node(
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(100.0, 195.0)),
    );
}

#[test]
pub fn sibling_increments_area() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1, 2]);
    tree_mapper.add(1, Some(0), vec![]);
    tree_mapper.add(2, Some(0), vec![]);

    layout.add(
        0,
        Node::from_size_and_direction(Size::Inner, Size::Inner, DirectionMode::Vertical),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(300.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(150.0, 100.0)),
    );
}

#[test]
pub fn node_removal() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1]);
    tree_mapper.add(1, Some(0), vec![2, 3]);
    tree_mapper.add(2, Some(1), vec![]);
    tree_mapper.add(3, Some(1), vec![]);

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(Size::Inner, Size::Inner, DirectionMode::Vertical),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        3,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 400.0)),
    );

    assert_eq!(
        layout.get_size(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 200.0), Size2D::new(200.0, 200.0)),
    );

    layout.remove(2, &tree_mapper, true);

    tree_mapper.remove(2);

    layout.has_pending_measurements(&tree_mapper);

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([1, 3]));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );
}

#[test]
pub fn display_horizontal() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1]);
    tree_mapper.add(1, Some(0), vec![]);

    layout.add(
        0,
        Node::from_size_and_display_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DisplayMode::Center,
            DirectionMode::Horizontal,
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(50.0, 0.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn display_vertical_with_inner_children() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1]);
    tree_mapper.add(1, Some(0), vec![2]);
    tree_mapper.add(2, Some(1), vec![]);

    layout.add(
        0,
        Node::from_size_and_display_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DisplayMode::Center,
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_padding(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Paddings::new(5.0, 5.0, 5.0, 5.0),
        ),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.has_pending_measurements(&tree_mapper);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 50.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(5.0, 55.0), Size2D::new(90.0, 90.0)),
    );
}

#[test]
pub fn deep_tree() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1]);
    tree_mapper.add(1, Some(0), vec![2]);
    tree_mapper.add(2, Some(1), vec![3]);
    tree_mapper.add(3, Some(2), vec![4]);
    tree_mapper.add(4, Some(3), vec![5]);
    tree_mapper.add(5, Some(4), vec![]);

    layout.add(
        0,
        Node::from_size_and_display_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DisplayMode::Center,
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        3,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        4,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        5,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.has_pending_measurements(&tree_mapper);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    layout.set_node(
        4,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(200.0)),
            Size::Percentage(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.has_pending_measurements(&tree_mapper);
    assert_eq!(layout.get_tallest_dirty_node(), TallestDirtyNode::Valid(4));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(layout.get_tallest_dirty_node(), TallestDirtyNode::None);
}

#[test]
pub fn stacked() {
    let (mut layout, mut measurer) = test_utils();

    let mut tree_mapper = TreeMapper::default();
    tree_mapper.add(0, None, vec![1, 2]);
    tree_mapper.add(1, Some(0), vec![]);
    tree_mapper.add(2, Some(0), vec![]);

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.insert(
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );

    layout.set_node(
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.has_pending_measurements(&tree_mapper);

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &tree_mapper,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );
}
