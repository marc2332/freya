use freya_node_state::Parse;
use torin::alignment::Alignment;

#[test]
fn parse_normal_alignment() {
    let alignment = Alignment::parse("start");
    assert_eq!(alignment, Ok(Alignment::Start));
}

#[test]
fn parse_center_alignment() {
    let alignment = Alignment::parse("center");
    assert_eq!(alignment, Ok(Alignment::Center));
}

#[test]
fn parse_end_alignment() {
    let alignment = Alignment::parse("end");
    assert_eq!(alignment, Ok(Alignment::End));
}

#[test]
fn parse_fallback_alignment() {
    let alignment = Alignment::parse("Hello, World!");
    assert_eq!(alignment, Ok(Alignment::Start));
}
