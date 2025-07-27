use euclid::Rect;
use torin::{
    node::Node,
    prelude::{Alignment, Direction, Length, Point2D, Size2D},
    size::Size,
    test_utils::{test_utils, TestingDOM},
    wrap_content::WrapContent,
};

fn wrapping_base_test(
    main_alignment: Alignment,
    cross_alignment: Alignment,
    root_width: f32,
    root_height: f32,
    child_origins: [Point2D; 4],
) {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    let mut parent = Node::from_size_and_alignments_and_direction(
        Size::Percentage(Length::new(100.)),
        Size::Percentage(Length::new(100.)),
        main_alignment,
        cross_alignment,
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
            Size::Pixels(Length::new(100.)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(root_width, root_height)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(child_origins[0], Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(child_origins[1], Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(child_origins[2], Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(4).unwrap().visible_area(),
        Rect::new(child_origins[3], Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn basic_wrapping() {}

#[test]
pub fn wrapping_space_between() {
    wrapping_base_test(
        Alignment::SpaceBetween,
        Alignment::Start,
        250.0,
        1000.0,
        [
            Point2D::new(0.0, 0.0),
            Point2D::new(150.0, 0.0),
            Point2D::new(0.0, 100.0),
            Point2D::new(150.0, 100.0),
        ],
    );
}

#[test]
pub fn wrapping_center() {
    wrapping_base_test(
        Alignment::Center,
        Alignment::Start,
        250.0,
        1000.0,
        [
            Point2D::new(25.0, 0.0),
            Point2D::new(125.0, 0.0),
            Point2D::new(25.0, 100.0),
            Point2D::new(125.0, 100.0),
        ],
    );
}

#[test]
pub fn wrapping_center_cross() {
    wrapping_base_test(
        Alignment::Start,
        Alignment::Center,
        250.0,
        250.0,
        [
            Point2D::new(0.0, 25.0),
            Point2D::new(100.0, 25.0),
            Point2D::new(0.0, 125.0),
            Point2D::new(100.0, 125.0),
        ],
    );
}

#[test]
pub fn wrapping_end() {}

#[test]
pub fn flex_wrapping() {}

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
