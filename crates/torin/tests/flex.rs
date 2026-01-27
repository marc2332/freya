use std::{
    any::Any,
    rc::Rc,
};

use euclid::Length;
use torin::{
    prelude::*,
    test_utils::*,
};

struct PhaseTrackingMeasurer {
    target: usize,
    saw_initial: bool,
    saw_final: bool,
}

impl PhaseTrackingMeasurer {
    fn new(target: usize) -> Self {
        Self {
            target,
            saw_initial: false,
            saw_final: false,
        }
    }
}

impl LayoutMeasurer<usize> for PhaseTrackingMeasurer {
    fn measure(
        &mut self,
        node_id: usize,
        _node: &Node,
        _size: &Size2D,
        phase: Phase,
        _parent_phase: Phase,
    ) -> Option<(Size2D, Rc<dyn Any>)> {
        if node_id != self.target {
            return None;
        }

        let size = match phase {
            Phase::Initial => {
                self.saw_initial = true;
                Size2D::new(80.0, 20.0)
            }
            Phase::Final => {
                self.saw_final = true;
                Size2D::new(80.0, 60.0)
            }
        };

        Some((size, Rc::new(())))
    }

    fn should_hook_measurement(&mut self, node_id: usize) -> bool {
        node_id == self.target
    }

    fn should_measure_inner_children(&mut self, _node_id: usize) -> bool {
        _node_id != self.target
    }
}

#[test]
pub fn flex_generic() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1, 2, 3, 4],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Percentage(Length::new(10.)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(1.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(50.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(3.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 20.0)),
    );
    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 20.0), Size2D::new(100.0, 32.5)),
    );
    assert_eq!(
        layout.get(&3).unwrap().area,
        Rect::new(Point2D::new(0.0, 52.5), Size2D::new(100.0, 50.0)),
    );
    assert_eq!(
        layout.get(&4).unwrap().area,
        Rect::new(Point2D::new(0.0, 102.5), Size2D::new(100.0, 97.5)),
    );
}

#[test]
pub fn flex_under_1_flex_grow() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(0.2)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(0.5)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 40.0)),
    );
    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 40.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn flex_grow_balance() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1, 2, 3, 4],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(1.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(2.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(3.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(4.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 20.0)),
    );
    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 20.0), Size2D::new(100.0, 40.0)),
    );
    assert_eq!(
        layout.get(&3).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 60.0), Size2D::new(100.0, 60.0)),
    );
    assert_eq!(
        layout.get(&4).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 120.0), Size2D::new(100.0, 80.0)),
    );
}

#[test]
pub fn flex_large_grow_balance() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1, 2, 3, 4],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(5.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(65.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(30.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(100.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 5.0)),
    );
    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(100.0, 65.0)),
    );
    assert_eq!(
        layout.get(&3).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 70.0), Size2D::new(100.0, 30.0)),
    );
    assert_eq!(
        layout.get(&4).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn flex_with_inner_percentage() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(1.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        2,
        Some(0),
        vec![3],
        Node::from_size_and_visible_size(
            Size::Pixels(Length::new(100.0)),
            Size::Inner,
            VisibleSize::Full,
            VisibleSize::InnerPercentage(Length::new(50.0)),
        ),
    );
    mocked_tree.add(
        3,
        Some(2),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 150.0)),
    );
    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 150.0), Size2D::new(100.0, 50.0)),
    );
    assert_eq!(
        layout.get(&3).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 150.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn flex_root_candidate_resolution() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        1,
        Some(0),
        vec![2, 3],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_tree.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(1.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        3,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(1.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );
    assert_eq!(
        layout.get(&3).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    mocked_tree.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(3.0)),
            Direction::Vertical,
        ),
    );
    layout.invalidate(2);

    layout.find_best_root(&mut mocked_tree);

    // It is Node 1 because it has a `flex` content
    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::Valid(1));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 150.0)),
    );
    assert_eq!(
        layout.get(&3).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 150.0), Size2D::new(100.0, 50.0)),
    );
}

#[test]
pub fn flex_cross_alignment_uses_initial_measurement() {
    let mut layout = Torin::<usize>::new();
    let mut measurer = Some(PhaseTrackingMeasurer::new(1));

    let mut mocked_tree = TestingTree::default();

    let mut parent = Node::from_size_and_content(
        Size::Pixels(Length::new(200.0)),
        Size::Pixels(Length::new(100.0)),
        Content::Flex,
    );
    parent.direction = Direction::Horizontal;
    parent.cross_alignment = Alignment::Center;
    parent.main_alignment = Alignment::Start;

    mocked_tree.add(0, None, vec![1], parent);
    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(Size::Inner, Size::Inner, Direction::Vertical),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    let child_area = layout.get(&1).unwrap().area;
    assert_eq!(child_area.origin.y, 40.0);
    assert_eq!(child_area.size.height, 60.0);

    let measurer = measurer.as_ref().unwrap();
    assert!(measurer.saw_initial);
    assert!(measurer.saw_final);
}
