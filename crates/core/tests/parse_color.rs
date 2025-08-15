use freya_core::{
    parsing::Parse,
    values::Color,
};

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
fn parse_hex_transparent_color() {
    let color = Color::parse("#5b7b3980");
    assert_eq!(color, Ok(Color::from_argb(128, 91, 123, 57)));
}

#[test]
fn parse_3_char_hex_color() {
    let color = Color::parse("#F42");
    assert_eq!(color, Ok(Color::from_rgb(255, 68, 34)));
}

#[test]
fn parse_4_char_hex_color() {
    let color = Color::parse("#4F69");
    assert_eq!(color, Ok(Color::from_argb(153, 68, 255, 102)));
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
    let incorrect_hex_length = Color::parse("#FFA0F");

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
