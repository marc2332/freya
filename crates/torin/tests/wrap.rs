use euclid::Length;
use torin::{
    prelude::*,
    test_utils::*,
};

#[test]
pub fn content_wrap_horizontal() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    let mut root = Node::from_size_and_direction(
        Size::Pixels(Length::new(300.0)),
        Size::Pixels(Length::new(200.0)),
        Direction::Horizontal,
    );
    root.content = Content::Wrap;

    mocked_tree.add(0, None, vec![1, 2, 3], root);

    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(150.0)),
            Size::Pixels(Length::new(20.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(150.0)),
            Size::Pixels(Length::new(20.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(150.0)),
            Size::Pixels(Length::new(20.0)),
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
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(150.0, 20.0)),
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(150.0, 0.0), Size2D::new(150.0, 20.0)),
    );

    assert_eq!(
        layout.get(&3).unwrap().area,
        Rect::new(Point2D::new(0.0, 20.0), Size2D::new(150.0, 20.0)),
    );
}

#[test]
pub fn content_wrap_vertical() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();

    mocked_tree.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Wrap,
        ),
    );

    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(150.0)),
            Direction::Vertical,
        ),
    );
    mocked_tree.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(150.0)),
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
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 150.0)),
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(100.0, 150.0)),
    );
}
