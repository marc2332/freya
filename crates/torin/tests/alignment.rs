use euclid::Length;
use torin::{prelude::*, test_utils::*};

#[test]
pub fn display_horizontal() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_alignments_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Alignment::Center,
            Alignment::Center,
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
        Rect::new(Point2D::new(50.0, 50.0), Size2D::new(100.0, 100.0)),
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
        Node::from_size_and_alignments_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Alignment::Center,
            Alignment::Center,
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
        Rect::new(Point2D::new(50.0, 50.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(55.0, 55.0), Size2D::new(90.0, 90.0)),
    );
}

#[test]
pub fn double_center_alignment() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_alignments_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Alignment::Center,
            Alignment::Center,
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(300.0)),
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
        Rect::new(Point2D::new(400.0, 250.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(350.0, 450.0), Size2D::new(300.0, 300.0)),
    );
}

#[test]
pub fn double_end_alignment() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_alignments_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Alignment::End,
            Alignment::End,
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(300.0)),
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
        Rect::new(Point2D::new(800.0, 500.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(700.0, 700.0), Size2D::new(300.0, 300.0)),
    );
}

#[test]
pub fn unsized_alignment() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_alignments_and_direction_and_padding(
            Size::Inner,
            Size::Inner,
            Alignment::Center,
            Alignment::End,
            DirectionMode::Horizontal,
            Gaps::new(15.0, 15.0, 15.0, 15.0),
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction_and_margin(
            Size::Pixels(Length::new(150.0)),
            Size::Pixels(Length::new(80.0)),
            DirectionMode::Vertical,
            Gaps::new(10.0, 50.0, 20.0, 0.0),
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(350.0, 190.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(15.0, 75.0), Size2D::new(100.0, 50.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(115.0, 25.0), Size2D::new(150.0, 80.0)),
    );
}

#[test]
pub fn nested_unsized_alignment() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_alignments_and_direction(
            Size::Percentage(Length::new(100.)),
            Size::Percentage(Length::new(100.)),
            Alignment::Center,
            Alignment::Center,
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_direction(Size::Inner, Size::Inner, DirectionMode::Vertical),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(50.0, 50.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(50.0, 50.0), Size2D::new(100.0, 100.0)),
    );
}
