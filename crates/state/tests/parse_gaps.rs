use freya_node_state::Parse;
use torin::gaps::Gaps;

#[test]
fn parse_all_gaps() {
    let gaps = Gaps::parse_value("10");
    assert_eq!(gaps, Ok(Gaps::new(10.0, 10.0, 10.0, 10.0)));
}

#[test]
fn parse_axis_gaps() {
    let gaps = Gaps::parse_value("50 10");
    assert_eq!(gaps, Ok(Gaps::new(50.0, 10.0, 50.0, 10.0)));
}

#[test]
fn parse_sides_gaps() {
    let gaps = Gaps::parse_value("1 2 3 4");
    assert_eq!(gaps, Ok(Gaps::new(1.0, 2.0, 3.0, 4.0)));
}

#[test]
fn parse_horizontal_axis_and_vertical_sides() {
    let gaps = Gaps::parse_value("5 50 30");
    assert_eq!(gaps, Ok(Gaps::new(5.0, 50.0, 30.0, 50.0)));
}
