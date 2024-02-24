use freya_engine::prelude::*;
use freya_node_state::Parse;

#[test]
fn parse_text_decoration() {
    let underline = TextDecoration::parse("underline");
    assert_eq!(underline, Ok(TextDecoration::UNDERLINE));

    let overline = TextDecoration::parse("overline");
    assert_eq!(overline, Ok(TextDecoration::OVERLINE));

    let line_through = TextDecoration::parse("line-through");
    assert_eq!(line_through, Ok(TextDecoration::LINE_THROUGH));

    let fallback = TextDecoration::parse("Rust");
    assert_eq!(fallback, Ok(TextDecoration::NO_DECORATION));
}

#[test]
fn parse_text_decoration_style() {
    let solid = TextDecorationStyle::parse("solid");
    assert_eq!(solid, Ok(TextDecorationStyle::Solid));

    let double = TextDecorationStyle::parse("double");
    assert_eq!(double, Ok(TextDecorationStyle::Double));

    let dotted = TextDecorationStyle::parse("dotted");
    assert_eq!(dotted, Ok(TextDecorationStyle::Dotted));

    let dashed = TextDecorationStyle::parse("dashed");
    assert_eq!(dashed, Ok(TextDecorationStyle::Dashed));

    let wavy = TextDecorationStyle::parse("wavy");
    assert_eq!(wavy, Ok(TextDecorationStyle::Wavy));

    let fallback = TextDecorationStyle::parse("Rust");
    assert_eq!(fallback, Ok(TextDecorationStyle::Solid));
}
