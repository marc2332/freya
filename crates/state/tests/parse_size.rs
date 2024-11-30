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
    let size = Size::parse("calc(90%- 5%* 123.6/ 50v(5 + 6) - min(50v, 50v', 100%, 100%') max(50v) clamp(50v, 5.0, 100%))");
    assert_eq!(
        size,
        Ok(Size::DynamicCalculations(Box::new(vec![
            DynamicCalculation::Percentage(Dimension::Current(90.0)),
            DynamicCalculation::Sub,
            DynamicCalculation::Percentage(Dimension::Current(5.0)),
            DynamicCalculation::Mul,
            DynamicCalculation::Pixels(123.6),
            DynamicCalculation::Div,
            DynamicCalculation::RootPercentage(Dimension::Current(50.0)),
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::Pixels(5.0),
            DynamicCalculation::Add,
            DynamicCalculation::Pixels(6.0),
            DynamicCalculation::ClosedParenthesis,
            DynamicCalculation::Sub,
            DynamicCalculation::Function(LexFunction::Min),
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::RootPercentage(Dimension::Current(50.0)),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::RootPercentage(Dimension::Other(50.0)),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Percentage(Dimension::Current(100.0)),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Percentage(Dimension::Other(100.0)),
            DynamicCalculation::ClosedParenthesis,
            DynamicCalculation::Function(LexFunction::Max),
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::RootPercentage(Dimension::Current(50.0)),
            DynamicCalculation::ClosedParenthesis,
            DynamicCalculation::Function(LexFunction::Clamp),
            DynamicCalculation::OpenParenthesis,
            DynamicCalculation::RootPercentage(Dimension::Current(50.0)),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Pixels(5.0),
            DynamicCalculation::FunctionSeparator,
            DynamicCalculation::Percentage(Dimension::Current(100.0)),
            DynamicCalculation::ClosedParenthesis,
        ])))
    );
}
