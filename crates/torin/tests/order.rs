use torin::{
    prelude::*,
    test_utils::*,
};

fn stacked_tree(parent: Node) -> (Torin<usize>, Option<NoopMeasurer>, TestingTree) {
    let (layout, measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(0, None, vec![1, 2, 3], parent);
    for id in 1..=3 {
        mocked_tree.add(
            id,
            Some(0),
            vec![],
            Node::from_size_and_direction(
                Size::Pixels(Length::new(100.0)),
                Size::Pixels(Length::new(100.0)),
                Direction::Vertical,
            ),
        );
    }

    (layout, measurer, mocked_tree)
}

#[test]
pub fn backward_order_stacks_horizontal_children_in_reverse() {
    let (mut layout, mut measurer, mut mocked_tree) =
        stacked_tree(Node::from_size_and_direction_and_order(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Horizontal,
            Order::Backward,
        ));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(300.0, 100.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(layout.get(&3).unwrap().area.origin, Point2D::new(0.0, 0.0));
    assert_eq!(
        layout.get(&2).unwrap().area.origin,
        Point2D::new(100.0, 0.0)
    );
    assert_eq!(
        layout.get(&1).unwrap().area.origin,
        Point2D::new(200.0, 0.0)
    );
}

#[test]
pub fn backward_order_stacks_vertical_children_in_reverse() {
    let (mut layout, mut measurer, mut mocked_tree) =
        stacked_tree(Node::from_size_and_direction_and_order(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(300.0)),
            Direction::Vertical,
            Order::Backward,
        ));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 300.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(layout.get(&3).unwrap().area.origin, Point2D::new(0.0, 0.0));
    assert_eq!(
        layout.get(&2).unwrap().area.origin,
        Point2D::new(0.0, 100.0)
    );
    assert_eq!(
        layout.get(&1).unwrap().area.origin,
        Point2D::new(0.0, 200.0)
    );
}

#[test]
pub fn backward_order_composes_with_center_alignment() {
    let (mut layout, mut measurer) = test_utils();

    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Pixels(Length::new(300.0)),
        Size::Pixels(Length::new(100.0)),
        Alignment::Center,
        Alignment::Start,
        Direction::Horizontal,
    );
    parent.order = Order::Backward;

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(0, None, vec![1, 2], parent);
    for id in 1..=2 {
        mocked_tree.add(
            id,
            Some(0),
            vec![],
            Node::from_size_and_direction(
                Size::Pixels(Length::new(50.0)),
                Size::Pixels(Length::new(50.0)),
                Direction::Vertical,
            ),
        );
    }

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(300.0, 100.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&2).unwrap().area.origin,
        Point2D::new(100.0, 0.0)
    );
    assert_eq!(
        layout.get(&1).unwrap().area.origin,
        Point2D::new(150.0, 0.0)
    );
}

#[test]
pub fn backward_order_keeps_spacing_between_children() {
    let mut parent = Node::from_size_and_alignments_and_direction_and_spacing(
        Size::Pixels(Length::new(320.0)),
        Size::Pixels(Length::new(100.0)),
        Alignment::Start,
        Alignment::Start,
        Direction::Horizontal,
        Length::new(10.0),
    );
    parent.order = Order::Backward;

    let (mut layout, mut measurer, mut mocked_tree) = stacked_tree(parent);

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(320.0, 100.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(layout.get(&3).unwrap().area.origin, Point2D::new(0.0, 0.0));
    assert_eq!(
        layout.get(&2).unwrap().area.origin,
        Point2D::new(110.0, 0.0)
    );
    assert_eq!(
        layout.get(&1).unwrap().area.origin,
        Point2D::new(220.0, 0.0)
    );
}

#[test]
pub fn backward_order_composes_with_flex() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    let mut root = Node::from_size_and_content(
        Size::Pixels(Length::new(300.0)),
        Size::Pixels(Length::new(100.0)),
        Content::Flex,
    );
    root.direction = Direction::Horizontal;
    root.order = Order::Backward;
    mocked_tree.add(0, None, vec![1, 2, 3], root);
    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Flex(Length::new(1.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(300.0, 100.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(layout.get(&3).unwrap().area.origin, Point2D::new(0.0, 0.0));
    assert_eq!(layout.get(&2).unwrap().area.width(), 200.0);
    assert_eq!(layout.get(&2).unwrap().area.origin, Point2D::new(50.0, 0.0));
    assert_eq!(
        layout.get(&1).unwrap().area.origin,
        Point2D::new(250.0, 0.0)
    );
}
