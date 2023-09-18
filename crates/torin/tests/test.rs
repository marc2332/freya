use std::collections::{HashMap, HashSet};

use torin::prelude::*;

struct TestingMeasurer;

impl LayoutMeasurer<usize> for TestingMeasurer {
    fn measure(
        &mut self,
        _node_id: usize,
        _node: &Node,
        _area: &Area,
        _parent_size: &Area,
        _available_parent_area: &Area,
    ) -> Option<Area> {
        None
    }
}

#[derive(Default)]
struct TestingDOM {
    mapper: HashMap<usize, (Option<usize>, Vec<usize>, u16, Node)>,
}

impl TestingDOM {
    fn add(&mut self, node_id: usize, parent: Option<usize>, children: Vec<usize>, node: Node) {
        let depth = parent.map(|p| self.mapper.get(&p).unwrap().2).unwrap_or(0) + 1;
        self.mapper.insert(node_id, (parent, children, depth, node));
    }

    fn set_node(&mut self, node_id: usize, node: Node) {
        self.mapper.get_mut(&node_id).unwrap().3 = node;
    }

    fn remove(&mut self, node_id: usize) {
        let node = self.mapper.get(&node_id).unwrap().clone();

        if let Some((_, parent_children, _, _)) = node.0.map(|p| self.mapper.get_mut(&p)).flatten()
        {
            parent_children.retain(|c| *c != node_id);
        }

        self.mapper.remove(&node_id);

        for child in node.1 {
            self.remove(child);
        }
    }
}

impl DOMAdapter<usize> for TestingDOM {
    fn children_of(&mut self, node_id: &usize) -> Vec<usize> {
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

    fn get_node(&self, node_id: &usize) -> Option<Node> {
        self.mapper.get(node_id).map(|c| c.3.clone())
    }

    fn is_node_valid(&mut self, _node_id: &usize) -> bool {
        true
    }

    fn closest_common_parent(&self, node_id_a: &usize, _node_id_b: &usize) -> Option<usize> {
        Some(self.parent_of(node_id_a)?)
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

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.find_best_root(&mut mocked_dom);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 500.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 500.0), Size2D::new(1000.0, 500.0)),
    );
}

#[test]
pub fn root_200px_children_50per50per() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Horizontal,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Horizontal,
        ),
    );

    layout.find_best_root(&mut mocked_dom);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );
}

#[test]
pub fn layout_dirty_nodes() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        None,
        vec![2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
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
        &mut mocked_dom,
    );

    // CASE 1
    // - Root is fixed
    // - Child A is fixed
    // - Child A[0] is fixed

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(50.0, 50.0)),
    );

    mocked_dom.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(10.0)),
            Size::Pixels(Length::new(10.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(2);

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2]));

    // CASE 2
    // Same as Case 1 but we make Child A depend on Child A[0]'s size

    mocked_dom.set_node(
        1,
        Node::from_size_and_direction(
            Size::Inner,
            Size::Pixels(Length::new(10.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(1);

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2, 1]));

    // CASE 3
    // Same as Case 2, but triggers a change in Child A[0]

    mocked_dom.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(2);

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2, 1]));

    // CASE 4
    // Same as Case 3, but triggers a change in the root

    mocked_dom.set_node(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(150.0)),
            Size::Pixels(Length::new(150.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(0);

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2, 1, 0]));
}

#[test]
pub fn direction() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
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
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    // Change the direction from vertical to horizontal

    mocked_dom.set_node(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Horizontal,
        ),
    );
    layout.invalidate(0);

    layout.find_best_root(&mut mocked_dom);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn scroll() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_scroll(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Length::new(50.0),
            Length::new(0.0),
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.find_best_root(&mut mocked_dom);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(50.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(50.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn padding() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_padding(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Gaps::new(5.0, 10.0, 15.0, 20.0),
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
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
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(20.0, 5.0), Size2D::new(170.0, 180.0)),
    );
}

#[test]
pub fn caching() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_padding(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Gaps::new(5.0, 0.0, 0.0, 0.0),
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
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
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(200.0, 195.0)),
    );

    mocked_dom.set_node(
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(1);

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(100.0, 195.0)),
    );
}

#[test]
pub fn sibling_increments_area() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(Size::Inner, Size::Inner, DirectionMode::Vertical),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
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
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(300.0, 200.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(150.0, 100.0)),
    );
}

#[test]
pub fn node_removal() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2, 3],
        Node::from_size_and_direction(Size::Inner, Size::Inner, DirectionMode::Vertical),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(1),
        vec![],
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
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 400.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 200.0), Size2D::new(200.0, 200.0)),
    );

    layout.remove(2, &mut mocked_dom, true);

    mocked_dom.remove(2);

    layout.find_best_root(&mut mocked_dom);

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([1, 3]));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );
}

#[test]
pub fn display_horizontal() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_display_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DisplayMode::Center,
            DirectionMode::Horizontal,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
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
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(50.0, 0.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn display_vertical_with_inner_children() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_display_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DisplayMode::Center,
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_padding(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Gaps::new(5.0, 5.0, 5.0, 5.0),
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.find_best_root(&mut mocked_dom);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 50.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(5.0, 55.0), Size2D::new(90.0, 90.0)),
    );
}

#[test]
pub fn deep_tree() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_display_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DisplayMode::Center,
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![3],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(2),
        vec![4],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(3),
        vec![5],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        5,
        Some(4),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.find_best_root(&mut mocked_dom);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    mocked_dom.set_node(
        4,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(200.0)),
            Size::Percentage(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(4);

    layout.find_best_root(&mut mocked_dom);
    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::Valid(4));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::None);
}

#[test]
pub fn stacked() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
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
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );

    mocked_dom.set_node(
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(2);

    layout.find_best_root(&mut mocked_dom);

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );
}

#[test]
pub fn two_cols_auto() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Inner,
            Size::Pixels(Length::new(400.0)),
            DirectionMode::Horizontal,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.find_best_root(&mut mocked_dom);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(400.0, 400.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(400.0, 400.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(200.0, 0.0), Size2D::new(200.0, 200.0)),
    );
}

#[test]
pub fn margin() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_margin(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Gaps::new(5.0, 5.0, 5.0, 5.0),
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_margin(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Gaps::new(5.0, 5.0, 5.0, 5.0),
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    let node_areas = layout.get(1).unwrap();

    assert_eq!(
        node_areas.area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(210.0, 210.0)),
    );

    assert_eq!(
        node_areas.box_area(),
        Rect::new(Point2D::new(5.0, 5.0), Size2D::new(200.0, 200.0)),
    );
}
