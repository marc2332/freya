use freya_engine::prelude::*;
use freya_node_state::Parse;

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
    _ = Color::parse("hsl(28deg, 80%, 50%, 25%)").unwrap();
}

#[test]
fn parse_argb_color_u8() {
    let color = Color::parse("rgb(91, 123, 57, 127)");
    assert_eq!(color, Ok(Color::from_argb(127, 91, 123, 57)));
}

#[test]
fn parse_argb_color_f32() {
    let color = Color::parse("rgb(91, 123, 57, 0.5)");
    assert_eq!(color, Ok(Color::from_argb(128, 91, 123, 57)));
}

#[test]
fn parse_hex_color() {
    let color = Color::parse("#FFA500");
    assert_eq!(color, Ok(Color::from_rgb(255, 165, 0)));
}

#[test]
fn invalid_colors() {
    let incorrect_name = Color::parse("wow(0, 0, 0)");
    let extra_lparen = Color::parse("rgb((0, 0, 0)");
    let extra_rparen = Color::parse("rgb(0, 0, 0))");
    let missing_lparen = Color::parse("rgb0, 0, 0)");
    let missing_rparen = Color::parse("rgb(0, 0, 0");
    let missing_commas = Color::parse("rgb(0 0 0)");
    let extra_commas = Color::parse("rgb(0,, 0, 0)");
    let extra_component = Color::parse("rgb(0, 0, 0, 0, 0)");
    let extra_ending_commas = Color::parse("rgb(0, 0, 0, 0,)");
    let bad_unit = Color::parse("hsl(28in, 0.4, 0.25, 50%)");
    let missing_number_sign = Color::parse("FFA500");
    let incorrect_hex_length = Color::parse("#FFA0");

    assert_eq!(incorrect_name.is_err(), true);
    assert_eq!(extra_lparen.is_err(), true);
    assert_eq!(extra_rparen.is_err(), true);
    assert_eq!(missing_lparen.is_err(), true);
    assert_eq!(missing_rparen.is_err(), true);
    assert_eq!(missing_commas.is_err(), true);
    assert_eq!(extra_commas.is_err(), true);
    assert_eq!(extra_component.is_err(), true);
    assert_eq!(extra_ending_commas.is_err(), true);
    assert_eq!(bad_unit.is_err(), true);
    assert_eq!(missing_number_sign.is_err(), true);
    assert_eq!(incorrect_hex_length.is_err(), true);
}
