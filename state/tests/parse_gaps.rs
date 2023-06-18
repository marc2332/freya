use torin::gap::Gaps;

const SCALE_FACTOR: f32 = 1.0;

#[test]
fn parse_all_gaps() {
    let gaps = Gaps::parse("10", SCALE_FACTOR);
    assert_eq!(gaps, Some(Gap::new(10.0, 10.0, 10.0, 10.0)));
}

#[test]
fn parse_axis_gaps() {
    let gaps = Gaps::parse("50 10", SCALE_FACTOR);
    assert_eq!(gaps, Some(Gap::new(50.0, 10.0, 50.0, 10.0)));
}

#[test]
fn parse_sides_gaps() {
    let gaps = Gaps::parse("1 2 3 4", SCALE_FACTOR);
    assert_eq!(gaps, Some(Gap::new(1.0, 2.0, 3.0, 4.0)));
}
