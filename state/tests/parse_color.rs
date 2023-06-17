use freya_node_state::Parse;
use skia_safe::Color;

#[test]
fn parse_manual_color() {
    let color = Color::parse("red", None);
    assert_eq!(color, Ok(Color::RED));
}

#[test]
fn parse_rgb_color() {
    let color = Color::parse("rgb(91, 123, 57", None);
    assert_eq!(color, Ok(Color::from_rgb(91, 123, 57)));
}
