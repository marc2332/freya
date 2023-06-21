use freya_node_state::Parse;
use skia_safe::Color;

#[test]
fn parse_manual_color() {
    let color = Color::parse("red");
    assert_eq!(color, Ok(Color::RED));
}

#[test]
fn parse_rgb_color() {
    let color = Color::parse("rgb(91, 123, 57)");
    assert_eq!(color, Ok(Color::from_rgb(91, 123, 57)));
}

#[test]
fn parse_hsl_color() {
    let color = Color::parse("hsl(28, 0.8, 0.5, 0.25)").unwrap();
    let color_pct = Color::parse("hsl(28deg, 80%, 50%, 25%)").unwrap();

    assert_eq!(color, color_pct);
}
