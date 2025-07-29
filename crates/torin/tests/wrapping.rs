use euclid::Rect;
use torin::{
    content::Content,
    node::Node,
    position::Position,
    prelude::{
        Alignment,
        Direction,
        Length,
        Point2D,
        Size2D,
    },
    size::Size,
    test_utils::{
        test_utils,
        TestingDOM,
    },
    wrap_content::WrapContent,
};

#[test]
pub fn basic_wrapping() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::Start,
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
            Size::Pixels(Length::new(75.)),
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
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(75.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(75.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}

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
pub fn flex_without_minimum_width_never_wraps() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_content(
        Size::Pixels(Length::new(200.0)),
        Size::Pixels(Length::new(200.0)),
        Content::Flex,
    );
    parent.wrap_content = WrapContent::Wrap;
    mocked_dom.add(0, None, vec![1, 2, 3, 4], parent);
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Percentage(Length::new(10.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(1.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(50.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(3.0)),
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
pub fn flex_min_width_wrapping() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_content(
        Size::Pixels(Length::new(200.0)),
        Size::Pixels(Length::new(200.0)),
        Content::Flex,
    );
    parent.wrap_content = WrapContent::Wrap;
    mocked_dom.add(0, None, vec![1, 2, 3, 4], parent);
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Percentage(Length::new(10.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_sizes(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(1.0)),
            Size::Inner,
            Size::Pixels(Length::new(35.0)),
            Size::Inner,
            Size::Inner,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(50.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(0),
        vec![],
        Node::from_sizes(
            Size::Pixels(Length::new(100.0)),
            Size::Flex(Length::new(3.0)),
            Size::Inner,
            Size::Pixels(Length::new(100.0)),
            Size::Inner,
            Size::Inner,
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
        Rect::new(Point2D::new(0.0, 20.0), Size2D::new(100.0, 130.0)),
    );
    assert_eq!(
        layout.get(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 150.0), Size2D::new(100.0, 50.0)),
    );
    assert_eq!(
        layout.get(4).unwrap().area,
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(100.0, 200.0)),
    );
}

#[test]
pub fn wrapping_padding() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction_and_spacing(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::Center,
        Alignment::Start,
        Direction::Horizontal,
        Length::new(15.0),
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
            Size::Pixels(Length::new(75.)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(250.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(17.5, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(132.5, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(30.0, 100.0), Size2D::new(75.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(120.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn wrapping_large_single_child() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::Start,
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
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(100.)),
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
        layout.get(0).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(250.0, 250.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(300.0, 100.0)),
    );
}

#[test]
pub fn wrapping_unsized_parent() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Inner,
        Size::Inner,
        Alignment::Start,
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
            Size::Percentage(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(75.)),
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
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(250.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 200.0), Size2D::new(75.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(75.0, 200.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn wrapping_absolute_child() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        Alignment::Start,
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
        Node::from_size_and_position(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
            Position::Absolute(Box::default()),
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
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn wrapping_last_line_single_child_cross_align_and_overflow() {
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
    mocked_dom.add(0, None, vec![1, 2, 3], parent);
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(200.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(200.)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(200.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(250.0, 350.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, -25.0), Size2D::new(100.0, 200.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(100.0, -25.0), Size2D::new(100.0, 200.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 175.0), Size2D::new(100.0, 200.0)),
    );
}
