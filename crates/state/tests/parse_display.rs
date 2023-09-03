use freya_node_state::Parse;
use torin::display::DisplayMode;

#[test]
fn parse_normal_display() {
    let display = DisplayMode::parse("normal");
    assert_eq!(display, Ok(DisplayMode::Normal));
}

#[test]
fn parse_center_display() {
    let display = DisplayMode::parse("center");
    assert_eq!(display, Ok(DisplayMode::Center));
}

#[test]
fn parse_fallback_display() {
    let display = DisplayMode::parse("freya!!");
    assert_eq!(display, Ok(DisplayMode::Normal));
}
