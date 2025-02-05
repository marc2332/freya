use euclid::Length;
use torin::{
    prelude::*,
    test_utils::*,
};

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
            Direction::Horizontal,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
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
            Direction::Vertical,
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
            Direction::Vertical,
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(300.0)),
            Direction::Vertical,
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(300.0)),
            Direction::Vertical,
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
            Direction::Horizontal,
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction_and_margin(
            Size::Pixels(Length::new(150.0)),
            Size::Pixels(Length::new(80.0)),
            Direction::Vertical,
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_direction(Size::Inner, Size::Inner, Direction::Vertical),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
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

#[test]
pub fn space_between_alignment() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3, 4],
        Node::from_size_and_alignments_and_direction(
            Size::Percentage(Length::new(100.)),
            Size::Percentage(Length::new(100.)),
            Alignment::SpaceBetween,
            Alignment::Start,
            Direction::Horizontal,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(300.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(600.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(900.0, 0.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn space_around_alignment() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3, 4],
        Node::from_size_and_alignments_and_direction(
            Size::Percentage(Length::new(100.)),
            Size::Percentage(Length::new(100.)),
            Alignment::SpaceAround,
            Alignment::Start,
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 75.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 325.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 575.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 825.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn space_evenly_alignment() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3, 4],
        Node::from_size_and_alignments_and_direction(
            Size::Percentage(Length::new(100.)),
            Size::Percentage(Length::new(100.)),
            Alignment::SpaceEvenly,
            Alignment::Start,
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 120.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 340.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 560.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 780.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn alignment_with_absolute_child() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3],
        Node::from_size_and_alignments_and_direction_and_spacing(
            Size::Percentage(Length::new(100.)),
            Size::Percentage(Length::new(100.)),
            Alignment::Center,
            Alignment::Center,
            Direction::Vertical,
            Length::new(15.0),
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_position(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Position::Absolute(Box::default()),
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(450.0, 392.5), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(450.0, 507.5), Size2D::new(100.0, 100.0)),
    );
}
