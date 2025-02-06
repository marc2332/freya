use euclid::Length;
use torin::{
    prelude::*,
    test_utils::*,
};

#[test]
pub fn grid_generic() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3],
        Node::from_size_and_content(
            Size::Pixels(Length::new(600.0)),
            Size::Pixels(Length::new(600.0)),
            Content::Grid {
                columns: vec![
                    GridSize::Pixels(Length::new(100.0)),
                    GridSize::Weight(Length::new(1.0)),
                    GridSize::Weight(Length::new(1.0)),
                ],
                rows: vec![GridSize::Weight(Length::new(1.0))],
            },
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_grid_position(Size::Fill, Size::Fill, GridPosition::new(0, 1, 0, 1)),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_grid_position(Size::Fill, Size::Fill, GridPosition::new(1, 1, 0, 1)),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_grid_position(Size::Fill, Size::Fill, GridPosition::new(2, 1, 0, 1)),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(600.0, 600.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(600.0, 600.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 600.0)),
    );
    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(250.0, 600.0)),
    );
    assert_eq!(
        layout.get(3).unwrap().area,
        Rect::new(Point2D::new(350.0, 0.0), Size2D::new(250.0, 600.0)),
    );
}

#[test]
pub fn grid_with_auto_sized_column() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3],
        Node::from_size_and_content(
            Size::Pixels(Length::new(600.0)),
            Size::Pixels(Length::new(600.0)),
            Content::Grid {
                columns: vec![
                    GridSize::Pixels(Length::new(100.0)),
                    GridSize::Weight(Length::new(1.0)),
                    GridSize::Inner,
                ],
                rows: vec![GridSize::Weight(Length::new(1.0))],
            },
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_grid_position(Size::Fill, Size::Fill, GridPosition::new(0, 1, 0, 1)),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_grid_position(Size::Fill, Size::Fill, GridPosition::new(1, 1, 0, 1)),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![4],
        Node::from_size_and_grid_position(Size::Inner, Size::Fill, GridPosition::new(2, 1, 0, 1)),
    );
    mocked_dom.add(
        4,
        Some(3),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(300.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(600.0, 600.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(600.0, 600.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 600.0)),
    );
    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(200.0, 600.0)),
    );
    assert_eq!(
        layout.get(3).unwrap().area,
        Rect::new(Point2D::new(300.0, 0.0), Size2D::new(300.0, 600.0)),
    );
}
