use freya_engine::prelude::*;
use freya_node_state::Parse;

#[test]
fn parse_manual_color() {
    let color = Color::parse_value("red");
    assert_eq!(color, Ok(Color::RED));
}

#[test]
fn parse_rgb_color() {
    let color = Color::parse_value("rgb(91, 123, 57)");
    assert_eq!(color, Ok(Color::from_rgb(91, 123, 57)));
}

#[test]
fn parse_hsl_color() {
    _ = Color::parse_value("hsl(28deg, 80%, 50%, 25%)").unwrap();
}

#[test]
fn parse_argb_color_u8() {
    let color = Color::parse_value("rgb(91, 123, 57, 127)");
    assert_eq!(color, Ok(Color::from_argb(127, 91, 123, 57)));
}

#[test]
fn parse_argb_color_f32() {
    let color = Color::parse_value("rgb(91, 123, 57, 0.5)");
    assert_eq!(color, Ok(Color::from_argb(128, 91, 123, 57)));
}

#[test]
fn parse_hex_color() {
    let color = Color::parse_value("#FFA500");
    assert_eq!(color, Ok(Color::from_rgb(255, 165, 0)));
}

#[test]
fn invalid_colors() {
    let incorrect_name = Color::parse_value("wow(0, 0, 0)");
    let extra_lparen = Color::parse_value("rgb((0, 0, 0)");
    let extra_rparen = Color::parse_value("rgb(0, 0, 0))");
    let missing_lparen = Color::parse_value("rgb0, 0, 0)");
    let missing_rparen = Color::parse_value("rgb(0, 0, 0");
    let missing_commas = Color::parse_value("rgb(0 0 0)");
    let extra_commas = Color::parse_value("rgb(0,, 0, 0)");
    let extra_component = Color::parse_value("rgb(0, 0, 0, 0, 0)");
    let extra_ending_commas = Color::parse_value("rgb(0, 0, 0, 0,)");
    let bad_unit = Color::parse_value("hsl(28in, 0.4, 0.25, 50%)");
    let missing_number_sign = Color::parse_value("FFA500");
    let incorrect_hex_length = Color::parse_value("#FFA0");

    assert!(incorrect_name.is_err());
    assert!(extra_lparen.is_err());
    assert!(extra_rparen.is_err());
    assert!(missing_lparen.is_err());
    assert!(missing_rparen.is_err());
    assert!(missing_commas.is_err());
    assert!(extra_commas.is_err());
    assert!(extra_component.is_err());
    assert!(extra_ending_commas.is_err());
    assert!(bad_unit.is_err());
    assert!(missing_number_sign.is_err());
    assert!(incorrect_hex_length.is_err());
}
