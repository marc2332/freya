use rustc_hash::FxHashSet;
#[cfg(test)]
use torin::{prelude::*, test_utils::*};

#[test]
pub fn caching() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_padding(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Gaps::new(5.0, 0.0, 0.0, 0.0),
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
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
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(200.0, 195.0)),
    );

    mocked_dom.set_node(
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(1);

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(100.0, 195.0)),
    );
}

#[test]
pub fn layout_dirty_nodes() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        None,
        vec![2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    // CASE 1
    // - Root is fixed
    // - Child A is fixed
    // - Child A[0] is fixed

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(50.0, 50.0)),
    );

    mocked_dom.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(10.0)),
            Size::Pixels(Length::new(10.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(2);

    assert_eq!(layout.get_dirty_nodes(), &FxHashSet::from_iter([2]));

    // CASE 2
    // Same as Case 1 but we make Child A depend on Child A[0]'s size

    mocked_dom.set_node(
        1,
        Node::from_size_and_direction(
            Size::Inner,
            Size::Pixels(Length::new(10.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(1);

    assert_eq!(layout.get_dirty_nodes(), &FxHashSet::from_iter([2, 1]));

    // CASE 3
    // Same as Case 2, but triggers a change in Child A[0]

    mocked_dom.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(2);

    assert_eq!(layout.get_dirty_nodes(), &FxHashSet::from_iter([2, 1]));

    // CASE 4
    // Same as Case 3, but triggers a change in the root

    mocked_dom.set_node(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(150.0)),
            Size::Pixels(Length::new(150.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(0);

    assert_eq!(layout.get_dirty_nodes(), &FxHashSet::from_iter([2, 1, 0]));
}

#[test]
pub fn node_removal() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2, 3],
        Node::from_size_and_direction(Size::Inner, Size::Inner, DirectionMode::Vertical),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![4],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(2),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(layout.size(), 5);

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 400.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 200.0), Size2D::new(200.0, 200.0)),
    );

    layout.remove(2, &mut mocked_dom, true);

    mocked_dom.remove(2);

    layout.find_best_root(&mut mocked_dom);

    assert_eq!(layout.get_dirty_nodes(), &FxHashSet::from_iter([1]));

    assert_eq!(layout.size(), 3);

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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );
}

#[test]
pub fn deep_tree() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![3],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(2),
        vec![4],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        4,
        Some(3),
        vec![5],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        5,
        Some(4),
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

    mocked_dom.set_node(
        4,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(200.0)),
            Size::Percentage(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(4);

    layout.find_best_root(&mut mocked_dom);
    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::Valid(4));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(layout.get_root_candidate(), RootNodeCandidate::None);
}
