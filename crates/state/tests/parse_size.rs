use freya_node_state::Parse;
use torin::{
    geometry::Length,
    size::{
        Dimension,
        DynamicCalculation,
        LexFunction,
        Size,
    },
};

#[test]
fn parse_pixel_size() {
    let size = Size::parse("123");
    assert_eq!(size, Ok(Size::Pixels(Length::new(123.0))));
}

#[test]
fn parse_relative_size() {
    let size = Size::parse("78.123%");
    assert_eq!(size, Ok(Size::Percentage(Length::new(78.123))));
}

#[test]
fn parse_auto_size() {
    let size = Size::parse("auto");
    assert_eq!(size, Ok(Size::Inner));
}

#[test]
fn parse_calc_size() {
    let size = Size::parse("calc(min(max(clamp(1, 2, 3), 4), parent.width + root.height - root.cross * 2 / 1, scale, parent, root, parent.height, parent.cross, +5.0, -3.0))");
    assert_eq!(
        size,
        Ok(Size::DynamicCalculations(Box::new(vec![
            DynamicCalculation::Function(LexFunction::Min),
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::Function(LexFunction::Max),
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::Function(LexFunction::Clamp),
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::Pixels(1.0),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Pixels(2.0),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Pixels(3.0),
            DynamicCalculation::ClosedParenthesis,
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Pixels(4.0),
            DynamicCalculation::ClosedParenthesis,
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Parent(Dimension::Width),
            DynamicCalculation::Add,
            DynamicCalculation::Root(Dimension::Height),
            DynamicCalculation::Sub,
            DynamicCalculation::Root(Dimension::Cross),
            DynamicCalculation::Mul,
            DynamicCalculation::Pixels(2.0),
            DynamicCalculation::Div,
            DynamicCalculation::Pixels(1.0),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::ScalingFactor,
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Parent(Dimension::Current),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Root(Dimension::Current),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Parent(Dimension::Height),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Parent(Dimension::Cross),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Add,
            DynamicCalculation::Pixels(5.0),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Sub,
            DynamicCalculation::Pixels(3.0),
            DynamicCalculation::ClosedParenthesis,
        ])))
    );
}
