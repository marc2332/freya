#[cfg(test)]
use torin::{prelude::*, test_utils::*};

#[test]
pub fn position() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_padding(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Gaps::new(20.0, 20.0, 20.0, 20.0),
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2, 3, 4, 5],
        Node::from_size_and_padding(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Gaps::new(30.0, 30.0, 30.0, 30.0),
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Absolute(Box::new(AbsolutePosition {
                top: Some(100.0),
                right: None,
                bottom: None,
                left: Some(50.0),
            })),
        ),
    );
    mocked_dom.add(
        3,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Absolute(Box::new(AbsolutePosition {
                top: Some(100.0),
                right: Some(50.0),
                bottom: None,
                left: None,
            })),
        ),
    );
    mocked_dom.add(
        4,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Absolute(Box::new(AbsolutePosition {
                top: None,
                right: Some(50.0),
                bottom: Some(100.0),
                left: None,
            })),
        ),
    );
    mocked_dom.add(
        5,
        Some(1),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Position::Absolute(Box::new(AbsolutePosition {
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
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(100.0, 150.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(3).unwrap().area.round(),
        Rect::new(Point2D::new(700.0, 150.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(4).unwrap().area.round(),
        Rect::new(Point2D::new(700.0, 650.0), Size2D::new(200.0, 200.0)),
    );
    assert_eq!(
        layout.get(5).unwrap().area.round(),
        Rect::new(Point2D::new(100.0, 650.0), Size2D::new(200.0, 200.0)),
    );
}
