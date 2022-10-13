use freya_node_state::{parse_size, CalcType, SizeMode};

#[test]
fn parse_manual_size() {
    let size = parse_size("123");
    assert_eq!(size, Some(SizeMode::Manual(123.0)));
}

#[test]
fn parse_relative_size() {
    let size = parse_size("78.123%");
    assert_eq!(size, Some(SizeMode::Percentage(78.123)));
}

#[test]
fn parse_auto_size() {
    let size = parse_size("auto");
    assert_eq!(size, Some(SizeMode::Auto));
}

#[test]
fn parse_calc_size() {
    let size = parse_size("calc(90% - 5% * 123.6)");
    assert_eq!(
        size,
        Some(SizeMode::Calculation(vec![
            CalcType::Percentage(90.0),
            CalcType::Sub,
            CalcType::Percentage(5.0),
            CalcType::Mul,
            CalcType::Manual(123.6)
        ]))
    );
}
