use std::collections::HashSet;

use euclid::*;
use torin::*;

#[derive(Clone, Default, Debug)]
pub struct EmbeddedData {
    _text: Option<String>,
}

struct SkiaTextMeasurer;

impl LayoutMeasurer<usize, EmbeddedData> for SkiaTextMeasurer {
    fn measure(
        &mut self,
        _node: &NodeData<usize, EmbeddedData>,
        _area: &Rect<f32, Measure>,
        _parent_size: &Rect<f32, Measure>,
        _available_parent_size: &Rect<f32, Measure>,
    ) -> Option<Rect<f32, Measure>> {
        None
    }
}

fn test_utils() -> (Torin<usize, EmbeddedData>, Option<SkiaTextMeasurer>) {
    let layout = Torin::<usize, EmbeddedData>::new();
    let measurer = Some(SkiaTextMeasurer);

    (layout, measurer)
}

#[test]
pub fn root_100per_children_50per50per() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        None,
        vec![1, 2],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            Direction::Horizontal,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.insert(
        2,
        0,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 500.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 500.0), Size2D::new(1000.0, 500.0)),
    );
}

#[test]
pub fn root_200px_children_50per50per() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        None,
        vec![1, 2],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            Direction::Horizontal,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.insert(
        2,
        0,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            Direction::Horizontal,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );
}

#[test]
pub fn layout_dirty_nodes() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        None,
        vec![1],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![2],
    );

    layout.insert(
        2,
        1,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    // CASE 1
    // - Root is fixed
    // - Child A is fixed
    // - Child A[0] is fixed

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(50.0, 50.0)),
    );

    layout.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(10.0)),
            Size::Pixels(Length::new(10.0)),
            Direction::Vertical,
        ),
    );

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2]));

    // CASE 2
    // Same as Case 1 but we make Child A depend on Child A[0]'s size

    layout.set_node(
        1,
        Node::from_size_and_direction(
            Size::Inner,
            Size::Pixels(Length::new(10.0)),
            Direction::Vertical,
        ),
    );

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2, 1]));

    // CASE 3
    // Same as Case 2, but triggers a change in Child A[0]

    layout.set_node(
        2,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(50.0)),
            Size::Pixels(Length::new(50.0)),
            Direction::Vertical,
        ),
    );

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2, 1]));

    // CASE 4
    // Same as Case 3, but triggers a change in the root

    layout.set_node(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(150.0)),
            Size::Pixels(Length::new(150.0)),
            Direction::Vertical,
        ),
    );

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([2, 1, 0]));
}

#[test]
pub fn direction() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        None,
        vec![1, 2],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.insert(
        2,
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    // Change the direction from vertical to horizontal

    layout.set_node(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Horizontal,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn scroll() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_scroll(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Length::new(50.0),
            Length::new(0.0),
        ),
        EmbeddedData::default(),
        None,
        vec![1, 2],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.insert(
        2,
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(50.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(50.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn padding() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_padding(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            (
                Length::new(5.0),
                Length::new(10.0),
                Length::new(15.0),
                Length::new(20.0),
            ),
        ),
        EmbeddedData::default(),
        None,
        vec![1],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(20.0, 5.0), Size2D::new(170.0, 180.0)),
    );
}

#[test]
pub fn caching() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_padding(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            (
                Length::new(5.0),
                Length::new(0.0),
                Length::new(0.0),
                Length::new(0.0),
            ),
        ),
        EmbeddedData::default(),
        None,
        vec![1],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(200.0, 195.0)),
    );

    layout.set_node(
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Percentage(Length::new(100.0)),
            Direction::Vertical,
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 5.0), Size2D::new(100.0, 195.0)),
    );
}

#[test]
pub fn sibling_increments_area() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_direction(Size::Inner, Size::Inner, Direction::Vertical),
        EmbeddedData::default(),
        None,
        vec![1, 2],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.insert(
        2,
        0,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(300.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(150.0, 100.0)),
    );
}

#[test]
pub fn node_removal() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        None,
        vec![1],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(Size::Inner, Size::Inner, Direction::Vertical),
        EmbeddedData::default(),
        vec![2, 3],
    );

    layout.insert(
        2,
        1,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.insert(
        3,
        1,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 400.0)),
    );

    assert_eq!(
        layout.get_size(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 200.0), Size2D::new(200.0, 200.0)),
    );

    layout.remove(2);

    assert_eq!(layout.get_dirty_nodes(), &HashSet::from([1, 3]));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(3).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );
}

#[test]
pub fn display_horizontal() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_display_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Display::Center,
            Direction::Horizontal,
        ),
        EmbeddedData::default(),
        None,
        vec![1],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(50.0, 0.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn display_vertical_with_inner_children() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_display_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Display::Center,
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        None,
        vec![1],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_padding(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            (
                Length::new(5.0),
                Length::new(5.0),
                Length::new(5.0),
                Length::new(5.0),
            ),
        ),
        EmbeddedData::default(),
        vec![2],
    );

    layout.insert(
        2,
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(
        layout.get_size(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get_size(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 50.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get_size(2).unwrap().area,
        Rect::new(Point2D::new(5.0, 55.0), Size2D::new(90.0, 90.0)),
    );
}

#[test]
pub fn deep_tree() {
    let (mut layout, mut measurer) = test_utils();

    layout.add(
        0,
        Node::from_size_and_display_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Display::Center,
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        None,
        vec![1],
    );

    layout.insert(
        1,
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![2],
    );

    layout.insert(
        2,
        1,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![3],
    );

    layout.insert(
        3,
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![4],
    );

    layout.insert(
        4,
        3,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![5],
    );

    layout.insert(
        5,
        4,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Direction::Vertical,
        ),
        EmbeddedData::default(),
        vec![],
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    layout.set_node(
        4,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(200.0)),
            Size::Percentage(Length::new(200.0)),
            Direction::Vertical,
        ),
    );

    assert_eq!(layout.get_tallest_dirty_node(), Some(4));

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
    );

    assert_eq!(layout.get_tallest_dirty_node(), None);
}
