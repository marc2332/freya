use euclid::Length;
use torin::{
    prelude::*,
    test_utils::*,
};

#[test]
pub fn unsized_parent_with_child_with_margin() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_size_and_direction(Size::Inner, Size::Inner, Direction::Vertical),
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
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
            Direction::Vertical,
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
            Direction::Horizontal,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(200.0)),
            Direction::Vertical,
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
        Node::from_size_and_direction(Size::Inner, Size::Inner, Direction::Vertical),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(300.0)),
            Size::Pixels(Length::new(100.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            Direction::Horizontal,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Percentage(Length::new(50.0)),
            Direction::Horizontal,
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
            Direction::Vertical,
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
            Direction::Horizontal,
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Pixels(Length::new(300.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(100.0)),
            Size::Fill,
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![2],
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Pixels(Length::new(300.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(1),
        vec![],
        Node::from_size_and_direction(
            Size::RootPercentage(Length::new(75.0)),
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
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        2,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Percentage(Length::new(30.0)),
            Direction::Vertical,
        ),
    );
    mocked_dom.add(
        3,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::FillMinimum,
            Size::Percentage(Length::new(30.0)),
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
        Node::from_size_and_visible_size(
            Size::Inner,
            Size::Inner,
            VisibleSize::Full,
            VisibleSize::InnerPercentage(Length::new(50.0)),
        ),
    );
    mocked_dom.add(
        1,
        Some(0),
        vec![],
        Node::from_size_and_direction(
            Size::Inner,
            Size::Percentage(Length::new(30.0)),
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
fn test_calc_and_scaling() {
    const PARENT_VALUE: f32 = 500.0;
    const ROOT_VALUE: f32 = 1000.0;
    const SCALING_FACTOR: f32 = 2.0;
    const PARENT_MARGIN: f32 = 0.0;

    #[track_caller]
    fn test_calc_with_scaling(
        calcs: Vec<DynamicCalculation>,
        expected_unscaled: Option<f32>,
        expected_scaled: Option<f32>,
    ) {
        let caller_location = std::panic::Location::caller();
        let mut size = Size::DynamicCalculations(1.0, Box::new(calcs.clone()));

        let unscaled = size.eval(
            PARENT_VALUE,
            PARENT_VALUE,
            PARENT_MARGIN,
            ROOT_VALUE,
            Phase::Initial,
        );
        assert_eq!(
            unscaled,
            expected_unscaled,
            "Assertion failed in test_calc_with_scaling at {}:{}
            Test case: {:?}
            Unscaled calculation failed
            Expected: {:?}, Got: {:?}",
            caller_location.file(),
            caller_location.line(),
            calcs,
            expected_unscaled,
            unscaled
        );

        size.scale(SCALING_FACTOR);

        let scaled = size.eval(
            PARENT_VALUE,
            PARENT_VALUE,
            PARENT_MARGIN,
            ROOT_VALUE,
            Phase::Initial,
        );
        assert_eq!(
            scaled,
            expected_scaled,
            "Assertion failed in test_calc_with_scaling at {}:{}
                Test case: {:?}
                Scaled calculation failed
                Expected: {:?}, Got: {:?}",
            caller_location.file(),
            caller_location.line(),
            calcs,
            expected_scaled,
            scaled
        );
    }

    test_calc_with_scaling(
        vec![DynamicCalculation::Pixels(10.0)],
        Some(10.0),
        Some(10.0 * SCALING_FACTOR),
    );

    test_calc_with_scaling(
        vec![DynamicCalculation::Percentage(10.0)],
        Some((10.0 / 100.0 * PARENT_VALUE).round()),
        Some((10.0 / 100.0 * PARENT_VALUE).round()),
    );

    test_calc_with_scaling(
        vec![
            DynamicCalculation::Pixels(10.0),
            DynamicCalculation::Add,
            DynamicCalculation::Pixels(20.0),
            DynamicCalculation::Mul,
            DynamicCalculation::Percentage(50.0),
        ],
        Some(10.0 + 20.0 * (50.0 / 100.0 * PARENT_VALUE).round()),
        Some(10.0 * SCALING_FACTOR + (20.0 * (50.0 / 100.0 * PARENT_VALUE).round())),
    );

    test_calc_with_scaling(
        vec![
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
        Some(10.0 + (10.0 / 100.0 * PARENT_VALUE).round() + 30.0 * 10.0 + 75.0 * 2.0),
        Some(
            10.0 * SCALING_FACTOR
                + (10.0 / 100.0 * PARENT_VALUE).round()
                + 30.0 * 10.0 * SCALING_FACTOR
                + 75.0 * 2.0 * SCALING_FACTOR,
        ),
    );

    test_calc_with_scaling(
        vec![
            DynamicCalculation::Pixels(10.0),
            DynamicCalculation::Pixels(20.0),
        ],
        Some(0.0),
        Some(0.0),
    );

    test_calc_with_scaling(
        vec![DynamicCalculation::Pixels(10.0), DynamicCalculation::Add],
        Some(0.0),
        Some(0.0),
    );

    test_calc_with_scaling(
        vec![DynamicCalculation::Add, DynamicCalculation::Pixels(10.0)],
        Some(10.0),
        Some(10.0 * SCALING_FACTOR),
    );

    test_calc_with_scaling(
        vec![
            DynamicCalculation::Pixels(10.0),
            DynamicCalculation::Add,
            DynamicCalculation::Add,
            DynamicCalculation::Pixels(10.0),
        ],
        Some(20.0),
        Some(20.0 * SCALING_FACTOR),
    );

    test_calc_with_scaling(
        vec![
            DynamicCalculation::Percentage(50.0),
            DynamicCalculation::Sub,
            DynamicCalculation::RootPercentage(20.0),
        ],
        Some((PARENT_VALUE * 0.5) - (ROOT_VALUE * 0.20)),
        Some((PARENT_VALUE * 0.5) - (ROOT_VALUE * 0.20)),
    );

    test_calc_with_scaling(
        vec![
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::Pixels(10.0),
            DynamicCalculation::ClosedParenthesis,
        ],
        Some(10.0),
        Some(10.0 * SCALING_FACTOR),
    );

    test_calc_with_scaling(
        vec![
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::Pixels(10.0),
        ],
        Some(0.0),
        Some(0.0),
    );

    // Example 1: calc(2 * 5)
    test_calc_with_scaling(
        vec![
            DynamicCalculation::Pixels(2.0),
            DynamicCalculation::Mul,
            DynamicCalculation::Pixels(5.0),
        ],
        Some(2.0 * 5.0),                    // unscaled: 2 * 5 = 10
        Some((2.0 * 5.0) * SCALING_FACTOR), // scaled: (2 * 5) * 2 = 20
    );

    // Example 2: calc(25% * 2)
    test_calc_with_scaling(
        vec![
            DynamicCalculation::Percentage(25.0),
            DynamicCalculation::Mul,
            DynamicCalculation::Pixels(2.0),
        ],
        Some((25.0 / 100.0 * PARENT_VALUE).round() * 2.0), // unscaled: 25% of 500 * 2
        Some((25.0 / 100.0 * PARENT_VALUE).round() * 2.0), // scaled: 25% of 500 * 2 (percentage not scaled)
    );

    // Example 3: calc(5 + 2 * 3 + 25% * 3)
    test_calc_with_scaling(
        vec![
            DynamicCalculation::Pixels(5.0),
            DynamicCalculation::Add,
            DynamicCalculation::Pixels(2.0),
            DynamicCalculation::Mul,
            DynamicCalculation::Pixels(3.0),
            DynamicCalculation::Add,
            DynamicCalculation::Percentage(25.0),
            DynamicCalculation::Mul,
            DynamicCalculation::Pixels(3.0),
        ],
        Some(5.0 + 2.0 * 3.0 + (25.0 / 100.0 * PARENT_VALUE).round() * 3.0), // unscaled
        Some(
            5.0 * SCALING_FACTOR
                + (2.0 * 3.0) * SCALING_FACTOR
                + (25.0 / 100.0 * PARENT_VALUE).round() * 3.0,
        ), // scaled
    );

    // Example 4: calc(2 * (25% * 2 + 10 * 3 + 2))
    test_calc_with_scaling(
        vec![
            DynamicCalculation::Pixels(2.0),
            DynamicCalculation::Mul,
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::Percentage(25.0),
            DynamicCalculation::Mul,
            DynamicCalculation::Pixels(2.0),
            DynamicCalculation::Add,
            DynamicCalculation::Pixels(10.0),
            DynamicCalculation::Mul,
            DynamicCalculation::Pixels(3.0),
            DynamicCalculation::Add,
            DynamicCalculation::Pixels(2.0),
            DynamicCalculation::ClosedParenthesis,
        ],
        Some(2.0 * ((25.0 / 100.0 * PARENT_VALUE).round() * 2.0 + 10.0 * 3.0 + 2.0)), // unscaled
        Some(
            2.0 * (25.0 / 100.0 * PARENT_VALUE).round() * 2.0
                + (10.0 * 3.0 + 2.0) * SCALING_FACTOR * 2.0,
        ), // scaled with new logic
    );
}

#[test]
pub fn inner_min_max_sizes() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![1],
        Node::from_sizes(
            Size::Inner,
            Size::Inner,
            Size::Inner,
            Size::Inner,
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
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

    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().visible_area().round(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );

    assert_eq!(
        layout.get(1).unwrap().visible_area().round(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(200.0, 200.0)),
    );
}

#[test]
pub fn fixed_min_max_sizes() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![],
        Node::from_sizes(
            Size::Pixels(Length::new(150.)),
            Size::Pixels(Length::new(150.)),
            Size::Inner,
            Size::Inner,
            Size::Pixels(Length::new(100.)),
            Size::Pixels(Length::new(100.)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(100.0, 100.0)),
    );
}

#[test]
pub fn relative_min_max_sizes() {
    let (mut layout, mut measurer) = test_utils();

    let mut mocked_dom = TestingDOM::default();
    mocked_dom.add(
        0,
        None,
        vec![],
        Node::from_sizes(
            Size::Inner,
            Size::Pixels(Length::new(250.)),
            Size::Inner,
            Size::Percentage(Length::new(50.)),
            Size::Pixels(Length::new(100.)),
            Size::Percentage(Length::new(70.)),
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
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(0.0, 500.0)),
    );

    layout.invalidate(0);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 400.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().visible_area().round(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(0.0, 250.0)),
    );

    layout.invalidate(0);
    layout.measure(
        0,
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 200.0)),
        &mut measurer,
        &mut mocked_dom,
    );

    assert_eq!(
        layout.get(0).unwrap().visible_area().round(),
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(0.0, 140.0)),
    );
}
