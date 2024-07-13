use freya_node_state::Parse;
use torin::alignment::Alignment;

#[test]
fn parse_normal_alignment() {
    let alignment = Alignment::parse_value("start");
    assert_eq!(alignment, Ok(Alignment::Start));
}

#[test]
fn parse_center_alignment() {
    let alignment = Alignment::parse_value("center");
    assert_eq!(alignment, Ok(Alignment::Center));
}

#[test]
fn parse_end_alignment() {
    let alignment = Alignment::parse_value("end");
    assert_eq!(alignment, Ok(Alignment::End));
}

#[test]
fn parse_space_between_alignment() {
    let alignment = Alignment::parse_value("space-between");
    assert_eq!(alignment, Ok(Alignment::SpaceBetween));
}

#[test]
fn parse_space_around_alignment() {
    let alignment = Alignment::parse_value("space-around");
    assert_eq!(alignment, Ok(Alignment::SpaceAround));
}

#[test]
fn parse_space_evenly_alignment() {
    let alignment = Alignment::parse_value("space-evenly");
    assert_eq!(alignment, Ok(Alignment::SpaceEvenly));
}

#[test]
fn parse_fallback_alignment() {
    let alignment = Alignment::parse_value("Hello, World!");

    assert!(alignment.is_err());
}
