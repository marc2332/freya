#[cfg(test)]
use torin::{
    prelude::*,
    test_utils::*,
};

#[test]
pub fn absolute() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1],
        Node::from_size_and_padding(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Gaps::new(20.0, 20.0, 20.0, 20.0),
        ),
    );
    mocked_tree.add(
        1,
        Some(0),
        vec![2, 3, 4, 5],
        Node::from_size_and_padding(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Gaps::new(30.0, 30.0, 30.0, 30.0),
        ),
    );
    mocked_tree.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Absolute(Box::new(PositionSides {
                top: Some(100.0),
                right: None,
                bottom: None,
                left: Some(50.0),
            })),
        ),
    );
    mocked_tree.add(
        3,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Absolute(Box::new(PositionSides {
                top: Some(100.0),
                right: Some(50.0),
                bottom: None,
                left: None,
            })),
        ),
    );
    mocked_tree.add(
        4,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Absolute(Box::new(PositionSides {
                top: None,
                right: Some(50.0),
                bottom: Some(100.0),
                left: None,
            })),
        ),
    );
    mocked_tree.add(
        5,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Absolute(Box::new(PositionSides {
                top: None,
                right: None,
                bottom: Some(100.0),
                left: Some(50.0),
            })),
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(100.0, 150.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(&3).unwrap().area.round(),
        Rect::new(Point2D::new(700.0, 150.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(&4).unwrap().area.round(),
        Rect::new(Point2D::new(700.0, 650.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(&5).unwrap().area.round(),
        Rect::new(Point2D::new(100.0, 650.0), Size2D::new(200.0, 200.0)),
    );
}

#[test]
pub fn absolute_inside_offset_container() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(1000.0)),
            Size::Pixels(Length::new(1000.0)),
            Direction::Vertical,
        ),
    );

    mocked_tree.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_offset(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Length::new(30.0),
            Length::new(10.0),
        ),
    );

    mocked_tree.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
            Position::Absolute(Box::new(PositionSides {
                top: Some(20.0),
                right: None,
                bottom: None,
                left: Some(40.0),
            })),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(70.0, 30.0), Size2D::new(50.0, 50.0)),
    );
}

#[test]
pub fn global() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1],
        Node::from_size_and_padding(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Gaps::new(20.0, 20.0, 20.0, 20.0),
        ),
    );
    mocked_tree.add(
        1,
        Some(0),
        vec![2, 3, 4, 5],
        Node::from_size_and_padding(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Gaps::new(30.0, 30.0, 30.0, 30.0),
        ),
    );
    mocked_tree.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Global(Box::new(PositionSides {
                top: Some(100.0),
                right: None,
                bottom: None,
                left: Some(50.0),
            })),
        ),
    );
    mocked_tree.add(
        3,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Global(Box::new(PositionSides {
                top: Some(100.0),
                right: Some(50.0),
                bottom: None,
                left: None,
            })),
        ),
    );
    mocked_tree.add(
        4,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Global(Box::new(PositionSides {
                top: None,
                right: Some(50.0),
                bottom: Some(100.0),
                left: None,
            })),
        ),
    );
    mocked_tree.add(
        5,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Global(Box::new(PositionSides {
                top: None,
                right: None,
                bottom: Some(100.0),
                left: Some(50.0),
            })),
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(50.0, 100.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(&3).unwrap().area.round(),
        Rect::new(Point2D::new(750.0, 100.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(&4).unwrap().area.round(),
        Rect::new(Point2D::new(750.0, 700.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(&5).unwrap().area.round(),
        Rect::new(Point2D::new(50.0, 700.0), Size2D::new(200.0, 200.0)),
    );
}

#[test]
pub fn absolute_with_inner_sized_height_and_bottom() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
    );
    // Fixed-size absolute child with bottom: 0 (baseline, should work)
    mocked_tree.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(40.0)),
            Size::Pixels(Length::new(40.0)),
            Position::new_absolute().bottom(0.0).left(0.0),
        ),
    );
    // Inner-sized height absolute child with bottom: 0
    mocked_tree.add(
        2,
        Some(0),
        vec![3],
        Node::from_size_and_position(
            Size::Pixels(Length::new(40.0)),
            Size::Inner,
            Position::new_absolute().bottom(0.0).left(50.0),
        ),
    );
    // Grandchild that gives the parent its height
    mocked_tree.add(
        3,
        Some(2),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(40.0)),
            Size::Pixels(Length::new(40.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    // Fixed-size child: bottom=0 means origin.y = 100 - 0 - 40 = 60
    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 60.0), Size2D::new(40.0, 40.0)),
    );

    // Inner-sized child: should also be at y=60 (100 - 0 - 40)
    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(50.0, 60.0), Size2D::new(40.0, 40.0)),
    );

    // Grandchild must be translated along with its parent
    assert_eq!(
        layout.get(&3).unwrap().area,
        Rect::new(Point2D::new(50.0, 60.0), Size2D::new(40.0, 40.0)),
    );
}

#[test]
pub fn absolute_with_inner_sized_width_and_right() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Horizontal,
        ),
    );
    // Inner-sized width absolute child with right: 0
    mocked_tree.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_position(
            Size::Inner,
            Size::Pixels(Length::new(50.0)),
            Position::new_absolute().right(0.0).top(0.0),
        ),
    );
    // Grandchild that gives the parent its width
    mocked_tree.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(60.0)),
            Size::Pixels(Length::new(50.0)),
            Direction::Horizontal,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    // Inner-sized width child: right=0 means origin.x = 200 - 0 - 60 = 140
    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(140.0, 0.0), Size2D::new(60.0, 50.0)),
    );

    // Grandchild must be translated along with its parent
    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(140.0, 0.0), Size2D::new(60.0, 50.0)),
    );
}

#[test]
pub fn global_with_inner_sized_and_bottom() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_tree = TestingTree::default();
    mocked_tree.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(500.0)),
            Size::Pixels(Length::new(500.0)),
            Direction::Vertical,
        ),
    );
    // Inner-sized height global child with bottom: 10
    mocked_tree.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_position(
            Size::Pixels(Length::new(80.0)),
            Size::Inner,
            Position::new_global().bottom(10.0).left(20.0),
        ),
    );
    // Grandchild that gives the parent its height
    mocked_tree.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(80.0)),
            Size::Pixels(Length::new(30.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(500.0, 500.0)),
        &mut measurer,
        &mut mocked_tree,
    );

    // Global inner-sized child: bottom=10 means y = 500 - 10 - 30 = 460
    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(20.0, 460.0), Size2D::new(80.0, 30.0)),
    );

    // Grandchild must be translated along with its parent
    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(20.0, 460.0), Size2D::new(80.0, 30.0)),
    );
}
