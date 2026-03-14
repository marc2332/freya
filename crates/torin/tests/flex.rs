use std::{
    any::Any,
    rc::Rc,
};

use euclid::Length;
use torin::{
    prelude::*,
    test_utils::*,
};

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

/// Simulates text wrapping: height grows as available width shrinks.
struct TextLikeMeasurer {
    text_nodes: Vec<usize>,
    text_content_width: f32,
    line_height: f32,
}

impl LayoutMeasurer<usize> for TextLikeMeasurer {
    fn measure(
        &mut self,
        node_id: usize,
        _node: &Node,
        size: &Size2D,
    ) -> Option<(Size2D, Rc<dyn Any>)> {
        if !self.text_nodes.contains(&node_id) {
            return None;
        }

        let available_width = size.width.max(1.0);
        let lines = (self.text_content_width / available_width).ceil();
        let width = self.text_content_width.min(available_width);
        let height = lines * self.line_height;

        Some((Size2D::new(width, height), Rc::new(())))
    }

    fn should_hook_measurement(&mut self, node_id: usize) -> bool {
        self.text_nodes.contains(&node_id)
    }

    fn should_measure_inner_children(&mut self, node_id: usize) -> bool {
        !self.text_nodes.contains(&node_id)
    }
}

/// Regression test for https://github.com/marc2332/freya/issues/1098
#[test]
pub fn flex_with_cross_align_center_and_text() {
    let mut layout = Torin::<usize>::new();

    let mut measurer = Some(TextLikeMeasurer {
        text_nodes: vec![3],
        text_content_width: 80.0,
        line_height: 20.0,
    });

    let mut mocked_tree = TestingTree::default();

    mocked_tree.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(48.0)),
            Direction::Vertical,
        ),
    );

    mocked_tree.add(1, Some(0), vec![2, 4], {
        let mut node = Node::from_size_and_alignments_and_direction(
            Size::Inner,
            Size::Inner,
            Alignment::Start,
            Alignment::Center,
            Direction::Horizontal,
        );
        node.content = Content::Flex;
        node.spacing = Length::new(8.0);
        node
    });

    mocked_tree.add(
        2,
        Some(1),
        vec![3],
        Node::from_size_and_direction(
            Size::Flex(Length::new(1.0)),
            Size::Inner,
            Direction::Vertical,
        ),
    );

    mocked_tree.add(
        3,
        Some(2),
        vec![],
        Node::from_size_and_direction(Size::Inner, Size::Inner, Direction::Vertical),
    );

    mocked_tree.add(
        4,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(12.0)),
            Size::Pixels(Length::new(12.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    let node_2_area = layout.get(&2).unwrap().area;
    let node_3_area = layout.get(&3).unwrap().area;
    let node_4_area = layout.get(&4).unwrap().area;

    // Text fits on one line with the flex-grown width
    assert_eq!(node_3_area.size.height, 20.0);
    assert_eq!(node_2_area.size.height, 20.0);

    // The 12px rect should be vertically centered: (20 - 12) / 2 = 4px offset
    assert_eq!(node_4_area.size.height, 12.0);
    assert_eq!(node_4_area.origin.y, node_2_area.origin.y + 4.0);
}
