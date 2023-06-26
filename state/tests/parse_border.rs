use freya_node_state::{Border, BorderAlignment, BorderStyle, Fill, Parse};
use skia_safe::Color;

#[test]
fn parse_basic_border() {
    let border = Border::parse("1 solid red");

    assert_eq!(
        border,
        Ok(Border {
            width: 1.0,
            fill: Fill::Color(Color::RED),
            style: BorderStyle::Solid,
            alignment: BorderAlignment::Inner
        })
    );
}

#[test]
fn parse_border_alignments() {
    let inner = BorderAlignment::parse("inner");
    let outer = BorderAlignment::parse("outer");
    let center = BorderAlignment::parse("center");
    let invalid = BorderAlignment::parse("invalid");

    assert_eq!(inner, Ok(BorderAlignment::Inner));
    assert_eq!(outer, Ok(BorderAlignment::Outer));
    assert_eq!(center, Ok(BorderAlignment::Center));
    assert_eq!(invalid, Ok(BorderAlignment::Inner));
}

#[test]
fn parse_border_style() {
    let solid = Border::parse("1 solid red");
    let none = Border::parse("1 rust red");
    let invalid = Border::parse("rust solid red");

    assert_eq!(
        solid,
        Ok(Border {
            width: 1.0,
            fill: Fill::Color(Color::RED),
            style: BorderStyle::Solid,
            alignment: BorderAlignment::default()
        })
    );
    assert_eq!(
        none,
        Ok(Border {
            width: 1.0,
            fill: Fill::Color(Color::RED),
            style: BorderStyle::None,
            alignment: BorderAlignment::default()
        })
    );
    assert!(invalid.is_err());
}