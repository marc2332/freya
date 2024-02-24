use torin::{prelude::*, test_utils::*};

#[test]
pub fn same_height_different_parent() {
    let (mut layout, mut measurer) = test_utils();

    //      0
    //      1
    //     / \
    //    2   3
    //   /     \
    // 4-5     6-7

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
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![4, 5],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(1),
        vec![6, 7],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(2),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        5,
        Some(2),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        6,
        Some(3),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        7,
        Some(3),
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

    // 5 get 7 get invalidated, which will force Torin to find a root candidate that
    // is ascendant of both Nodes, in this case, Node 1
    //       0
    //      (1)
    //      / \
    //     2   3
    //    /     \
    //  4-[5]   6-[7]

    layout.invalidate(5);
    layout.invalidate(7);

    layout.find_best_root(&mut mocked_dom);

    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::Valid(1));
}

#[test]
pub fn same_height_same_parent() {
    let (mut layout, mut measurer) = test_utils();

    //      0
    //      1
    //     / \
    //    2   3
    //   /     \
    // 4-5     6-7

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
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![4, 5],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(1),
        vec![6, 7],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(2),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        5,
        Some(2),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        6,
        Some(3),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        7,
        Some(3),
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

    // 6 and 7 get invalidated, which will force Torin to find a root candidate that
    // is ascendant of both Nodes, in this case, Node 3
    //       0
    //       1
    //      / \
    //     2  (3)
    //    /     \
    //  4-5   [6]-[7]

    layout.invalidate(6);
    layout.invalidate(7);

    layout.find_best_root(&mut mocked_dom);

    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::Valid(3));
}
