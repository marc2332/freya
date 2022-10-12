use freya_node_state::{parse_display, DisplayMode};

#[test]
fn parse_normal_display() {
    let display = parse_display("normal");
    assert_eq!(display, DisplayMode::Normal);
}

#[test]
fn parse_center_display() {
    let display = parse_display("center");
    assert_eq!(display, DisplayMode::Center);
}

#[test]
fn parse_fallback_display() {
    let display = parse_display("freya!!");
    assert_eq!(display, DisplayMode::Normal);
}
