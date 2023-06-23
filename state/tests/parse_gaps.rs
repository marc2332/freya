use freya_node_state::Parse;
use torin::gaps::Gaps;

#[test]
fn parse_all_gaps() {
    let gaps = Gaps::parse("10");
    assert_eq!(gaps, Ok(Gaps::new(10.0, 10.0, 10.0, 10.0)));
}

#[test]
fn parse_axis_gaps() {
    let gaps = Gaps::parse("50 10");
    assert_eq!(gaps, Ok(Gaps::new(50.0, 10.0, 50.0, 10.0)));
}

#[test]
fn parse_sides_gaps() {
    let gaps = Gaps::parse("1 2 3 4");
    assert_eq!(gaps, Ok(Gaps::new(1.0, 2.0, 3.0, 4.0)));
}
