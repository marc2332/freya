use freya_node_state::Parse;
use torin::padding::Paddings;

#[test]
fn parse_all_paddings() {
    let paddings = Paddings::parse("10");
    assert_eq!(paddings, Ok(Paddings::new(10.0, 10.0, 10.0, 10.0)));
}

#[test]
fn parse_axis_paddings() {
    let paddings = Paddings::parse("50 10");
    assert_eq!(paddings, Ok(Paddings::new(50.0, 10.0, 50.0, 10.0)));
}

#[test]
fn parse_sides_paddings() {
    let paddings = Paddings::parse("1 2 3 4");
    assert_eq!(paddings, Ok(Paddings::new(1.0, 2.0, 3.0, 4.0)));
}
