#[cfg(test)]
use torin::{prelude::*, test_utils::*};

#[test]
pub fn margin() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_margin(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Gaps::new(5.0, 5.0, 5.0, 5.0),
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_margin(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Gaps::new(5.0, 5.0, 5.0, 5.0),
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    let layout_node = layout.get(1).unwrap();

    assert_eq!(
        layout_node.area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(210.0, 210.0)),
    );

    assert_eq!(
        layout_node.visible_area(),
        Rect::new(Point2D::new(5.0, 5.0), Size2D::new(200.0, 200.0)),
    );
}
