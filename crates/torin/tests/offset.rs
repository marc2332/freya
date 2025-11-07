use rustc_hash::FxHashMap;
use torin::{
    prelude::*,
    test_utils::*,
};

#[test]
pub fn offset_change() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingTree::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(Size::Inner, Size::Inner, Direction::Vertical),
    );
    mocked_dom.add(
        1,
        None,
        vec![2],
        Node::from_size_and_offset(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Length::new(0.),
            Length::new(100.),
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
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
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(50.0, 50.0)),
    );

    mocked_dom.set_node(
        1,
        Node::from_size_and_offset(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Length::new(0.),
            Length::new(150.),
        ),
    );
    layout.invalidate_with_reason(1, DirtyReason::InnerLayout);

    layout.find_best_root(&mut mocked_dom);

    assert_eq!(
        layout.get_dirty_nodes(),
        &FxHashMap::from_iter([(1, DirtyReason::InnerLayout)])
    );
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 150.0), Size2D::new(50.0, 50.0)),
    );
}

#[test]
pub fn offset_change_and_nested_changed() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingTree::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 3],
        Node::from_size_and_direction(Size::Inner, Size::Inner, Direction::Vertical),
    );
    mocked_dom.add(
        1,
        None,
        vec![2],
        Node::from_size_and_offset(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Length::new(0.),
            Length::new(100.),
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        3,
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
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 200.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(50.0, 50.0)),
    );

    assert_eq!(
        layout.get(&3).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    mocked_dom.set_node(
        1,
        Node::from_size_and_offset(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Length::new(0.),
            Length::new(150.),
        ),
    );
    layout.invalidate_with_reason(1, DirtyReason::InnerLayout);
    mocked_dom.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
    );
    layout.invalidate_with_reason(2, DirtyReason::None);

    layout.find_best_root(&mut mocked_dom);

    assert_eq!(layout.get_dirty_nodes().len(), 2);
    assert_eq!(layout.get_dirty_nodes().get(&1), Some(&DirtyReason::None));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(&0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 200.0)),
    );

    assert_eq!(
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(0.0, 150.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(&3).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn offset() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingTree::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_offset(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Length::new(50.0),
            Length::new(0.0),
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
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
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
        layout.get(&1).unwrap().area,
        Rect::new(Point2D::new(50.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(&2).unwrap().area,
        Rect::new(Point2D::new(50.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}
