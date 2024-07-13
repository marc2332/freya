use freya_engine::prelude::*;
use freya_node_state::Parse;

#[test]
fn parse_text_decoration() {
    let underline = TextDecoration::parse_value("underline");
    assert_eq!(underline, Ok(TextDecoration::UNDERLINE));

    let overline = TextDecoration::parse_value("overline");
    assert_eq!(overline, Ok(TextDecoration::OVERLINE));

    let line_through = TextDecoration::parse_value("line-through");
    assert_eq!(line_through, Ok(TextDecoration::LINE_THROUGH));

    let invalid_decoration_name = TextDecoration::parse_value("Rust");

    assert!(invalid_decoration_name.is_err());
}

#[test]
fn parse_text_decoration_style() {
    let solid = TextDecorationStyle::parse_value("solid");
    assert_eq!(solid, Ok(TextDecorationStyle::Solid));

    let double = TextDecorationStyle::parse_value("double");
    assert_eq!(double, Ok(TextDecorationStyle::Double));

    let dotted = TextDecorationStyle::parse_value("dotted");
    assert_eq!(dotted, Ok(TextDecorationStyle::Dotted));

    let dashed = TextDecorationStyle::parse_value("dashed");
    assert_eq!(dashed, Ok(TextDecorationStyle::Dashed));

    let wavy = TextDecorationStyle::parse_value("wavy");
    assert_eq!(wavy, Ok(TextDecorationStyle::Wavy));

    let invalid_decoration_style = TextDecorationStyle::parse_value("Rust");

    assert!(invalid_decoration_style.is_err());
}
