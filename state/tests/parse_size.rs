use freya_node_state::parse_size;
use torin::{DynamicCalculation, Length, Size};

const SCALE_FACTOR: f32 = 1.0;

#[test]
fn parse_pixel_size() {
    let size = parse_size("123", SCALE_FACTOR);
    assert_eq!(size, Some(Size::Pixels(Length::new(123.0))));
}

#[test]
fn parse_relative_size() {
    let size = parse_size("78.123%", SCALE_FACTOR);
    assert_eq!(size, Some(Size::Percentage(Length::new(78.123))));
}

#[test]
fn parse_auto_size() {
    let size = parse_size("auto", SCALE_FACTOR);
    assert_eq!(size, Some(Size::Inner));
}

#[test]
fn parse_calc_size() {
    let size = parse_size("calc(90% - 5% * 123.6)", SCALE_FACTOR);
    assert_eq!(
        size,
        Some(Size::DynamicCalculations(vec![
            DynamicCalculation::Percentage(90.0),
            DynamicCalculation::Sub,
            DynamicCalculation::Percentage(5.0),
            DynamicCalculation::Mul,
            DynamicCalculation::Pixels(123.6)
        ]))
    );
}
