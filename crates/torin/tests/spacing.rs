use torin::{
    prelude::*,
    test_utils::*,
};

#[test]
pub fn spacing() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction_and_spacing(
            Size::Fill,
            Size::Fill,
            DirectionMode::Vertical,
            Length::new(40.0),
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Horizontal,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![3, 4],
        Node::from_size_and_direction_and_spacing(
            Size::Pixels(Length::new(600.0)),
            Size::Pixels(Length::new(600.0)),
            DirectionMode::Horizontal,
            Length::new(50.0),
        ),
    );
    mocked_dom.add(
        3,
        Some(2),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(300.0)),
            DirectionMode::Horizontal,
        ),
    );
    mocked_dom.add(
        4,
        Some(2),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Horizontal,
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 240.0), Size2D::new(600.0, 600.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 240.0), Size2D::new(300.0, 300.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().area,
        Rect::new(Point2D::new(350.0, 240.0), Size2D::new(200.0, 200.0)),
    );
}
