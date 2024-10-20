use euclid::Length;
use torin::{prelude::*, test_utils::*};

#[test]
pub fn unsized_parent_with_child_with_margin() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(Size::Inner, Size::Inner, DirectionMode::Vertical),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_margin(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            Gaps::new(10.0, 20.0, 30.0, 40.0),
        ),
    );

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(40.0, 10.0), Size2D::new(940.0, 960.0)),
    );
}

#[test]
pub fn unsized_parent_with_padding() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_padding(Size::Inner, Size::Inner, Gaps::new(10.0, 20.0, 30.0, 40.0)),
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
        layout.get(0).unwrap().visible_area().round(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area().round(),
        Rect::new(Point2D::new(40.0, 10.0), Size2D::new(940.0, 960.0)),
    );
}

#[test]
pub fn stacked() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );

    mocked_dom.set_node(
        2,
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(2);

    layout.find_best_root(&mut mocked_dom);

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );
}

#[test]
pub fn two_cols_auto() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Inner,
            Size::Pixels(Length::new(400.0)),
            DirectionMode::Horizontal,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );

    layout.find_best_root(&mut mocked_dom);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(400.0, 400.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(400.0, 400.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(200.0, 0.0), Size2D::new(200.0, 200.0)),
    );
}

#[test]
pub fn sibling_increments_area() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(Size::Inner, Size::Inner, DirectionMode::Vertical),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(300.0, 200.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(150.0, 100.0)),
    );
}

#[test]
pub fn root_100per_children_50per50per() {
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
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 500.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 500.0), Size2D::new(1000.0, 500.0)),
    );
}

#[test]
pub fn root_200px_children_50per50per() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Horizontal,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Horizontal,
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(200.0, 100.0)),
    );
}

#[test]
pub fn direction() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Vertical,
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
    mocked_dom.add(
        2,
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
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(0.0, 100.0), Size2D::new(100.0, 100.0)),
    );

    // Change the direction from vertical to horizontal

    mocked_dom.set_node(
        0,
        Node::from_size_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            DirectionMode::Horizontal,
        ),
    );
    layout.invalidate(0);

    layout.find_best_root(&mut mocked_dom);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(100.0, 0.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn scroll() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_scroll(
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
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
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
        layout.get(1).unwrap().area,
        Rect::new(Point2D::new(50.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().area,
        Rect::new(Point2D::new(50.0, 100.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn fill_size() {
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
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Pixels(Length::new(300.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Fill,
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
        layout.get(1).unwrap().visible_area().round(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 300.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area().round(),
        Rect::new(Point2D::new(0.0, 300.0), Size2D::new(1000.0, 700.0)),
    );
}

#[test]
pub fn root_percentage() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(300.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::RootPercentage(Length::new(75.0)),
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
        layout.get(1).unwrap().visible_area().round(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(500.0, 300.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area().round(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(750.0, 100.0)),
    );
}

#[test]
pub fn content_fit_fill_min() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2, 3],
        Node::from_size_and_content(
            Size::Inner,
            Size::Percentage(Length::new(100.0)),
            Content::Fit,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::FillMinimum,
            Size::Percentage(Length::new(30.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Percentage(Length::new(30.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::FillMinimum,
            Size::Percentage(Length::new(30.0)),
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
        layout.get(0).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 1000.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 300.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 300.0), Size2D::new(100.0, 300.0)),
    );

    assert_eq!(
        layout.get(3).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 600.0), Size2D::new(100.0, 300.0)),
    );
}

#[test]
pub fn inner_percentage() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1, 2],
        Node::from_size_and_direction(
            Size::Inner,
            Size::InnerPercentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Inner,
            Size::Percentage(Length::new(30.0)),
            DirectionMode::Vertical,
        ),
    );
    mocked_dom.add(
        2,
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
        layout.get(0).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 200.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(0.0, 300.0)),
    );

    assert_eq!(
        layout.get(2).unwrap().visible_area(),
        Rect::new(Point2D::new(0.0, 300.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn test_calc() {
    const PARENT_VALUE: f32 = 500.0;

    assert_eq!(
        run_calculations(
            &vec![DynamicCalculation::Pixels(10.0)],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        Some(10.0)
    );

    assert_eq!(
        run_calculations(
            &vec![DynamicCalculation::Percentage(87.5)],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        Some((87.5 / 100.0 * PARENT_VALUE).round())
    );

    assert_eq!(
        run_calculations(
            &vec![
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::Add,
                DynamicCalculation::Pixels(20.0),
                DynamicCalculation::Mul,
                DynamicCalculation::Percentage(50.0),
            ],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        Some(10.0 + 20.0 * (50.0 / 100.0 * PARENT_VALUE).round())
    );

    assert_eq!(
        run_calculations(
            &vec![
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::Add,
                DynamicCalculation::Percentage(10.0),
                DynamicCalculation::Add,
                DynamicCalculation::Pixels(30.0),
                DynamicCalculation::Mul,
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::Add,
                DynamicCalculation::Pixels(75.0),
                DynamicCalculation::Mul,
                DynamicCalculation::Pixels(2.0),
            ],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        Some(10.0 + (10.0 / 100.0 * PARENT_VALUE).round() + 30.0 * 10.0 + 75.0 * 2.0)
    );

    assert_eq!(
        run_calculations(
            &vec![
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::Pixels(20.0),
            ],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        None
    );

    assert_eq!(
        run_calculations(
            &vec![DynamicCalculation::Pixels(10.0), DynamicCalculation::Add],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        None
    );

    assert_eq!(
        run_calculations(
            &vec![DynamicCalculation::Add, DynamicCalculation::Pixels(10.0)],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        // becasue +10 is just 10
        Some(10.0)
    );

    assert_eq!(
        run_calculations(
            &vec![
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::Add,
                // counts as a prefix
                DynamicCalculation::Add,
                DynamicCalculation::Pixels(10.0)
            ],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        Some(20.0)
    );

    assert_eq!(
        run_calculations(
            &vec![
                DynamicCalculation::Percentage(50.0),
                DynamicCalculation::Sub,
                DynamicCalculation::RootPercentage(20.0)
            ],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        Some((PARENT_VALUE * 0.5) - (PARENT_VALUE * 0.20))
    );

    assert_eq!(
        run_calculations(
            &vec![
                DynamicCalculation::OpenParenthesis,
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::ClosedParenthesis,
            ],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        Some(10.0)
    );

    assert_eq!(
        run_calculations(
            &vec![
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::OpenParenthesis,
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::Add,
                DynamicCalculation::Pixels(20.0),
                DynamicCalculation::ClosedParenthesis,
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::Add,
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::OpenParenthesis,
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::ClosedParenthesis,
                DynamicCalculation::Pixels(10.0),
            ],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        Some((10.0 * (10.0 + 20.0) * 10.0) + (10.0 * (10.0) * 10.0))
    );

    assert_eq!(
        run_calculations(
            &vec![
                DynamicCalculation::Sub,
                DynamicCalculation::OpenParenthesis,
                DynamicCalculation::Pixels(10.0),
                DynamicCalculation::ClosedParenthesis,
                DynamicCalculation::Pixels(20.0)
            ],
            PARENT_VALUE,
            PARENT_VALUE
        ),
        Some(-1.0 * 10.0 * 20.0)
    );
}
