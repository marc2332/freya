use euclid::Rect;
use torin::{
    node::Node,
    prelude::{Alignment, Direction, Length, Point2D, Size2D},
    size::Size,
    test_utils::{test_utils, TestingDOM},
    wrap_content::WrapContent,
};

#[test]
pub fn basic_wrapping() {}

#[test]
pub fn wrapping_space_between() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::SpaceBetween,
        Alignment::Start,
        Direction::Horizontal,
    );
    parent.wrap_content = WrapContent::Wrap;
    mocked_dom.add(0, None, vec![1, 2, 3, 4], parent);
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
            Size::Pixels(Length::new(50.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(250.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(150.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(200.0, 100.0), Size2D::new(50.0, 100.0)),
    );
}

#[test]
pub fn wrapping_space_between_cross() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::Start,
        Alignment::SpaceBetween,
        Direction::Horizontal,
    );
    parent.wrap_content = WrapContent::Wrap;
    mocked_dom.add(0, None, vec![1, 2, 3, 4], parent);
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
            Size::Pixels(Length::new(50.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(250.0, 250.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 150.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(100.0, 150.0), Size2D::new(100.0, 50.0)),
    );
}

#[test]
pub fn wrapping_center() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::Center,
        Alignment::Start,
        Direction::Horizontal,
    );
    parent.wrap_content = WrapContent::Wrap;
    mocked_dom.add(0, None, vec![1, 2, 3, 4], parent);
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
            Size::Pixels(Length::new(50.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(250.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(25.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(125.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(50.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(150.0, 100.0), Size2D::new(50.0, 100.0)),
    );
}

#[test]
pub fn wrapping_center_cross() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::Start,
        Alignment::Center,
        Direction::Horizontal,
    );
    parent.wrap_content = WrapContent::Wrap;
    mocked_dom.add(0, None, vec![1, 2, 3, 4], parent);
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
            Size::Pixels(Length::new(50.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(250.0, 250.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 25.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(100.0, 25.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 125.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(100.0, 150.0), Size2D::new(100.0, 50.0)),
    );
}

#[test]
pub fn wrapping_end() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::End,
        Alignment::Start,
        Direction::Horizontal,
    );
    parent.wrap_content = WrapContent::Wrap;
    mocked_dom.add(0, None, vec![1, 2, 3, 4], parent);
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
            Size::Pixels(Length::new(50.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(250.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(50.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(150.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(100.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(200.0, 100.0), Size2D::new(50.0, 100.0)),
    );
}

#[test]
pub fn wrapping_end_cross() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::Start,
        Alignment::End,
        Direction::Horizontal,
    );
    parent.wrap_content = WrapContent::Wrap;
    mocked_dom.add(0, None, vec![1, 2, 3, 4], parent);
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
            Size::Pixels(Length::new(50.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(25.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(250.0, 250.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(100.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 200.0), Size2D::new(100.0, 50.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(100.0, 225.0), Size2D::new(100.0, 25.0)),
    );
}

#[test]
pub fn flex_wrapping() {}

#[test]
pub fn flex_min_width_wrapping() {}

#[test]
pub fn relative_children_wrapping() {}

#[test]
pub fn wrapping_padding_margin() {}

#[test]
pub fn wrapping_large_single_child() {}

#[test]
pub fn wrapping_unsized_container() {}

#[test]
pub fn wrapping_unsized_children() {}

#[test]
pub fn wrapping_absolute_child() {}
