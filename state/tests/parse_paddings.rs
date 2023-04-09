use freya_node_state::parse_padding;

const SCALE_FACTOR: f32 = 1.0;

#[test]
fn parse_all_paddings() {
    let paddings = parse_padding("10", SCALE_FACTOR);
    assert_eq!(paddings, Some((10.0, 10.0, 10.0, 10.0)));
}

#[test]
fn parse_axis_paddings() {
    let paddings = parse_padding("50 10", SCALE_FACTOR);
    assert_eq!(paddings, Some((50.0, 10.0, 50.0, 10.0)));
}

#[test]
fn parse_sides_paddings() {
    let paddings = parse_padding("1 2 3 4", SCALE_FACTOR);
    assert_eq!(paddings, Some((1.0, 2.0, 3.0, 4.0)));
}
