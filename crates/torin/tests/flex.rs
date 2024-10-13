use euclid::Length;
use torin::{
    prelude::*,
    test_utils::*,
};

#[test]
pub fn flex_generic() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3, 4],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Percentage(Length::new(10.)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(1.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(3.0)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 20.0)),
    );
    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 20.0), Size2D::new(100.0, 32.5)),
    );
    assert_eq!(
        layout.get(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 52.5), Size2D::new(100.0, 50.0)),
    );
    assert_eq!(
        layout.get(4).unwrap().area,
        Rect::new(Point2D::new(0.0, 102.5), Size2D::new(100.0, 97.5)),
    );
}

#[test]
pub fn flex_under_1_flex_grow() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(0.2)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(0.5)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 40.0)),
    );
    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 40.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn flex_grow_balance() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3, 4],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(1.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(2.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(3.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(4.0)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 20.0)),
    );
    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 20.0), Size2D::new(100.0, 40.0)),
    );
    assert_eq!(
        layout.get(3).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 60.0), Size2D::new(100.0, 60.0)),
    );
    assert_eq!(
        layout.get(4).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 120.0), Size2D::new(100.0, 80.0)),
    );
}

#[test]
pub fn flex_large_grow_balance() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3, 4],
        Node::from_size_and_content(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Content::Flex,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(5.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(65.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(30.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(100.0)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 5.0)),
    );
    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(100.0, 65.0)),
    );
    assert_eq!(
        layout.get(3).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 70.0), Size2D::new(100.0, 30.0)),
    );
    assert_eq!(
        layout.get(4).unwrap().area.round(),
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}
